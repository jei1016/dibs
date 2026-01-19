<script lang="ts">
    import { Clock, House } from "phosphor-svelte";
    import type { TableInfo } from "../types.js";
    import type { DibsAdminConfig } from "../types/config.js";
    import { getTableLabel } from "../lib/config.js";
    import DynamicIcon from "./DynamicIcon.svelte";

    interface Props {
        tables: TableInfo[];
        selected: string | null;
        onSelect: (tableName: string) => void;
        config?: DibsAdminConfig;
        showDashboardButton?: boolean;
        onDashboard?: () => void;
        dashboardActive?: boolean;
        timeMode?: "relative" | "absolute";
        onTimeModeChange?: (mode: "relative" | "absolute") => void;
    }

    let {
        tables,
        selected,
        onSelect,
        config,
        showDashboardButton = false,
        onDashboard,
        dashboardActive = false,
        timeMode = "relative",
        onTimeModeChange
    }: Props = $props();

    function toggleTimeMode() {
        const newMode = timeMode === "relative" ? "absolute" : "relative";
        onTimeModeChange?.(newMode);
    }
</script>

<aside class="bg-sidebar p-6 overflow-y-auto border-r border-sidebar-border flex flex-col">
    {#if showDashboardButton}
        <button
            class="w-full text-left px-3 py-2 text-sm rounded-md transition-all duration-150 flex items-center gap-2 mb-4 {dashboardActive
                ? 'text-sidebar-primary-foreground bg-sidebar-primary font-medium'
                : 'text-sidebar-foreground/70 hover:text-sidebar-foreground hover:bg-sidebar-accent'}"
            onclick={onDashboard}
        >
            <House size={16} class="shrink-0 opacity-70" />
            <span>Dashboard</span>
        </button>
    {/if}

    <h2 class="text-xs font-medium text-sidebar-foreground/60 uppercase tracking-widest mb-4">Tables</h2>
    <ul class="space-y-1 flex-1">
        {#each tables as table}
            <li>
                <button
                    class="w-full text-left px-3 py-2 text-sm rounded-md transition-all duration-150 flex items-center gap-2 {selected ===
                    table.name && !dashboardActive
                        ? 'text-sidebar-primary-foreground bg-sidebar-primary font-medium'
                        : 'text-sidebar-foreground/70 hover:text-sidebar-foreground hover:bg-sidebar-accent'}"
                    onclick={() => onSelect(table.name)}
                >
                    <DynamicIcon name={table.icon ?? "table"} size={16} class="shrink-0 opacity-70" />
                    <span class="truncate">{getTableLabel(config, table.name) ?? table.name}</span>
                </button>
            </li>
        {/each}
    </ul>

    <!-- Settings -->
    <div class="pt-4 mt-4 border-t border-sidebar-border">
        <button
            class="w-full text-left px-3 py-2 text-xs rounded-md transition-all duration-150 flex items-center gap-2 text-sidebar-foreground/60 hover:text-sidebar-foreground hover:bg-sidebar-accent"
            onclick={toggleTimeMode}
        >
            <Clock size={14} />
            <span>Times: {timeMode}</span>
        </button>
    </div>
</aside>
