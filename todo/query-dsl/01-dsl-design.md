# Phase 1: Query DSL Design

## Decision: Styx-Based Syntax

Rather than inventing a new language, we build on **styx** - a human-friendly configuration language with:
- Minimal syntax (whitespace-separated, no colons)
- Tag-based type system (`@string`, `@query`, etc.)
- Schema validation built-in
- Heredocs with syntax hints
- Existing parser, already has facet integration

## Syntax

See `prototype-queries.styx` for working examples. Key patterns:

### Query Definition

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
    // ... fields
  }
}
```

### Parameter References

Parameters are referenced with `$name`:
```styx
where{ locale $locale }
```

Styx parses `$locale` as a scalar - we detect the `$` prefix at interpretation time.

### Relations (JOINs)

Relations use `@rel{...}` and dibs infers the JOIN from FK metadata:

```styx
translation @rel{
  where{ locale $locale }
  first true
  select{ title, description }
}
```

This translates to a JOIN on `product_translation.product_id = product.id`.

### Operators

SQL operators as tags:
- `@null` - IS NULL check
- `@ilike($q)` - case-insensitive LIKE
- `@count(table)` - COUNT aggregate
- More to be defined: `@gt`, `@lt`, `@in`, `@between`, etc.

### Raw SQL Escape Hatch

For complex queries, use heredocs:

```styx
TrendingProducts @query{
  params{ locale @string, days @int }

  sql <<SQL,sql
    SELECT p.id, COUNT(*) as order_count
    FROM product p
    JOIN order_line_item oli ON ...
    WHERE ...
  SQL

  returns{
    id @int
    order_count @int
  }
}
```

## What Styx Gives Us

1. **Parser** - styx-parse already handles the syntax
2. **Tree structure** - styx-tree gives us a typed AST
3. **Schema validation** - we can write a schema for valid queries
4. **LSP foundation** - styx already has LSP support we can extend
5. **Facet integration** - existing deserialize infrastructure

## What We Add

1. **Query-specific tags** - `@query`, `@rel`, `@null`, `@ilike`, etc.
2. **Schema for queries** - validates query structure
3. **Parameter detection** - recognize `$name` references
4. **SQL generation** - compile styx tree to SQL
5. **Rust codegen** - generate structs and query functions

## Open Questions

1. **Conditional filters** - How to express "include this filter only if param is Some"?
   - Maybe: `status @if-set($status)`
   - Or: handled at runtime, not in DSL

2. **Aliases** - How to rename fields in the result?
   - Maybe: `total_orders @as(order_count)` or attributes

3. **OR conditions** - Need a way to express disjunction
   - Maybe: `@or{ condition1, condition2 }`

## File Structure

```
my-app-db/
  queries/
    storefront.styx    # ProductListing, ProductByHandle, etc.
    admin.styx         # AdminProductList, etc.
```

## Next Steps

1. ✅ Prototype syntax in styx (done - prototype-queries.styx parses)
2. Write schema for query DSL
3. Build interpreter that reads styx tree + dibs schema
4. Generate SQL and Rust code
