# dibs Roadmap

## Completed

- [x] Schema collection via facet reflection
- [x] Table, column, and constraint metadata
- [x] Primary keys, unique constraints, indices
- [x] Foreign keys with references
- [x] Composite indices
- [x] Default values
- [x] TUI schema browser with vim-style navigation
- [x] Plain text output (for piping)
- [x] SQL generation (`dibs sql`)
- [x] Integration tests with testcontainers + Postgres 18

## In Progress

- [ ] TUI-centric refactor
  - [ ] `dibs` launches TUI by default
  - [ ] `dibs -d <url>` connects to database
  - [ ] `dibs check` for pre-commit hooks

## Planned

### Database Connection in TUI
- Connect to Postgres from TUI
- Show connection status in header/footer
- Query `information_schema` for live database state
- Color-code tables based on sync status

### Schema Diffing
- Compare code schema vs database schema
- Detect:
  - Missing tables (in code but not DB)
  - Extra tables (in DB but not code)
  - Column changes (type, nullability, default)
  - Index changes
  - Foreign key changes
- Show diff in TUI with highlighting

### Migration Generation
- Generate migration SQL from diff
- Smart naming (timestamp + description)
- Preview before creating
- Open in `$EDITOR` after creation
- Track applied migrations (migrations table)

### Migration Execution
- Apply pending migrations
- Rollback support (if down migrations provided)
- Migration history/status

### Advanced Features
- Schema validation (check for common issues)
- Generate Rust types from database (reverse engineering)
- Support for more column types
- Custom type mappings
- Schema snapshots for CI comparison

## Non-Goals (for now)

- ORM functionality (dibs is schema-focused)
- Query building
- Connection pooling (use existing solutions)
- Support for databases other than Postgres
