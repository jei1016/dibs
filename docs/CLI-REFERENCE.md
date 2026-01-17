# CLI Reference

## Synopsis

```
dibs [OPTIONS] [COMMAND]
```

## Options

```
-d, --database-url <URL>    Database connection URL
-V, --version               Show version information
-h, --help                  Show help
```

## Commands

### (default) - Launch TUI

```bash
dibs
dibs -d postgres://user:pass@localhost/mydb
```

Launches the interactive TUI for browsing schema and managing migrations.

Without `-d`: Shows schema defined in code only.
With `-d`: Connects to database and shows diff.

### sql - Output Schema SQL

```bash
dibs sql
```

Outputs CREATE TABLE statements for the entire schema. Useful for:
- CI pipelines
- Generating initial migration
- Documentation

Exit codes:
- 0: Success
- 1: No tables registered

### check - Verify Schema Sync

```bash
dibs check
dibs check -d postgres://...
```

Compares code schema against database and exits non-zero if they differ.

Intended for pre-commit/pre-push hooks.

Exit codes:
- 0: Schema in sync (or no database URL provided)
- 1: Schema drift detected
- 2: Connection error

Output on drift:
```
❌ Database schema out of sync with code

  Missing tables:
    + audit_logs

  Column changes in 'users':
    + email_verified (boolean)
    - legacy_flag

Run `dibs` to generate a migration.
```

## Environment Variables

```
DATABASE_URL    Default database connection URL (if -d not provided)
EDITOR          Editor to open generated migrations
```

## Examples

```bash
# Browse schema in TUI
dibs

# Browse with database connection
dibs -d postgres://localhost/mydb

# Generate SQL for CI
dibs sql > schema.sql

# Pre-commit hook
dibs check -d $DATABASE_URL || exit 1

# Using DATABASE_URL env var
export DATABASE_URL=postgres://localhost/mydb
dibs check
```
