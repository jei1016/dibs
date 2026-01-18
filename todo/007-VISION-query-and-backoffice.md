# Vision: Query Builder + Backoffice Service

## The Big Picture

dibs evolves from "just migrations" to a full data access layer:

```
┌─────────────────────────────────────────────────────────────┐
│                         dibs                                 │
│                                                              │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────────┐   │
│  │  Migrations  │  │ Query Builder│  │ Backoffice Svc   │   │
│  │              │  │              │  │                  │   │
│  │  #[migration]│  │ type-safe &  │  │ generic CRUD     │   │
│  │  transactions│  │ dynamic      │  │ over roam        │   │
│  └──────────────┘  └──────────────┘  └──────────────────┘   │
│                           │                   │              │
│                           └─────────┬─────────┘              │
│                                     │                        │
│                            ┌────────▼────────┐               │
│                            │     Hooks       │               │
│                            │ before/after    │               │
│                            │ create/update/  │               │
│                            │ delete          │               │
│                            └─────────────────┘               │
└─────────────────────────────────────────────────────────────┘
```

## Two Distinct Use Cases

### 1. Storefront API (customer-facing)

Hand-crafted, optimized, business-logic-heavy:

```rust
// In shops codebase - lovely hand-written roam methods

#[service]
pub trait StorefrontService {
    async fn get_product(&self, slug: String) -> Result<Product, Error>;
    async fn search_products(&self, query: SearchQuery) -> Result<SearchResults, Error>;
    async fn add_to_cart(&self, cart_id: Uuid, item: CartItem) -> Result<Cart, Error>;
    async fn checkout(&self, cart_id: Uuid, payment: PaymentInfo) -> Result<Order, Error>;
}

// Implementation uses dibs query builder underneath:
impl StorefrontService for ShopsStorefront {
    async fn get_product(&self, slug: String) -> Result<Product, Error> {
        self.db
            .query::<Product>()
            .filter(Product::slug.eq(&slug))
            .filter(Product::published.eq(true))
            .with(Product::variants)  // eager load
            .with(Product::images)
            .one()
            .await
    }
}
```

### 2. Backoffice API (admin-facing)

Generic, schema-driven, dynamic:

```rust
// In dibs - one service for everything

#[service]
pub trait BackofficeService {
    async fn schema(&self) -> SchemaInfo;
    async fn list(&self, table: String, query: Query) -> Result<ListResult, Error>;
    async fn get(&self, table: String, pk: Value) -> Result<Option<Row>, Error>;
    async fn create(&self, table: String, data: Row) -> Result<Row, Error>;
    async fn update(&self, table: String, pk: Value, data: Row) -> Result<Row, Error>;
    async fn delete(&self, table: String, pk: Value) -> Result<(), Error>;
}

// Frontend (Svelte) is 100% dynamic:
// - Fetches schema on load
// - Renders tables/forms based on column metadata
// - Builds filter/sort UI from column types
// - No hardcoded entity types
```

## Query Builder: Serves Both

The query builder is foundational - used by both patterns:

```rust
// Type-safe API (for storefront)
db.query::<Order>()
    .filter(Order::status.eq("paid"))
    .filter(Order::created_at.gte(last_week))
    .order_by(Order::created_at.desc())
    .limit(50)
    .all()
    .await

// Dynamic API (for backoffice)
db.query_table("orders")
    .filter("status", Op::Eq, Value::String("paid"))
    .filter("created_at", Op::Gte, Value::Timestamp(last_week))
    .order_by("created_at", Desc)
    .limit(50)
    .all()
    .await
```

Both compile to the same SQL. The type-safe version catches errors at compile time; the dynamic version validates against the schema at runtime.

## Hooks: Business Logic Gateway

All data mutations go through hooks:

```rust
// In shops db crate
dibs::hooks! {
    orders => {
        before_create: |ctx, row| {
            // Validate inventory
            // Check customer exists
            // Apply business rules
        },
        after_create: |ctx, row| {
            // Send confirmation email
            // Update inventory
            // Emit event
        },
    },
    products => {
        before_update: |ctx, pk, changes| {
            // Reindex for search
        },
    },
}
```

The backoffice service respects these hooks - it's not a backdoor, it's the front door with admin credentials.

## Prior Art Research

### [sea-query](https://github.com/SeaQL/sea-query)
Standalone SQL builder, foundation of SeaORM. Key patterns:
```rust
Query::select()
    .column(Char::Character)
    .from(Char::Table)
    .and_where(Expr::col(Char::SizeW).is_in([3, 4]))
    .and_where(Expr::col(Char::Character).like("A%"))
```
- Fluent API building an AST
- `Expr` for expressions, `Cond` for complex AND/OR
- `.build()` returns (SQL, Values) - safe parameterization
- Multi-dialect (MySQL, Postgres, SQLite)

### [Diesel](https://diesel.rs/)
Compile-time type-safe query builder:
```rust
users.filter(name.eq("Sean")).load::<User>(&mut conn)
```
- Expression methods on columns (`.eq()`, `.lt()`, `.like()`)
- `AsExpression` trait lets you pass Rust values or other expressions
- Composable - pull query fragments into functions
- Heavy macro use, compile-time SQL verification

### [PostgREST](https://docs.postgrest.org/en/v12/references/api/tables_views.html)
REST API syntax for Postgres, very relevant for backoffice:
```
GET /people?age=gte.18&student=is.true
GET /people?or=(age.lt.18,age.gt.21)
GET /people?order=age.desc,height.asc
GET /people?select=name,age
```
- `column=operator.value` syntax
- `or=()` and `and=()` for boolean logic, nestable
- Operators: `eq`, `neq`, `lt`, `lte`, `gt`, `gte`, `like`, `ilike`, `is`, `in`, `fts`
- `not.` prefix to negate any operator
- JSON column filtering with `->` and `->>`

### Takeaways

1. **For type-safe (storefront)**: Diesel-style with expression methods on columns
2. **For dynamic (backoffice)**: PostgREST-style filter syntax is battle-tested
3. **Under the hood**: sea-query-style AST building

We can support both APIs that compile to the same SQL.

## Open Questions

1. **Query syntax**: What operators do we need? AND/OR nesting? Subqueries?
2. **Relations**: How do we express eager loading? `with: ["customer", "items.product"]`?
3. **Aggregations**: Do we need COUNT/SUM/GROUP BY in backoffice, or just raw data?
4. **Authorization**: Per-table? Per-row? Column-level hiding?
5. **Audit logging**: Should all backoffice mutations be logged?
