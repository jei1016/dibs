<script lang="ts">
    import type { TableInfo } from "../types.js";

    interface Props {
        tables: TableInfo[];
        selected: string | null;
        onSelect: (tableName: string) => void;
    }

    let { tables, selected, onSelect }: Props = $props();
</script>

<aside class="bg-neutral-900 p-6 overflow-y-auto border-r border-neutral-800">
    <h2 class="text-xs font-medium text-neutral-500 uppercase tracking-widest mb-6">Tables</h2>
    <ul class="space-y-1">
        {#each tables as table}
            <li>
                <button
                    class="w-full text-left px-3 py-2 text-sm transition-colors {selected ===
                    table.name
                        ? 'text-white bg-neutral-800'
                        : 'text-neutral-400 hover:text-white'}"
                    onclick={() => onSelect(table.name)}
                >
                    {table.name}
                </button>
            </li>
        {/each}
    </ul>
</aside>
