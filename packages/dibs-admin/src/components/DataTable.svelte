<script lang="ts">
    import { CaretUp, CaretDown, Clock } from "phosphor-svelte";
    import type { Row, ColumnInfo, Value, Sort, SortDir } from "../types.js";

    interface Props {
        columns: ColumnInfo[];
        rows: Row[];
        sort: Sort | null;
        onSort: (column: string) => void;
        onRowClick?: (row: Row) => void;
    }

    let { columns, rows, sort, onSort, onRowClick }: Props = $props();

    // Time display mode: "relative" or "absolute"
    let timeMode = $state<"relative" | "absolute">("relative");

    function isTimestampColumn(col: ColumnInfo): boolean {
        const t = col.sql_type.toUpperCase();
        return t.includes("TIMESTAMP") || t.includes("TIMESTAMPTZ");
    }

    function parseTimestamp(value: string): Date | null {
        // Remove quotes if present
        const cleaned = value.replace(/^"|"$/g, "");
        const date = new Date(cleaned);
        return isNaN(date.getTime()) ? null : date;
    }

    function formatRelativeTime(date: Date): string {
        const now = new Date();
        const diffMs = now.getTime() - date.getTime();
        const diffSec = Math.floor(diffMs / 1000);
        const diffMin = Math.floor(diffSec / 60);
        const diffHour = Math.floor(diffMin / 60);
        const diffDay = Math.floor(diffHour / 24);

        if (diffSec < 60) return "just now";
        if (diffMin < 60) return `${diffMin}m ago`;
        if (diffHour < 24) return `${diffHour}h ago`;
        if (diffDay < 30) return `${diffDay}d ago`;
        return date.toLocaleDateString();
    }

    function formatValue(value: Value, col?: ColumnInfo): string {
        if (value.tag === "Null") return "null";
        if (typeof value.value === "bigint") {
            return value.value.toString();
        }
        if (value.tag === "Bytes") {
            return `<${value.value.length} bytes>`;
        }
        if (value.tag === "String") {
            // Check if this is a timestamp column and format accordingly
            if (col && isTimestampColumn(col) && timeMode === "relative") {
                const date = parseTimestamp(value.value);
                if (date) return formatRelativeTime(date);
            }
            if (value.value.length > 100) {
                return value.value.slice(0, 100) + "...";
            }
            return value.value;
        }
        return JSON.stringify(value.value);
    }

    function getRowValue(row: Row, col: ColumnInfo): { value: string; isNull: boolean } {
        const field = row.fields.find((f) => f.name === col.name);
        if (!field) return { value: "null", isNull: true };
        return {
            value: formatValue(field.value, col),
            isNull: field.value.tag === "Null",
        };
    }

    function handleHeaderClick(colName: string) {
        onSort(colName);
    }

    function handleRowClick(row: Row) {
        onRowClick?.(row);
    }

    function getSortDir(colName: string): SortDir["tag"] | null {
        if (sort && sort.field === colName) {
            return sort.dir.tag;
        }
        return null;
    }
</script>

<div class="flex-1 overflow-auto">
    {#if columns.some(isTimestampColumn)}
        <div class="flex justify-end mb-3">
            <button
                class="inline-flex items-center gap-2 text-xs text-neutral-500 hover:text-white transition-colors"
                onclick={() => timeMode = timeMode === "relative" ? "absolute" : "relative"}
            >
                <Clock size={14} />
                {timeMode === "relative" ? "relative" : "absolute"}
            </button>
        </div>
    {/if}
    <table class="w-full border-collapse text-sm">
        <thead>
            <tr>
                {#each columns as col}
                    {@const sortDir = getSortDir(col.name)}
                    <th
                        class="px-4 py-3 text-left text-neutral-500 font-medium sticky top-0 cursor-pointer select-none whitespace-nowrap bg-neutral-950 hover:text-white transition-colors"
                        onclick={() => handleHeaderClick(col.name)}
                    >
                        <span class="inline-flex items-center gap-2">
                            {col.name}
                            <span
                                class="opacity-0 transition-opacity"
                                class:opacity-100={sortDir !== null}
                            >
                                {#if sortDir === "Desc"}
                                    <CaretDown size={12} weight="bold" />
                                {:else}
                                    <CaretUp size={12} weight="bold" />
                                {/if}
                            </span>
                        </span>
                    </th>
                {/each}
            </tr>
        </thead>
        <tbody>
            {#each rows as row}
                {@const clickable = onRowClick !== undefined}
                <tr
                    class="border-t border-neutral-900 transition-colors {clickable
                        ? 'cursor-pointer hover:bg-neutral-900'
                        : ''}"
                    onclick={() => handleRowClick(row)}
                >
                    {#each columns as col}
                        {@const cell = getRowValue(row, col)}
                        <td
                            class="px-4 py-3 text-sm max-w-[300px] overflow-hidden text-ellipsis whitespace-nowrap"
                            class:text-neutral-600={cell.isNull}
                        >
                            {cell.value}
                        </td>
                    {/each}
                </tr>
            {/each}
        </tbody>
    </table>
</div>
