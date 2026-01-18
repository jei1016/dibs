# roam-codegen TypeScript gaps

Issues found while building the dibs admin-ui:

## 1. No connection helper generated ✅ FIXED

The generated client now includes a `connect<ServiceName>(url)` function that handles WebSocket connection and roam handshake.

## 2. Duplicate type names ✅ FIXED

Request/response type aliases now skip generation if they would conflict with an existing named type (e.g., `ListRequest` interface).

## 3. Missing encode/decode for complex methods ✅ FIXED

All methods now use schema-driven encoding via `encodeWithSchema`/`decodeWithSchema`. No more "Not yet implemented" errors.

## 4. MethodSchema missing `returns` field ✅ FIXED

Added `returns: Schema` to the `MethodSchema` interface in `@bearcove/roam-postcard`.

## Remaining TODOs

### Streaming channels (Tx/Rx)

The decode.rs still has TODOs for Tx/Rx channel types:
```rust
// TODO: Need Connection access to create proper Tx handle
// TODO: Need Connection access to create proper Rx handle
```

These aren't used by SquelService, so they're not blocking. They would need to be addressed for services that use streaming.
