// Simple components - direct export for easy usage
export { Button, buttonVariants } from "./button/index.js";
export { Input } from "./input/index.js";
export { Textarea } from "./textarea/index.js";
export { Checkbox } from "./checkbox/index.js";
export { Label } from "./label/index.js";
export { Separator } from "./separator/index.js";
export { Skeleton } from "./skeleton/index.js";
export { Badge } from "./badge/index.js";

// Complex components - namespace export for composition
export * as Dialog from "./dialog/index.js";
export * as Table from "./table/index.js";
export * as Card from "./card/index.js";
export * as ScrollArea from "./scroll-area/index.js";
export * as Tooltip from "./tooltip/index.js";
export * as Popover from "./popover/index.js";
export * as DropdownMenu from "./dropdown-menu/index.js";
export * as AlertDialog from "./alert-dialog/index.js";
export * as Command from "./command/index.js";
export * as Breadcrumb from "./breadcrumb/index.js";
export * as Pagination from "./pagination/index.js";
export * as Sidebar from "./sidebar/index.js";
export * as Sheet from "./sheet/index.js";
export * as Select from "./select/index.js";
export * as DataTable from "./data-table/index.js";

// Custom components
export { default as NumberInput } from "./number-input.svelte";
export { default as DatetimeInput } from "./datetime-input.svelte";
