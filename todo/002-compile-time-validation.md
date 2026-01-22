# 002: Query Linting & Validation

**Priority:** Medium

## Problem

Queries can have subtle issues that would fail at runtime or produce unexpected results. We should catch these at compile time (LSP squiggles + build errors/warnings).

## Validations to Implement

### Type Safety

- [ ] **FK type mismatch**: Foreign key column type doesn't match target PK type
  ```
  error: type mismatch in relation 'variants'
    = product_id is INTEGER but product.id is BIGINT
  ```

- [ ] **Param type vs column type**: Parameter declared with wrong type for column
  ```
  error: type mismatch for param 'handle'
    = param is @int but column 'handle' is TEXT
  ```

- [ ] **Literal type vs column type**: Literal value doesn't match column type
  ```
  error: type mismatch in where clause
    = 'active' is BOOLEAN but got string "yes"
  ```

### SQL Best Practices

- [ ] **OFFSET without LIMIT**: Usually a mistake
  ```
  warning: 'offset' without 'limit' - did you forget limit?
  ```

- [ ] **LIMIT without ORDER BY**: Non-deterministic results
  ```
  warning: 'limit' without 'order-by' returns arbitrary rows
  ```

- [ ] **Large LIMIT values**: Potential performance issue
  ```
  warning: limit of 10000 may cause performance issues
  ```

- [ ] **Missing WHERE on UPDATE/DELETE**: Dangerous, affects all rows
  ```
  error: @update without 'where' clause affects all rows
    = add 'where' clause or use 'all true' to confirm intent
  ```

### Soft Delete Patterns

- [ ] **Missing deleted_at filter**: Table has `deleted_at` column but query doesn't filter it
  ```
  warning: query on 'product' doesn't filter 'deleted_at'
    = table has soft deletes - add 'deleted_at @null' or 'include-deleted true'
  ```

- [ ] **Hard delete on soft-delete table**: Using @delete on table with `deleted_at`
  ```
  warning: @delete on table with 'deleted_at' column
    = consider soft delete with @update instead
  ```

### Relations

- [ ] **first without order-by**: Non-deterministic which row is "first"
  ```
  warning: 'first true' without 'order-by' returns arbitrary row
  ```

- [ ] **Deep nesting**: Performance warning for deeply nested relations
  ```
  warning: relation nested 4 levels deep may cause N+1 queries
  ```

### Upsert

- [ ] **on-conflict target not unique**: Will fail at runtime
  ```
  error: on-conflict target 'status' is not a unique constraint
    = target must be a unique index or primary key
  ```

### Unused/Missing

- [ ] **Unused param**: Parameter declared but never referenced
  ```
  warning: param 'filter' is declared but never used
  ```

- [ ] **Unknown column**: Column in select/where doesn't exist (already implemented in LSP)

- [ ] **Unknown table**: Table doesn't exist (already implemented in LSP)

## Implementation

### Phase 1: LSP Extension
Add validation in `collect_diagnostics()` - user sees squiggles immediately.

### Phase 2: Codegen
Add validation pass in query parsing - build fails on errors, warns on warnings.

### Schema Requirements

Some validations need schema metadata we may not have yet:
- Unique constraints (for upsert validation)
- Column nullability
- Default values

May need to expand `SchemaInfo` to include constraint information.
