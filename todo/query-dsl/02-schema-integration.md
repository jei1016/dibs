# Phase 2: Schema Integration

## Goal

The query DSL needs to know the database schema to:
1. Validate table and column references
2. Infer JOIN conditions from FK relationships
3. Know Rust types for codegen
4. Provide LSP completions

## Crate Structure

```
my-app-db/          # Schema crate (already exists in dibs)
  src/lib.rs        # Product, ProductVariant, etc. with #[derive(Facet)]
  Cargo.toml

my-app-queries/     # Queries crate (new)
  queries/
    storefront.styx
    admin.styx
  build.rs          # Reads schema from my-app-db, generates Rust
  src/lib.rs        # include!(concat!(env!("OUT_DIR"), "/queries.rs"))
  Cargo.toml
    [build-dependencies]
    my-app-db = { path = "../my-app-db" }
    dibs-query-gen = "..."

my-app/             # Application
  Cargo.toml
    [dependencies]
    my-app-db = { path = "../my-app-db" }
    my-app-queries = { path = "../my-app-queries" }
```

## Schema Access in build.rs

The key insight: `my-app-db` is a build dependency, so `build.rs` can import and reflect on its types.

```rust
// my-app-queries/build.rs
use my_app_db::{Product, ProductVariant, ProductTranslation, VariantPrice};
use facet::Facet;
use dibs_query_gen::{QueryGenerator, SchemaInfo};

fn main() {
    // Collect schema info via facet reflection
    let schema = SchemaInfo::new()
        .add_table::<Product>()
        .add_table::<ProductVariant>()
        .add_table::<ProductTranslation>()
        .add_table::<VariantPrice>();

    // Parse .styx query files
    let queries = dibs_query_gen::parse_queries("queries/");

    // Validate queries against schema
    // - Check table names exist
    // - Check column names exist
    // - Resolve FK relationships for @rel
    // - Check param types match column types
    let validated = schema.validate_queries(&queries)?;

    // Generate Rust code
    let code = QueryGenerator::new(&schema, &validated).generate();

    // Write to OUT_DIR
    let out_dir = std::env::var("OUT_DIR").unwrap();
    std::fs::write(format!("{}/queries.rs", out_dir), code).unwrap();
}
```

## SchemaInfo via Facet Reflection

Facet reflection gives us:
- Table name (from `#[facet(dibs::table = "...")]`)
- Column names and types (from struct fields)
- Primary key (from `#[facet(dibs::pk)]`)
- Foreign keys (from `#[facet(dibs::fk = "table.column")]`)
- Nullability (from `Option<T>`)

```rust
pub struct SchemaInfo {
    tables: HashMap<String, TableInfo>,
}

pub struct TableInfo {
    name: String,           // "product"
    rust_type: String,      // "Product"
    columns: Vec<ColumnInfo>,
    primary_key: String,
    foreign_keys: Vec<ForeignKey>,
}

pub struct ColumnInfo {
    name: String,           // "handle"
    rust_type: String,      // "String"
    sql_type: String,       // "TEXT"
    nullable: bool,
}

pub struct ForeignKey {
    column: String,         // "product_id"
    references_table: String, // "product"
    references_column: String, // "id"
}
```

## Relation Resolution

When we see:
```styx
translation @rel{
  where{ locale $locale }
  select{ title, description }
}
```

We need to:
1. We're in `product` context
2. Look for a table with FK pointing to `product`
3. Find `product_translation` has `product_id -> product.id`
4. The relation name `translation` → maps to `product_translation` (strip prefix)

Resolution rules:
- `translation` → `product_translation` (table has FK to current table)
- `variants` → `product_variant` (table has FK to current table)
- `prices` → `variant_price` (when in variant context)

## Explicit vs Inferred Relations

For simple cases, infer from FK:
```styx
translation @rel{ ... }  // infers product_translation
```

For ambiguous cases, allow explicit table:
```styx
default_variant @rel{
  from product_variant  // explicit
  ...
}
```

## Type Mapping

Query params and result fields need Rust types:

| Styx Type | Rust Type |
|-----------|-----------|
| `@string` | `String` |
| `@int` | `i64` |
| `@bool` | `bool` |
| `@uuid` | `uuid::Uuid` |
| `@decimal` | `rust_decimal::Decimal` |
| `@timestamp` | `jiff::Timestamp` |
| `@optional(@T)` | `Option<T>` |

Column types inferred from schema:
```styx
select{ id handle status }
```
→ types come from `Product` struct fields

## Validation Errors

The build.rs should produce helpful errors:

```
error[Q001]: unknown table 'products'
  --> queries/storefront.styx:14:8
   |
14 |   from products
   |        ^^^^^^^^ table not found
   |
   = help: did you mean 'product'?
   = note: available tables: product, product_variant, variant_price
```

```
error[Q002]: unknown column 'titel' in table 'product_translation'
  --> queries/storefront.styx:27:14
   |
27 |       select{ titel, description }
   |               ^^^^^ column not found
   |
   = help: did you mean 'title'?
```

```
error[Q003]: no relation 'pricing' found from 'product'
  --> queries/storefront.styx:31:5
   |
31 |     pricing @rel{ ... }
   |     ^^^^^^^ no FK relationship found
   |
   = note: tables with FK to 'product': product_variant, product_translation, product_source
```

## Caching / Incremental

build.rs reruns when:
- Any `.styx` file changes
- Schema types change (my-app-db recompiles)

Use `println!("cargo:rerun-if-changed=queries/")` etc.

## Alternative: Proc Macro

Could also do this as a proc macro:
```rust
dibs_queries!("queries/storefront.styx");
```

But build.rs is preferred because:
- Proc macros can't be cached well
- build.rs can produce better error messages
- Easier to debug (can print intermediate output)

## Deliverables

1. `dibs-query-gen` crate with:
   - `SchemaInfo` type that collects facet metadata
   - Query file parser (uses styx-parse)
   - Validator that checks queries against schema
   - Error formatting with spans

2. Example `my-app-queries` crate demonstrating the pattern
