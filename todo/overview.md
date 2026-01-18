# dibs roadmap

Schema-first Postgres toolkit for Rust, powered by facet reflection.

## Phases

| # | Status | Description |
|---|--------|-------------|
| [001](./001-TODO-schema-definition.md) | DONE | Schema definition via facet attributes |
| [002](./002-TODO-schema-introspection.md) | IN PROGRESS | Read schema from Postgres |
| [003](./003-TODO-schema-diffing.md) | TODO | Compare Rust vs database schema |
| [004](./004-TODO-migration-generation.md) | TODO | Generate Rust migration files |
| [005](./005-TODO-migration-execution.md) | TODO | Run and track migrations |
| [006](./006-TODO-query-building.md) | TODO | Type-safe queries (stretch) |

## Design decisions

### CLI-driven migrations (not automatic at startup)

Migrations run via `dibs migrate`, not automatically when the app starts. Reasons:
- Avoids race conditions with multiple replicas
- Clear failure point for debugging
- Can review/inspect before running
- Doesn't block health checks during slow migrations

### No `down` migrations

Only forward migrations. To rollback, write a new forward migration that undoes the change.
- Down migrations are rarely used in production
- Hard to write correctly for data migrations
- Adds cognitive overhead

### Diff against live database

`dibs diff` introspects the actual database via `information_schema`, not a local snapshot.
- Works against dev DB (for generating migrations)
- Works against staging/prod (for verification)
- Catches manual schema changes

## Current focus

Phase 002: Schema Introspection
