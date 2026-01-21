# 005: JSONB Operators

**Status:** ✅ COMPLETED

**Priority:** Medium

## Goal

Support PostgreSQL JSONB operators for flexible schema patterns.

## Completion Summary

All JSONB operators have been implemented and tested:
- ✅ `@json-get` for `->` operator (get JSON object)
- ✅ `@json-get-text` for `->>` operator (get JSON value as text)
- ✅ `@contains` for `@>` operator (containment check)
- ✅ `@key-exists` for `?` operator (key existence check)

**Integration Tests Added:**
- 10 comprehensive integration tests in `postgres_integration.rs`
- Tests cover parameterized and literal usage patterns
- Tests for complex queries combining multiple operators
- NULL and empty object edge case handling
- All tests execute against real PostgreSQL in Docker

## Syntax Ideas

**Path access in SELECT:**
```styx
select{
  id
  brand @json{ path "metadata.brand" }
}
```
→ `metadata->'brand' as brand`

**Filtering:**
```styx
where{
  metadata @json_path{ path "$.brand", eq $brand }
}
```
→ `metadata->>'brand' = $1`

**Containment:**
```styx
where{
  metadata @json_contains{ "premium": true }
}
```
→ `metadata @> '{"premium":true}'`

**Key existence:**
```styx
where{
  metadata @json_has_key{ "premium" }
}
```
→ `metadata ? 'premium'`

## PostgreSQL Operators

| Op | Description |
|----|-------------|
| `->` | Get field as JSON |
| `->>` | Get field as text |
| `@>` | Contains |
| `?` | Key exists |
| `?|` | Any key exists |
| `?&` | All keys exist |

## Files

- `ast.rs` - Add JSON field/filter types
- `parse.rs` - Parse JSON syntax
- `sql.rs` - Generate JSONB SQL
