import type { SquelServiceCaller, SchemaInfo } from "@bearcove/dibs-admin/types";

// Module-level cache that persists across component remounts
export const schemaCache = new WeakMap<SquelServiceCaller, SchemaInfo>();
