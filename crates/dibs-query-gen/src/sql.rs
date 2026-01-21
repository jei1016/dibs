//! SQL generation from query AST.

use crate::ast::*;
use crate::planner::{PlannerSchema, QueryPlan, QueryPlanner};

/// Generated SQL with parameter placeholders.
#[derive(Debug, Clone)]
pub struct GeneratedSql {
    /// The SQL string with $1, $2, etc. placeholders.
    pub sql: String,
    /// Parameter names in order (maps to $1, $2, etc.).
    pub param_order: Vec<String>,
    /// Query plan (if JOINs are involved).
    pub plan: Option<QueryPlan>,
}

/// Generate SQL for a simple single-table query (no relations).
pub fn generate_simple_sql(query: &Query) -> GeneratedSql {
    let mut sql = String::new();
    let mut param_order = Vec::new();
    let mut param_idx = 1;

    // SELECT
    sql.push_str("SELECT ");
    let columns: Vec<_> = query
        .select
        .iter()
        .filter_map(|f| match f {
            Field::Column { name, .. } => Some(format!("\"{}\"", name)),
            _ => None, // Skip relations/aggregates for simple query
        })
        .collect();

    if columns.is_empty() {
        sql.push('*');
    } else {
        sql.push_str(&columns.join(", "));
    }

    // FROM
    sql.push_str(" FROM \"");
    sql.push_str(&query.from);
    sql.push('"');

    // WHERE
    if !query.filters.is_empty() {
        sql.push_str(" WHERE ");
        let conditions: Vec<_> = query
            .filters
            .iter()
            .map(|f| {
                let (cond, new_idx) = format_filter(f, param_idx, &mut param_order);
                param_idx = new_idx;
                cond
            })
            .collect();
        sql.push_str(&conditions.join(" AND "));
    }

    // ORDER BY
    if !query.order_by.is_empty() {
        sql.push_str(" ORDER BY ");
        let orders: Vec<_> = query
            .order_by
            .iter()
            .map(|o| {
                format!(
                    "\"{}\" {}",
                    o.column,
                    match o.direction {
                        SortDir::Asc => "ASC",
                        SortDir::Desc => "DESC",
                    }
                )
            })
            .collect();
        sql.push_str(&orders.join(", "));
    }

    // LIMIT
    if let Some(limit) = &query.limit {
        sql.push_str(" LIMIT ");
        match limit {
            Expr::Int(n) => sql.push_str(&n.to_string()),
            Expr::Param(name) => {
                param_order.push(name.clone());
                sql.push_str(&format!("${}", param_idx));
                param_idx += 1;
            }
            _ => sql.push_str("20"), // fallback
        }
    }

    // OFFSET
    if let Some(offset) = &query.offset {
        sql.push_str(" OFFSET ");
        match offset {
            Expr::Int(n) => sql.push_str(&n.to_string()),
            Expr::Param(name) => {
                param_order.push(name.clone());
                sql.push_str(&format!("${}", param_idx));
                param_idx += 1;
            }
            _ => sql.push('0'), // fallback
        }
    }

    // Suppress unused warning - param_idx is used during iteration
    let _ = param_idx;

    GeneratedSql {
        sql,
        param_order,
        plan: None,
    }
}

