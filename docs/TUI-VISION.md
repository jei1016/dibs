# TUI-Centric Workflow Vision

dibs is designed with a TUI-first approach for human interaction, with minimal CLI commands for automation.

## Philosophy

- **TUI for humans**: Interactive exploration, diffing, and migration generation
- **CLI for machines**: Pre-commit hooks, CI pipelines, scripts

## Running dibs

### Default: Launch TUI

```bash
dibs                              # Browse schema (no DB connection)
dibs -d postgres://...            # Browse schema + connected to DB
dibs --database-url postgres://...
```

The TUI shows:
- Schema tables defined in code
- Database tables (if connected)
- Diff between schema and database
- Migration generation workflow

### Automation Commands

```bash
dibs sql      # Output schema as CREATE TABLE statements (for CI)
dibs check    # Exit non-zero if schema != database (for pre-commit)
```

## Pre-commit Hook Example

```yaml
# .pre-commit-config.yaml
repos:
  - repo: local
    hooks:
      - id: dibs-check
        name: Check schema sync
        entry: dibs check --database-url $DATABASE_URL
        language: system
        pass_filenames: false
```

When schema drifts:

```
$ dibs check
❌ Database schema out of sync with code

  Missing tables:
    + audit_logs

  Column changes in 'users':
    + email_verified (boolean)
    - legacy_flag

Run `dibs` to generate a migration.
```

## TUI Features

### Schema Browser (Current)
- Left panel: list of tables
- Right panel: table details (columns, types, constraints, indices)
- Vim-style navigation (j/k/g/G/Ctrl-d/Ctrl-u)

### Database Connection (Planned)
- Show connection status in header
- Color-code tables: green (in sync), yellow (drift), red (missing)

### Diff View (Planned)
- Side-by-side: schema vs database
- Highlight additions, removals, changes
- Press `d` to toggle diff mode

### Migration Generation (Planned)
- Press `g` to generate migration from current diff
- Preview SQL in a panel
- Edit migration name (default: timestamp + auto-description)
- Confirm to create file
- Option to open in `$EDITOR`

## Keybindings

```
Navigation:
  j/↓     Move down
  k/↑     Move up
  h/←     Focus tables list
  l/→     Focus details
  gg      Go to first
  G       Go to last
  Ctrl-d  Half page down
  Ctrl-u  Half page up
  Tab     Toggle focus
  q/Esc   Quit

Actions:
  d       Toggle diff view (when connected)
  g       Generate migration (when drift detected)
  r       Refresh database state
  ?       Show help
```
