# Phase 4: Language Server Protocol (LSP)

## Goal

Provide IDE support for `.styx` query files:
- Autocomplete for table names, column names, relation names
- Hover info showing types
- Go-to-definition for tables/columns
- Diagnostics (errors/warnings)
- Syntax highlighting (via TextMate grammar or tree-sitter)

## Architecture

```
┌─────────────────┐     ┌──────────────────┐     ┌─────────────────┐
│   VS Code /     │────▶│  dibs-query-lsp  │────▶│  Schema from    │
│   Zed / etc     │◀────│  (Rust binary)   │◀────│  my-app-db      │
└─────────────────┘     └──────────────────┘     └─────────────────┘
        │                       │
        │ LSP protocol          │ Parses .styx files
        │ (JSON-RPC over stdio) │ Validates against schema
        │                       │
```

## Schema Discovery

The LSP needs to know the database schema. Options:

### Option A: Schema manifest file

build.rs in my-app-db generates a `schema.json`:
```json
{
  "tables": {
    "product": {
      "columns": {
        "id": { "type": "i64", "nullable": false },
        "handle": { "type": "String", "nullable": false },
        "status": { "type": "String", "nullable": false }
      },
      "primary_key": "id",
      "relations": {
        "variants": { "table": "product_variant", "fk": "product_id" },
        "translations": { "table": "product_translation", "fk": "product_id" }
      }
    }
  }
}
```

LSP reads this file. Regenerated on schema changes.

### Option B: Connect to running my-app-db

my-app-db exposes schema info via a local socket/HTTP:
```
GET http://localhost:9999/schema
```

More complex but always up-to-date.

### Option C: Parse Rust source directly

LSP parses `my-app-db/src/lib.rs` using syn/rust-analyzer.

Complex, but no build step needed.

**Recommendation**: Start with Option A (manifest file). Simple, fast, cacheable.

## LSP Capabilities

### 1. Completion

**Table names** after `from`:
```styx
from pro|
      ^^^
      Completions: product, product_variant, product_translation, ...
```

**Column names** in `select` and `where`:
```styx
from product
select{ han|
        ^^^
        Completions: handle, id, status, active, created_at, ...
```

**Relation names** in select:
```styx
from product
select{
  id
  trans|
  ^^^^^
  Completions: translations (-> product_translation)
               source (-> product_source)
               variants (-> product_variant)
```

**Parameter references**:
```styx
params{ locale @string, currency @string }
from product
where{ status $|
               ^
               Completions: $locale, $currency
```

### 2. Hover

**On table name**:
```
from product
     ^^^^^^^
┌─────────────────────────────────────┐
│ Table: product                      │
│ Columns: id, handle, status, ...    │
│ PK: id                              │
│ Relations: variants, translations   │
└─────────────────────────────────────┘
```

**On column name**:
```
select{ handle }
        ^^^^^^
┌─────────────────────────────────────┐
│ Column: product.handle              │
│ Type: String (TEXT)                 │
│ Nullable: false                     │
│ Constraint: UNIQUE                  │
└─────────────────────────────────────┘
```

**On relation**:
```
translation @rel{ ... }
^^^^^^^^^^^
┌─────────────────────────────────────┐
│ Relation: translation               │
│ Target: product_translation         │
│ Join: product_translation.product_id│
│       = product.id                  │
└─────────────────────────────────────┘
```

### 3. Diagnostics

Real-time error reporting:

```styx
from products  // Error: unknown table 'products', did you mean 'product'?
     ^^^^^^^^

where{ titel $locale }  // Error: unknown column 'titel', did you mean 'title'?
       ^^^^^

select{ foo @rel{ ... } }  // Error: no relation 'foo' from 'product'
        ^^^
```

### 4. Go to Definition

Click on table name → jump to Rust struct definition.
Click on column name → jump to struct field.

Requires knowing file paths from schema manifest.

### 5. Code Actions

**"Add missing column"** - if you reference a column that doesn't exist, offer to add it to the schema.

**"Generate query function"** - quick action to scaffold the Rust caller.

## Implementation

### Using tower-lsp

```rust
use tower_lsp::{LspService, Server};
use tower_lsp::lsp_types::*;

#[derive(Debug)]
struct DibsQueryLsp {
    schema: SchemaInfo,
}

#[tower_lsp::async_trait]
impl LanguageServer for DibsQueryLsp {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                completion_provider: Some(CompletionOptions::default()),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                // ...
            },
            ..Default::default()
        })
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let uri = params.text_document_position.text_document.uri;
        let position = params.text_document_position.position;

        // Parse document, find context at position
        // Return relevant completions based on schema
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        // Parse document, find item at position
        // Return type info from schema
    }
}
```

### Extending styx LSP

styx already has an LSP (`styx lsp`). We could:
1. Fork/extend it with query-specific features
2. Run alongside it (separate LSP for `.styx` files in queries/)
3. Add plugin system to styx LSP

**Recommendation**: Start as separate LSP, consider merging later.

## Editor Integration

### VS Code

Extension provides:
- Language configuration for `.styx` in `queries/` directories
- LSP client configuration
- Syntax highlighting via TextMate grammar

```json
{
  "contributes": {
    "languages": [{
      "id": "dibs-query",
      "extensions": [".styx"],
      "configuration": "./language-configuration.json"
    }],
    "grammars": [{
      "language": "dibs-query",
      "scopeName": "source.dibs-query",
      "path": "./syntaxes/dibs-query.tmLanguage.json"
    }]
  }
}
```

### Zed

Zed extension with:
- Tree-sitter grammar (styx already has one)
- LSP configuration

### Neovim

nvim-lspconfig entry + tree-sitter parser.

## Incremental Parsing

For large files, use incremental parsing:
1. On document change, get changed range
2. Reparse only affected queries
3. Revalidate only changed queries against schema

styx-parse may need incremental support, or we cache parsed trees.

## Deliverables

1. `dibs-query-lsp` binary
   - Schema manifest loader
   - Completion provider
   - Hover provider
   - Diagnostic publisher

2. Schema manifest generation in build.rs

3. VS Code extension (basic)

4. Documentation for editor setup