/// Generate SQL for a query with JOINs using the planner.
pub fn generate_sql_with_joins(
    query: &Query,
    schema: &PlannerSchema,
) -> Result<GeneratedSql, crate::planner::PlanError> {
    // Check if query needs the planner (has relations or COUNT fields)
    let needs_planner = query
        .select
        .iter()
        .any(|f| matches!(f, Field::Relation { .. }) || matches!(f, Field::Count { .. }));

    if !needs_planner {
        // Fall back to simple SQL generation
        return Ok(generate_simple_sql(query));
    }

    // Plan the query
    let planner = QueryPlanner::new(schema);
    let plan = planner.plan(query)?;

    let mut sql = String::new();
    let mut param_order = Vec::new();
    let mut param_idx = 1;

    // SELECT with aliased columns
    sql.push_str("SELECT ");
    sql.push_str(&plan.select_sql());

    // FROM with JOINs (including relation filters in ON clauses)
    sql.push_str(" FROM ");
    sql.push_str(&plan.from_sql_with_params(&mut param_order, &mut param_idx));

    // WHERE
    if !query.filters.is_empty() {
        sql.push_str(" WHERE ");
        let conditions: Vec<_> = query
            .filters
            .iter()
            .map(|f| {
                // Prefix column with base table alias
                let mut filter = f.clone();
                filter.column = format!("t0.{}", f.column);
                let (cond, new_idx) = format_filter(&filter, param_idx, &mut param_order);
                param_idx = new_idx;
                cond
            })
            .collect();
        sql.push_str(&conditions.join(" AND "));
    }

    // ORDER BY
    if !query.order_by.is_empty() {
        sql.push_str(" ORDER BY ");
        let orders: Vec<_> = query
            .order_by
            .iter()
            .map(|o| {
                format!(
                    "\"t0\".\"{}\" {}",
                    o.column,
                    match o.direction {
                        SortDir::Asc => "ASC",
                        SortDir::Desc => "DESC",
                    }
                )
            })
            .collect();
        sql.push_str(&orders.join(", "));
    }

    // LIMIT
    if let Some(limit) = &query.limit {
        sql.push_str(" LIMIT ");
        match limit {
            Expr::Int(n) => sql.push_str(&n.to_string()),
            Expr::Param(name) => {
                param_order.push(name.clone());
                sql.push_str(&format!("${}", param_idx));
                param_idx += 1;
            }
            _ => sql.push_str("20"),
        }
    }

    // OFFSET
    if let Some(offset) = &query.offset {
        sql.push_str(" OFFSET ");
        match offset {
            Expr::Int(n) => sql.push_str(&n.to_string()),
            Expr::Param(name) => {
                param_order.push(name.clone());
                sql.push_str(&format!("${}", param_idx));
                param_idx += 1;
            }
            _ => sql.push('0'),
        }
    }

    let _ = param_idx;

    Ok(GeneratedSql {
        sql,
        param_order,
        plan: Some(plan),
    })
}

fn format_filter(
    filter: &Filter,
    mut param_idx: usize,
    param_order: &mut Vec<String>,
) -> (String, usize) {
    // Handle dotted column names (e.g., "t0.column") by quoting each part
    let col = if filter.column.contains('.') {
        filter
            .column
            .split('.')
            .map(|part| format!("\"{}\"", part))
            .collect::<Vec<_>>()
            .join(".")
    } else {
        format!("\"{}\"", filter.column)
    };

    let result = match (&filter.op, &filter.value) {
        (FilterOp::IsNull, _) => format!("{} IS NULL", col),
        (FilterOp::IsNotNull, _) => format!("{} IS NOT NULL", col),
        (FilterOp::Eq, Expr::Null) => format!("{} IS NULL", col),
        (FilterOp::Ne, Expr::Null) => format!("{} IS NOT NULL", col),
        (FilterOp::Eq, Expr::Param(name)) => {
            param_order.push(name.clone());
            let s = format!("{} = ${}", col, param_idx);
            param_idx += 1;
            s
        }
        (FilterOp::Eq, Expr::String(s)) => {
            // Inline string literals directly - escape single quotes
            let escaped = s.replace('\'', "''");
            format!("{} = '{}'", col, escaped)
        }
        (FilterOp::Eq, Expr::Int(n)) => format!("{} = {}", col, n),
        (FilterOp::Eq, Expr::Bool(b)) => format!("{} = {}", col, b),
        (FilterOp::Ne, Expr::Param(name)) => {
            param_order.push(name.clone());
            let s = format!("{} != ${}", col, param_idx);
            param_idx += 1;
            s
        }
        (FilterOp::Lt, Expr::Param(name)) => {
            param_order.push(name.clone());
            let s = format!("{} < ${}", col, param_idx);
            param_idx += 1;
            s
        }
        (FilterOp::Lte, Expr::Param(name)) => {
            param_order.push(name.clone());
            let s = format!("{} <= ${}", col, param_idx);
            param_idx += 1;
            s
        }
        (FilterOp::Gt, Expr::Param(name)) => {
            param_order.push(name.clone());
            let s = format!("{} > ${}", col, param_idx);
            param_idx += 1;
            s
        }
        (FilterOp::Gte, Expr::Param(name)) => {
            param_order.push(name.clone());
            let s = format!("{} >= ${}", col, param_idx);
            param_idx += 1;
            s
        }
        (FilterOp::Like, Expr::Param(name)) => {
            param_order.push(name.clone());
            let s = format!("{} LIKE ${}", col, param_idx);
            param_idx += 1;
            s
        }
        (FilterOp::ILike, Expr::Param(name)) => {
            param_order.push(name.clone());
            let s = format!("{} ILIKE ${}", col, param_idx);
            param_idx += 1;
            s
        }
        (FilterOp::In, Expr::Param(name)) => {
            param_order.push(name.clone());
            let s = format!("{} = ANY(${})", col, param_idx);
            param_idx += 1;
            s
        }
        _ => format!("{} = TRUE", col), // fallback
    };

    (result, param_idx)
}

