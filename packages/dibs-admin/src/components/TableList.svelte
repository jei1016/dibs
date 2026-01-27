<script lang="ts">
    import ClockIcon from "phosphor-svelte/lib/ClockIcon";
    import HouseIcon from "phosphor-svelte/lib/HouseIcon";
    import MagnifyingGlassIcon from "phosphor-svelte/lib/MagnifyingGlassIcon";
    import type { TableInfo } from "@bearcove/dibs-admin/types";
    import type { DibsAdminConfig } from "@bearcove/dibs-admin/types/config";
    import { getTableLabel } from "@bearcove/dibs-admin/lib/config";
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
        onTimeModeChange,
    }: Props = $props();

    let filterText = $state("");

    const filteredTables = $derived(
        filterText.trim() === ""
            ? tables
            : tables.filter((t) => {
                  const label = getTableLabel(config, t.name).toLowerCase();
                  const name = t.name.toLowerCase();
                  const query = filterText.toLowerCase();
                  return label.includes(query) || name.includes(query);
              }),
    );

    function toggleTimeMode() {
        const newMode = timeMode === "relative" ? "absolute" : "relative";
        onTimeModeChange?.(newMode);
    }
</script>

<aside class="sidebar">
    {#if showDashboardButton}
        <button
            class="nav-button dashboard-button"
            class:active={dashboardActive}
            onclick={onDashboard}
        >
            <HouseIcon size={16} class="nav-icon" />
            <span>Dashboard</span>
        </button>
    {/if}

    <h2 class="section-title">Tables</h2>
    <div class="filter-input">
        <MagnifyingGlassIcon size={14} class="filter-icon" />
        <input type="text" placeholder="Filter tables..." bind:value={filterText} />
    </div>
    <ul class="table-list">
        {#each filteredTables as table}
            <li>
                <button
                    class="nav-button"
                    class:active={selected === table.name && !dashboardActive}
                    onclick={() => onSelect(table.name)}
                >
                    <DynamicIcon name={table.icon ?? "table"} size={16} class="nav-icon" />
                    <span class="truncate">{getTableLabel(config, table.name) ?? table.name}</span>
                </button>
            </li>
        {/each}
    </ul>

    <div class="settings">
        <button class="settings-button" onclick={toggleTimeMode}>
            <ClockIcon size={14} />
            <span>Times: {timeMode}</span>
        </button>
    </div>
</aside>

<style>
    .sidebar {
        background-color: var(--sidebar);
        padding: 1.5rem;
        overflow-y: auto;
        border-right: 1px solid var(--sidebar-border);
        display: flex;
        flex-direction: column;
        min-height: 100vh;
    }

    .nav-button {
        width: 100%;
        text-align: left;
        padding: 0.5rem 0.75rem;
        font-size: 0.875rem;
        border-radius: var(--radius-md);
        transition: all 0.15s;
        display: flex;
        align-items: center;
        gap: 0.5rem;
        background: none;
        border: none;
        cursor: pointer;
        color: color-mix(in oklch, var(--sidebar-foreground) 70%, transparent);
    }

    .nav-button:hover {
        color: var(--sidebar-foreground);
        background-color: var(--sidebar-accent);
    }

    .nav-button.active {
        color: var(--sidebar-primary-foreground);
        background-color: var(--sidebar-primary);
        font-weight: 500;
    }

    .nav-button :global(.nav-icon) {
        flex-shrink: 0;
        opacity: 0.7;
    }

    .dashboard-button {
        margin-bottom: 1rem;
    }

    .section-title {
        font-size: 0.75rem;
        font-weight: 500;
        color: color-mix(in oklch, var(--sidebar-foreground) 60%, transparent);
        text-transform: uppercase;
        letter-spacing: 0.1em;
        margin-bottom: 0.75rem;
    }

    .filter-input {
        position: relative;
        margin-bottom: 0.75rem;
    }

    .filter-input :global(.filter-icon) {
        position: absolute;
        left: 0.75rem;
        top: 50%;
        transform: translateY(-50%);
        color: color-mix(in oklch, var(--sidebar-foreground) 50%, transparent);
        pointer-events: none;
    }

    .filter-input input {
        width: 100%;
        padding: 0.5rem 0.75rem 0.5rem 2rem;
        font-size: 0.8125rem;
        border: 1px solid var(--sidebar-border);
        border-radius: var(--radius-md);
        background: color-mix(in oklch, var(--sidebar) 80%, black);
        color: var(--sidebar-foreground);
        outline: none;
        transition: border-color 0.15s;
    }

    .filter-input input::placeholder {
        color: color-mix(in oklch, var(--sidebar-foreground) 40%, transparent);
    }

    .filter-input input:focus {
        border-color: var(--sidebar-primary);
    }

    .table-list {
        list-style: none;
        padding: 0;
        margin: 0;
        display: flex;
        flex-direction: column;
        gap: 0.25rem;
        flex: 1;
    }

    .truncate {
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
    }

    .settings {
        padding-top: 1rem;
        margin-top: 1rem;
        border-top: 1px solid var(--sidebar-border);
    }

    .settings-button {
        width: 100%;
        text-align: left;
        padding: 0.5rem 0.75rem;
        font-size: 0.75rem;
        border-radius: var(--radius-md);
        transition: all 0.15s;
        display: flex;
        align-items: center;
        gap: 0.5rem;
        background: none;
        border: none;
        cursor: pointer;
        color: color-mix(in oklch, var(--sidebar-foreground) 60%, transparent);
    }

    .settings-button:hover {
        color: var(--sidebar-foreground);
        background-color: var(--sidebar-accent);
    }
</style>
