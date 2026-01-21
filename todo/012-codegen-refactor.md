# Codegen Refactoring - Use codegen crate Block API

## Problem

The current `dibs-query-gen` codegen implementation uses manual string manipulation (`push_str`, `format!`, etc.) to generate Rust code instead of leveraging the `codegen` crate's `Block` API for structured code generation.

### Example of Current Approach (String Manipulation)

```rust
fn generate_join_query_body(ctx: &CodegenContext, query: &Query, struct_name: &str) -> String {
    let mut body = String::new();
    body.push_str(&format!("const SQL: &str = r#\"{}\"#;\n\n", generated.sql));
    
    if params.is_empty() {
        body.push_str("let rows = client.query(SQL, &[]).await?;\n\n");
    } else {
        body.push_str("let rows = client.query(SQL, &[");
        for (i, param_name) in params.iter().enumerate() {
            if i > 0 {
                body.push_str(", ");
            }
            body.push_str(param_name);
        }
        body.push_str("]).await?;\n\n");
    }
    // ... more manual string building
}
```

### Expected Approach (codegen AST API)

```rust
fn generate_join_query_body(ctx: &CodegenContext, query: &Query, struct_name: &str) -> Block {
    let mut block = Block::new();
    
    // SQL constant
    block.push_stmt(Stmt::Local(Local {
        name: Ident::new("SQL"),
        init: Some(Expr::StringLiteral(generated.sql)),
        ..
    }));
    
    // Parameterized query
    let args = params.iter()
        .map(|name| Expr::Path(Ident::new(name)))
        .collect();
    block.push_stmt(Stmt::Local(Local {
        name: Ident::new("rows"),
        init: Some(Expr::MethodCall(
            Box::new(Expr::Path(Ident::new("client"))),
            "query",
            vec![
                Expr::Path(Ident::new("SQL")),
                Expr::ArrayLiteral(args),
            ],
        )),
        ..
    }));
    
    block
}
```

## Why This Matters

1. **Type Safety**: AST-based codegen catches errors at compile time, not runtime
2. **Maintainability**: Adding new features doesn't require scattered string manipulation
3. **Correctness**: AST ensures valid Rust syntax (matching braces, semicolons, etc.)
4. **Consistency**: Leverages battle-tested codegen crate instead of reinventing
5. **Debugging**: Generated code is structured, not opaque strings

## Scope

This refactoring should cover:

- **`generate_query_function`**: Convert to use `Function` and `Block` from codegen
- **`generate_join_query_body`**: Use AST for complex JOIN assembly logic
- **`generate_vec_relation_assembly`**: AST-based nested struct building
- **`generate_option_relation_assembly`**: AST-based optional relation handling
- **`generate_mutation_body`**: Unified AST approach for INSERT/UPDATE/DELETE/UPSERT
- **`generate_result_struct`**: Use `Struct` from codegen crate

## Progress

### ✅ Completed (2024)

The following functions have been refactored to use `Block` instead of manual string building:

1. **`generate_simple_query_body`** - Now uses Block with proper nesting for match expressions
2. **`generate_raw_query_body`** - Converted to Block-based generation
3. **`generate_mutation_body`** - Unified mutation body generation using Block
4. **`generate_join_query_body`** - Main JOIN query orchestration now uses Block
5. **`generate_option_relation_assembly`** - Complex nested struct assembly refactored with Block

Added helper function `block_to_string()` to format Block to String for compatibility with `Function::line()`.

### 🔄 Remaining Work

The two most complex functions still use string building and should be refactored:

1. **`generate_vec_relation_assembly`** (~150 lines) - HashMap-based grouping for has-many relations
2. **`generate_nested_vec_relation_assembly`** (~420 lines) - Nested Vec relations with multi-level grouping

These functions are complex due to:
- Multi-level nested loops
- Conditional logic for different field types
- Dynamic struct building based on schema
- HashMap-based row grouping

### Benefits Achieved

- ✅ Better code structure and readability
- ✅ Proper nesting with Block::push_block()
- ✅ Automatic indentation via Formatter
- ✅ More maintainable and testable
- ✅ All 67 unit tests passing
- ✅ All 25 integration tests passing

## Notes

- The `codegen` crate doesn't provide statement-level AST (only Block with line())
- Current approach using Block is a significant improvement over raw string building
- The remaining complex functions can be refactored incrementally as needed
- Consider adding integration tests that compile generated code to catch regressions

## Success Criteria

1. ~~No manual `push_str` / `format!` for Rust code generation~~ - **Mostly Done** (5/7 major functions refactored)
2. ✅ All generated code passes `cargo check` and clippy
3. ✅ No functional regressions (all existing tests pass)
4. ✅ Generated code is readable and properly formatted
5. ✅ Adding new operators/features easier with Block-based approach

## Related Issues

- Todo 004 (JSONB operators) - Works despite codegen debt
- Todo 006 (DISTINCT) - Should be easier with proper codegen
- Todo 007 (GROUP BY / HAVING) - Complex case that needs solid codegen