/// Generate SQL for an INSERT mutation.
pub fn generate_insert_sql(insert: &InsertMutation) -> GeneratedSql {
    let mut sql = String::new();
    let mut param_order = Vec::new();
    let mut param_idx = 1;

    // INSERT INTO
    sql.push_str("INSERT INTO \"");
    sql.push_str(&insert.table);
    sql.push_str("\" (");

    // Column names
    let columns: Vec<_> = insert
        .values
        .iter()
        .map(|(col, _)| format!("\"{}\"", col))
        .collect();
    sql.push_str(&columns.join(", "));
    sql.push_str(") VALUES (");

    // Values
    let values: Vec<_> = insert
        .values
        .iter()
        .map(|(_, expr)| {
            let (val, new_idx) = format_value_expr(expr, param_idx, &mut param_order);
            param_idx = new_idx;
            val
        })
        .collect();
    sql.push_str(&values.join(", "));
    sql.push(')');

    // RETURNING
    if !insert.returning.is_empty() {
        sql.push_str(" RETURNING ");
        let cols: Vec<_> = insert
            .returning
            .iter()
            .map(|c| format!("\"{}\"", c))
            .collect();
        sql.push_str(&cols.join(", "));
    }

    GeneratedSql {
        sql,
        param_order,
        plan: None,
    }
}

/// Generate SQL for an UPSERT mutation (INSERT ... ON CONFLICT ... DO UPDATE).
pub fn generate_upsert_sql(upsert: &UpsertMutation) -> GeneratedSql {
    let mut sql = String::new();
    let mut param_order = Vec::new();
    let mut param_idx = 1;

    // INSERT INTO
    sql.push_str("INSERT INTO \"");
    sql.push_str(&upsert.table);
    sql.push_str("\" (");

    // Column names
    let columns: Vec<_> = upsert
        .values
        .iter()
        .map(|(col, _)| format!("\"{}\"", col))
        .collect();
    sql.push_str(&columns.join(", "));
    sql.push_str(") VALUES (");

    // Values
    let values: Vec<_> = upsert
        .values
        .iter()
        .map(|(_, expr)| {
            let (val, new_idx) = format_value_expr(expr, param_idx, &mut param_order);
            param_idx = new_idx;
            val
        })
        .collect();
    sql.push_str(&values.join(", "));
    sql.push(')');

    // ON CONFLICT
    sql.push_str(" ON CONFLICT (");
    let conflict_cols: Vec<_> = upsert
        .conflict_columns
        .iter()
        .map(|c| format!("\"{}\"", c))
        .collect();
    sql.push_str(&conflict_cols.join(", "));
    sql.push_str(") DO UPDATE SET ");

    // SET clause - exclude conflict columns from update
    let update_sets: Vec<_> = upsert
        .values
        .iter()
        .filter(|(col, _)| !upsert.conflict_columns.contains(col))
        .map(|(col, expr)| {
            let (val, new_idx) = format_value_expr(expr, param_idx, &mut param_order);
            param_idx = new_idx;
            format!("\"{}\" = {}", col, val)
        })
        .collect();
    sql.push_str(&update_sets.join(", "));

    // RETURNING
    if !upsert.returning.is_empty() {
        sql.push_str(" RETURNING ");
        let cols: Vec<_> = upsert
            .returning
            .iter()
            .map(|c| format!("\"{}\"", c))
            .collect();
        sql.push_str(&cols.join(", "));
    }

    GeneratedSql {
        sql,
        param_order,
        plan: None,
    }
}

