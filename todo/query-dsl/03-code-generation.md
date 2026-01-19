# Phase 3: Code Generation

## Goal

From a validated query, generate:
1. SQL string (parameterized)
2. Rust result struct
3. Async function to execute the query

## Example Input

```styx
ProductListing @query{
  params{
    locale @string
    currency @string
  }

  from product
  where{ status "published", active true }
  order_by{ created_at desc }
  limit 20

  select{
    id
    handle

    translation @rel{
      where{ locale $locale }
      first true
      select{ title, description }
    }

    variants @rel{
      where{ deleted_at @null }
      order_by{ sort_order asc }
      select{
        id
        sku
        title

        price @rel{
          where{ currency_code $currency }
          first true
          select{ amount }
        }
      }
    }
  }
}
```

## Generated Output

### Result Structs

```rust
/// Result type for ProductListing query
#[derive(Debug, Clone)]
pub struct ProductListingRow {
    pub id: i64,
    pub handle: String,
    pub translation: Option<ProductListingTranslation>,
    pub variants: Vec<ProductListingVariant>,
}

#[derive(Debug, Clone)]
pub struct ProductListingTranslation {
    pub title: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ProductListingVariant {
    pub id: i64,
    pub sku: String,
    pub title: String,
    pub price: Option<ProductListingVariantPrice>,
}

#[derive(Debug, Clone)]
pub struct ProductListingVariantPrice {
    pub amount: rust_decimal::Decimal,
}
```

### Query Function

```rust
/// Execute ProductListing query
pub async fn product_listing(
    db: &PgPool,
    locale: &str,
    currency: &str,
) -> Result<Vec<ProductListingRow>, dibs::Error> {
    // Strategy depends on query shape...
}
```

## SQL Generation Strategies

### Strategy 1: Single Query with JOINs (Flat)

For queries where relations are `first: true` (single related row):

```sql
SELECT
    p.id,
    p.handle,
    pt.title AS "translation.title",
    pt.description AS "translation.description"
FROM product p
LEFT JOIN product_translation pt
    ON pt.product_id = p.id AND pt.locale = $1
WHERE p.status = 'published' AND p.active = true
ORDER BY p.created_at DESC
LIMIT 20
```

Then unflatten in Rust.

### Strategy 2: Multiple Queries (N+1 avoided via batching)

For queries with `Vec<T>` relations:

```rust
// 1. Fetch products
let products = sqlx::query!(r#"
    SELECT id, handle FROM product
    WHERE status = 'published' AND active = true
    ORDER BY created_at DESC LIMIT 20
"#).fetch_all(db).await?;

let product_ids: Vec<i64> = products.iter().map(|p| p.id).collect();

// 2. Batch fetch translations
let translations = sqlx::query!(r#"
    SELECT product_id, title, description
    FROM product_translation
    WHERE product_id = ANY($1) AND locale = $2
"#, &product_ids, locale).fetch_all(db).await?;

// 3. Batch fetch variants
let variants = sqlx::query!(r#"
    SELECT id, product_id, sku, title
    FROM product_variant
    WHERE product_id = ANY($1) AND deleted_at IS NULL
    ORDER BY sort_order ASC
"#, &product_ids).fetch_all(db).await?;

let variant_ids: Vec<i64> = variants.iter().map(|v| v.id).collect();

// 4. Batch fetch prices
let prices = sqlx::query!(r#"
    SELECT variant_id, amount
    FROM variant_price
    WHERE variant_id = ANY($1) AND currency_code = $2
"#, &variant_ids, currency).fetch_all(db).await?;

// 5. Assemble results
// ... group by foreign keys and build nested structs
```

### Strategy 3: Lateral Joins (Postgres-specific)

For ordered/limited nested relations:

```sql
SELECT
    p.id,
    p.handle,
    v.variants
FROM product p
LEFT JOIN LATERAL (
    SELECT jsonb_agg(
        jsonb_build_object(
            'id', pv.id,
            'sku', pv.sku,
            'title', pv.title
        ) ORDER BY pv.sort_order
    ) AS variants
    FROM product_variant pv
    WHERE pv.product_id = p.id AND pv.deleted_at IS NULL
) v ON true
WHERE p.status = 'published' AND p.active = true
ORDER BY p.created_at DESC
LIMIT 20
```

## Choosing a Strategy

The code generator picks strategy based on query shape:

| Query Shape | Strategy |
|-------------|----------|
| No relations | Single query |
| Only `first: true` relations | JOIN with flattening |
| `Vec<T>` relations, no ordering | Batch queries |
| `Vec<T>` with ordering/limit | Lateral joins or batch |

## Row Mapping

Use facet for deserialization:

```rust
use facet::Facet;
use facet_sqlx::FromRow;

#[derive(Debug, Clone, Facet)]
#[facet(derive(FromRow))]  // or similar
pub struct ProductListingRow {
    pub id: i64,
    pub handle: String,
    // ...
}
```

Or generate manual mapping code if facet-sqlx isn't ready.

## Raw SQL Queries

For queries with `sql` heredoc:

```styx
TrendingProducts @query{
  params{ locale @string, days @int }

  sql <<SQL,sql
    SELECT p.id, p.handle, pt.title, COUNT(*) as total_orders
    FROM product p
    JOIN product_translation pt ON ...
    ...
  SQL

  returns{
    id @int
    handle @string
    title @string
    total_orders @int
  }
}
```

Generates:

```rust
#[derive(Debug, Clone)]
pub struct TrendingProductsRow {
    pub id: i64,
    pub handle: String,
    pub title: String,
    pub total_orders: i64,
}

pub async fn trending_products(
    db: &PgPool,
    locale: &str,
    days: i32,
) -> Result<Vec<TrendingProductsRow>, dibs::Error> {
    let rows = sqlx::query_as!(
        TrendingProductsRow,
        r#"
        SELECT p.id, p.handle, pt.title, COUNT(*) as total_orders
        FROM product p
        JOIN product_translation pt ON ...
        ...
        "#,
        locale,
        days
    )
    .fetch_all(db)
    .await?;

    Ok(rows)
}
```

## Parameter Handling

Parameters in `.styx`:
```styx
params{
  locale @string
  currency @string
  limit @optional(@int)
}
```

Become function arguments:
```rust
pub async fn product_listing(
    db: &PgPool,
    locale: &str,           // @string → &str
    currency: &str,         // @string → &str
    limit: Option<i32>,     // @optional(@int) → Option<i32>
) -> Result<Vec<ProductListingRow>, dibs::Error>
```

## Generated File Structure

```rust
// Generated by dibs-query-gen - do not edit

// === storefront.styx ===

pub mod storefront {
    use super::*;

    // ProductListing
    #[derive(Debug, Clone)]
    pub struct ProductListingRow { ... }

    pub async fn product_listing(...) -> Result<Vec<ProductListingRow>, Error> { ... }

    // ProductByHandle
    #[derive(Debug, Clone)]
    pub struct ProductByHandleRow { ... }

    pub async fn product_by_handle(...) -> Result<Option<ProductByHandleRow>, Error> { ... }
}

// === admin.styx ===

pub mod admin {
    use super::*;

    // AdminProductList
    ...
}
```

## Deliverables

1. SQL generator that handles:
   - Simple selects
   - JOINs for `first: true` relations
   - Batch queries for `Vec<T>` relations
   - Parameter substitution

2. Rust codegen that produces:
   - Result structs (nested)
   - Query functions
   - Proper error handling

3. Integration with sqlx or tokio-postgres for execution