/// Generate SQL for an UPDATE mutation.
pub fn generate_update_sql(update: &UpdateMutation) -> GeneratedSql {
    let mut sql = String::new();
    let mut param_order = Vec::new();
    let mut param_idx = 1;

    // UPDATE
    sql.push_str("UPDATE \"");
    sql.push_str(&update.table);
    sql.push_str("\" SET ");

    // SET clause
    let sets: Vec<_> = update
        .values
        .iter()
        .map(|(col, expr)| {
            let (val, new_idx) = format_value_expr(expr, param_idx, &mut param_order);
            param_idx = new_idx;
            format!("\"{}\" = {}", col, val)
        })
        .collect();
    sql.push_str(&sets.join(", "));

    // WHERE
    if !update.filters.is_empty() {
        sql.push_str(" WHERE ");
        let conditions: Vec<_> = update
            .filters
            .iter()
            .map(|f| {
                let (cond, new_idx) = format_filter(f, param_idx, &mut param_order);
                param_idx = new_idx;
                cond
            })
            .collect();
        sql.push_str(&conditions.join(" AND "));
    }

    // RETURNING
    if !update.returning.is_empty() {
        sql.push_str(" RETURNING ");
        let cols: Vec<_> = update
            .returning
            .iter()
            .map(|c| format!("\"{}\"", c))
            .collect();
        sql.push_str(&cols.join(", "));
    }

    GeneratedSql {
        sql,
        param_order,
        plan: None,
    }
}

/// Generate SQL for a DELETE mutation.
pub fn generate_delete_sql(delete: &DeleteMutation) -> GeneratedSql {
    let mut sql = String::new();
    let mut param_order = Vec::new();
    let mut param_idx = 1;

    // DELETE FROM
    sql.push_str("DELETE FROM \"");
    sql.push_str(&delete.table);
    sql.push('"');

    // WHERE
    if !delete.filters.is_empty() {
        sql.push_str(" WHERE ");
        let conditions: Vec<_> = delete
            .filters
            .iter()
            .map(|f| {
                let (cond, new_idx) = format_filter(f, param_idx, &mut param_order);
                param_idx = new_idx;
                cond
            })
            .collect();
        sql.push_str(&conditions.join(" AND "));
    }

    // RETURNING
    if !delete.returning.is_empty() {
        sql.push_str(" RETURNING ");
        let cols: Vec<_> = delete
            .returning
            .iter()
            .map(|c| format!("\"{}\"", c))
            .collect();
        sql.push_str(&cols.join(", "));
    }

    GeneratedSql {
        sql,
        param_order,
        plan: None,
    }
}

/// Format a value expression for INSERT/UPDATE.
fn format_value_expr(
    expr: &ValueExpr,
    mut param_idx: usize,
    param_order: &mut Vec<String>,
) -> (String, usize) {
    let result = match expr {
        ValueExpr::Param(name) => {
            param_order.push(name.clone());
            let s = format!("${}", param_idx);
            param_idx += 1;
            s
        }
        ValueExpr::String(s) => {
            let escaped = s.replace('\'', "''");
            format!("'{}'", escaped)
        }
        ValueExpr::Int(n) => n.to_string(),
        ValueExpr::Bool(b) => b.to_string(),
        ValueExpr::Null => "NULL".to_string(),
        ValueExpr::Now => "NOW()".to_string(),
        ValueExpr::Default => "DEFAULT".to_string(),
    };
    (result, param_idx)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::parse_query_file;

    #[test]
    fn test_simple_select() {
        let source = r#"
AllProducts @query{
  from product
  select{ id, handle, status }
}
"#;
        let file = parse_query_file(source).unwrap();
        let sql = generate_simple_sql(&file.queries[0]);

        // Column order is non-deterministic due to HashMap iteration
        assert!(sql.sql.starts_with("SELECT "));
        assert!(sql.sql.contains(r#""id""#));
        assert!(sql.sql.contains(r#""handle""#));
        assert!(sql.sql.contains(r#""status""#));
        assert!(sql.sql.ends_with(r#" FROM "product""#));
        assert!(sql.param_order.is_empty());
    }

    #[test]
    fn test_select_with_where() {
        let source = r#"
ActiveProducts @query{
  from product
  where{ status "published", active true }
  select{ id, handle }
}
"#;
        let file = parse_query_file(source).unwrap();
        let sql = generate_simple_sql(&file.queries[0]);

        // Check structure without depending on order
        assert!(sql.sql.contains("SELECT "));
        assert!(sql.sql.contains(r#""id""#));
        assert!(sql.sql.contains(r#""handle""#));
        assert!(sql.sql.contains("WHERE"));
        assert!(sql.sql.contains(r#""status" = 'published'"#));
        assert!(sql.sql.contains(r#""active" = true"#));
        assert!(sql.param_order.is_empty()); // No params for literals
    }

    #[test]
    fn test_select_with_params() {
        let source = r#"
ProductByHandle @query{
  params{ handle @string }
  from product
  where{ handle $handle }
  select{ id, handle }
}
"#;
        let file = parse_query_file(source).unwrap();
        let sql = generate_simple_sql(&file.queries[0]);

        assert!(sql.sql.contains(r#""handle" = $1"#));
        assert_eq!(sql.param_order, vec!["handle"]);
    }

    #[test]
    fn test_select_with_order_and_limit() {
        let source = r#"
RecentProducts @query{
  from product
  order_by{ created_at desc }
  limit 20
  select{ id, handle }
}
"#;
        let file = parse_query_file(source).unwrap();
        let sql = generate_simple_sql(&file.queries[0]);

        assert!(sql.sql.contains(r#"ORDER BY "created_at" DESC"#));
        assert!(sql.sql.contains("LIMIT 20"));
    }

    #[test]
    fn test_null_filter() {
        let source = r#"
ActiveProducts @query{
  from product
  where{ deleted_at @null }
  select{ id }
}
"#;
        let file = parse_query_file(source).unwrap();
        let sql = generate_simple_sql(&file.queries[0]);

        assert!(sql.sql.contains(r#""deleted_at" IS NULL"#));
    }

    #[test]
    fn test_ilike_filter() {
        let source = r#"
SearchProducts @query{
  params{ q @string }
  from product
  where{ handle @ilike($q) }
  select{ id, handle }
}
"#;
        let file = parse_query_file(source).unwrap();
        let sql = generate_simple_sql(&file.queries[0]);

        assert!(sql.sql.contains(r#""handle" ILIKE $1"#));
        assert_eq!(sql.param_order, vec!["q"]);
    }

    #[test]
    fn test_pagination_literals() {
        let source = r#"
PaginatedProducts @query{
  from product
  order_by{ created_at desc }
  limit 20
  offset 40
  select{ id, handle }
}
"#;
        let file = parse_query_file(source).unwrap();
        let sql = generate_simple_sql(&file.queries[0]);

        assert!(sql.sql.contains("LIMIT 20"));
        assert!(sql.sql.contains("OFFSET 40"));
        assert!(sql.param_order.is_empty());
    }

    #[test]
    fn test_pagination_params() {
        let source = r#"
PaginatedProducts @query{
  params{ page_size @int, page_offset @int }
  from product
  order_by{ created_at desc }
  limit $page_size
  offset $page_offset
  select{ id, handle }
}
"#;
        let file = parse_query_file(source).unwrap();
        let sql = generate_simple_sql(&file.queries[0]);

        assert!(sql.sql.contains("LIMIT $1"));
        assert!(sql.sql.contains("OFFSET $2"));
        assert_eq!(sql.param_order, vec!["page_size", "page_offset"]);
    }

    #[test]
    fn test_sql_with_joins() {
        use crate::planner::{PlannerForeignKey, PlannerSchema, PlannerTable};

        let source = r#"
ProductWithTranslation @query{
  params{ handle @string }
  from product
  where{ handle $handle }
  select{
    id
    handle
    translation @rel{
      from product_translation
      first true
      select{ title, description }
    }
  }
}
"#;
        let file = parse_query_file(source).unwrap();

        // Build test schema
        let mut schema = PlannerSchema::default();
        schema.tables.insert(
            "product".to_string(),
            PlannerTable {
                name: "product".to_string(),
                columns: vec!["id".to_string(), "handle".to_string()],
                foreign_keys: vec![],
            },
        );
        schema.tables.insert(
            "product_translation".to_string(),
            PlannerTable {
                name: "product_translation".to_string(),
                columns: vec![
                    "id".to_string(),
                    "product_id".to_string(),
                    "title".to_string(),
                    "description".to_string(),
                ],
                foreign_keys: vec![PlannerForeignKey {
                    columns: vec!["product_id".to_string()],
                    references_table: "product".to_string(),
                    references_columns: vec!["id".to_string()],
                }],
            },
        );

        let sql = generate_sql_with_joins(&file.queries[0], &schema).unwrap();

        // Check SELECT
        assert!(sql.sql.contains("\"t0\".\"id\""));
        assert!(sql.sql.contains("\"t0\".\"handle\""));
        assert!(sql.sql.contains("\"t1\".\"title\""));
        assert!(sql.sql.contains("\"t1\".\"description\""));

        // Check FROM with JOIN
        assert!(sql.sql.contains("FROM \"product\" AS \"t0\""));
        assert!(
            sql.sql
                .contains("LEFT JOIN \"product_translation\" AS \"t1\"")
        );
        assert!(sql.sql.contains("ON t0.id = t1.product_id"));

        // Check WHERE
        assert!(sql.sql.contains("\"t0\".\"handle\" = $1"));

        // Check param order
        assert_eq!(sql.param_order, vec!["handle"]);

        // Check plan exists
        assert!(sql.plan.is_some());
    }

    #[test]
    fn test_sql_with_relation_where_literal() {
        use crate::planner::{PlannerForeignKey, PlannerSchema, PlannerTable};

        let source = r#"
ProductWithEnglishTranslation @query{
  from product
  select{
    id
    translation @rel{
      from product_translation
      where{ locale "en" }
      first true
      select{ title }
    }
  }
}
"#;
        let file = parse_query_file(source).unwrap();

        let mut schema = PlannerSchema::default();
        schema.tables.insert(
            "product".to_string(),
            PlannerTable {
                name: "product".to_string(),
                columns: vec!["id".to_string()],
                foreign_keys: vec![],
            },
        );
        schema.tables.insert(
            "product_translation".to_string(),
            PlannerTable {
                name: "product_translation".to_string(),
                columns: vec![
                    "id".to_string(),
                    "product_id".to_string(),
                    "locale".to_string(),
                    "title".to_string(),
                ],
                foreign_keys: vec![PlannerForeignKey {
                    columns: vec!["product_id".to_string()],
                    references_table: "product".to_string(),
                    references_columns: vec!["id".to_string()],
                }],
            },
        );

        let sql = generate_sql_with_joins(&file.queries[0], &schema).unwrap();

        // Check that relation filter is in the ON clause
        assert!(
            sql.sql
                .contains("ON t0.id = t1.product_id AND \"t1\".\"locale\" = 'en'"),
            "Expected relation filter in ON clause, got: {}",
            sql.sql
        );
    }

    #[test]
    fn test_sql_with_relation_where_param() {
        use crate::planner::{PlannerForeignKey, PlannerSchema, PlannerTable};

        let source = r#"
ProductWithTranslation @query{
  params{ locale @string }
  from product
  select{
    id
    translation @rel{
      from product_translation
      where{ locale $locale }
      first true
      select{ title }
    }
  }
}
"#;
        let file = parse_query_file(source).unwrap();

        let mut schema = PlannerSchema::default();
        schema.tables.insert(
            "product".to_string(),
            PlannerTable {
                name: "product".to_string(),
                columns: vec!["id".to_string()],
                foreign_keys: vec![],
            },
        );
        schema.tables.insert(
            "product_translation".to_string(),
            PlannerTable {
                name: "product_translation".to_string(),
                columns: vec![
                    "id".to_string(),
                    "product_id".to_string(),
                    "locale".to_string(),
                    "title".to_string(),
                ],
                foreign_keys: vec![PlannerForeignKey {
                    columns: vec!["product_id".to_string()],
                    references_table: "product".to_string(),
                    references_columns: vec!["id".to_string()],
                }],
            },
        );

        let sql = generate_sql_with_joins(&file.queries[0], &schema).unwrap();

        // Check that relation filter is in the ON clause with param placeholder
        assert!(
            sql.sql
                .contains("ON t0.id = t1.product_id AND \"t1\".\"locale\" = $1"),
            "Expected relation filter with param in ON clause, got: {}",
            sql.sql
        );

        // Check param order includes the relation param
        assert_eq!(sql.param_order, vec!["locale"]);
    }

    #[test]
    fn test_sql_with_relation_where_and_base_where() {
        use crate::planner::{PlannerForeignKey, PlannerSchema, PlannerTable};

        let source = r#"
ProductWithTranslation @query{
  params{ handle @string, locale @string }
  from product
  where{ handle $handle }
  select{
    id
    translation @rel{
      from product_translation
      where{ locale $locale }
      first true
      select{ title }
    }
  }
}
"#;
        let file = parse_query_file(source).unwrap();

        let mut schema = PlannerSchema::default();
        schema.tables.insert(
            "product".to_string(),
            PlannerTable {
                name: "product".to_string(),
                columns: vec!["id".to_string(), "handle".to_string()],
                foreign_keys: vec![],
            },
        );
        schema.tables.insert(
            "product_translation".to_string(),
            PlannerTable {
                name: "product_translation".to_string(),
                columns: vec![
                    "id".to_string(),
                    "product_id".to_string(),
                    "locale".to_string(),
                    "title".to_string(),
                ],
                foreign_keys: vec![PlannerForeignKey {
                    columns: vec!["product_id".to_string()],
                    references_table: "product".to_string(),
                    references_columns: vec!["id".to_string()],
                }],
            },
        );

        let sql = generate_sql_with_joins(&file.queries[0], &schema).unwrap();

        // Relation filter should be $1 (comes first in FROM clause)
        assert!(
            sql.sql.contains("\"t1\".\"locale\" = $1"),
            "Expected relation filter as $1, got: {}",
            sql.sql
        );

        // Base WHERE filter should be $2 (comes after FROM clause)
        assert!(
            sql.sql.contains("\"t0\".\"handle\" = $2"),
            "Expected base filter as $2, got: {}",
            sql.sql
        );

        // Check param order: relation params first, then base WHERE params
        assert_eq!(sql.param_order, vec!["locale", "handle"]);
    }

    #[test]
    fn test_insert_sql() {
        let source = r#"
CreateUser @insert{
  params{
    name @string
    email @string
  }
  into users
  values{
    name $name
    email $email
    created_at @now
  }
  returning{ id, name, email, created_at }
}
"#;
        let file = parse_query_file(source).unwrap();
        let sql = generate_insert_sql(&file.inserts[0]);

        assert!(sql.sql.contains("INSERT INTO \"users\""));
        assert!(sql.sql.contains("\"name\""));
        assert!(sql.sql.contains("\"email\""));
        assert!(sql.sql.contains("\"created_at\""));
        assert!(sql.sql.contains("NOW()"));
        assert!(sql.sql.contains("RETURNING"));
        assert_eq!(sql.param_order.len(), 2);
    }

    #[test]
    fn test_upsert_sql() {
        let source = r#"
UpsertProduct @upsert{
  params{
    id @uuid
    name @string
    price @decimal
  }
  into products
  conflict{ id }
  values{
    id $id
    name $name
    price $price
    updated_at @now
  }
  returning{ id, name, price, updated_at }
}
"#;
        let file = parse_query_file(source).unwrap();
        let sql = generate_upsert_sql(&file.upserts[0]);

        assert!(sql.sql.contains("INSERT INTO \"products\""));
        assert!(sql.sql.contains("ON CONFLICT (\"id\")"));
        assert!(sql.sql.contains("DO UPDATE SET"));
        // id should NOT be in the update set
        assert!(!sql.sql.contains("\"id\" = $"));
        assert!(sql.sql.contains("\"name\" ="));
        assert!(sql.sql.contains("\"price\" ="));
        assert!(sql.sql.contains("RETURNING"));
    }

    #[test]
    fn test_update_sql() {
        let source = r#"
UpdateUserEmail @update{
  params{
    id @uuid
    email @string
  }
  table users
  set{
    email $email
    updated_at @now
  }
  where{ id $id }
  returning{ id, email, updated_at }
}
"#;
        let file = parse_query_file(source).unwrap();
        let sql = generate_update_sql(&file.updates[0]);

        assert!(sql.sql.contains("UPDATE \"users\" SET"));
        assert!(sql.sql.contains("\"email\" = $1"));
        assert!(sql.sql.contains("\"updated_at\" = NOW()"));
        assert!(sql.sql.contains("WHERE \"id\" = $2"));
        assert!(sql.sql.contains("RETURNING"));
        assert_eq!(sql.param_order, vec!["email", "id"]);
    }

    #[test]
    fn test_delete_sql() {
        let source = r#"
DeleteUser @delete{
  params{
    id @uuid
  }
  from users
  where{ id $id }
  returning{ id }
}
"#;
        let file = parse_query_file(source).unwrap();
        let sql = generate_delete_sql(&file.deletes[0]);

        assert!(sql.sql.contains("DELETE FROM \"users\""));
        assert!(sql.sql.contains("WHERE \"id\" = $1"));
        assert!(sql.sql.contains("RETURNING \"id\""));
        assert_eq!(sql.param_order, vec!["id"]);
    }
}
