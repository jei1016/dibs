<script lang="ts">
  import { untrack, onMount } from "svelte";
  import { Router, useNavigate } from "@bearcove/sextant";
  import type { SquelServiceCaller, SchemaInfo, Row } from "@bearcove/dibs-admin/types";
  import type { DibsAdminConfig } from "@bearcove/dibs-admin/types/config";
  import TableList from "./components/TableList.svelte";
  import { Tooltip } from "@bearcove/dibs-admin/lib/ui";
  import { isTableHidden, hasDashboard } from "@bearcove/dibs-admin/lib/config";
  import { schemaCache } from "./lib/schema-cache.js";
  import { setAdminContext, type BreadcrumbEntry } from "./lib/admin-context.js";
  import { adminRoutes } from "./routes.js";
  import "@bearcove/dibs-admin/styles/tokens.css";

  interface Props {
    client: SquelServiceCaller;
    config?: DibsAdminConfig;
  }

  let { client, config }: Props = $props();

  // Navigation
  const navigate = useNavigate();

  // Schema state
  let schema = $state<SchemaInfo | null>(schemaCache.get(client) ?? null);
  let loading = $state(false);
  let error = $state<string | null>(null);

  // Shared state
  let fkLookup = $state<Map<string, Map<string, Row>>>(new Map());
  let timeMode = $state<"relative" | "absolute">("relative");
  let breadcrumbs = $state<BreadcrumbEntry[]>([]);
  let selectedTable = $state<string | null>(null);

  // Filter hidden tables
  let visibleTables = $derived(
    schema?.tables.filter((t) => !isTableHidden(config, t.name)) ?? []
  );

  // Context - views will use this for navigation and shared state
  setAdminContext({
    client,
    config,
    get schema() {
      return schema;
    },
    navigateToDashboard: () => {
      breadcrumbs = [];
      selectedTable = null;
      navigate(adminRoutes.dashboard, {});
    },
    navigateToTable: (table: string) => {
      breadcrumbs = [{ table, label: table }];
      selectedTable = table;
      navigate(adminRoutes.tableList, { table });
    },
    navigateToRow: (table: string, pk: string) => {
      navigate(adminRoutes.rowDetail, { table, pk });
    },
    navigateToNewRow: (table: string) => {
      navigate(adminRoutes.rowCreate, { table });
    },
    get fkLookup() {
      return fkLookup;
    },
    setFkLookup: (lookup) => {
      fkLookup = lookup;
    },
    get timeMode() {
      return timeMode;
    },
    setTimeMode: (mode) => {
      timeMode = mode;
    },
    get breadcrumbs() {
      return breadcrumbs;
    },
    setBreadcrumbs: (entries) => {
      breadcrumbs = entries;
    },
    addBreadcrumb: (entry) => {
      breadcrumbs = [...breadcrumbs, entry];
    },
  });

  // Load schema on mount
  let schemaLoaded = false;

  $effect(() => {
    untrack(() => loadSchema());
  });

  async function loadSchema() {
    if (schemaLoaded) return;
    schemaLoaded = true;

    const cached = schemaCache.get(client);
    if (cached) {
      schema = cached;
    } else {
      loading = true;
      error = null;
      try {
        schema = await client.schema();
        schemaCache.set(client, schema);
      } catch (e) {
        error = e instanceof Error ? e.message : String(e);
      } finally {
        loading = false;
      }
      if (!schema) return;
    }

    // If no table selected and no dashboard configured, navigate to first visible table
    if (!selectedTable && !hasDashboard(config) && schema.tables.length > 0) {
      const firstVisible = schema.tables.find((t) => !isTableHidden(config, t.name));
      if (firstVisible) {
        selectTable(firstVisible.name);
      }
    }
  }

  function selectTable(tableName: string) {
    breadcrumbs = [{ table: tableName, label: tableName }];
    selectedTable = tableName;
    navigate(adminRoutes.tableList, { table: tableName });
  }

  function goToDashboard() {
    breadcrumbs = [];
    selectedTable = null;
    navigate(adminRoutes.dashboard, {});
  }
</script>

<Tooltip.Provider>
  <div class="admin-root">
    {#if loading && !schema}
      <div class="loading-state">Loading schema...</div>
    {:else if schema}
      <div class="admin-layout">
        <TableList
          tables={visibleTables}
          selected={selectedTable}
          onSelect={selectTable}
          {config}
          showDashboardButton={hasDashboard(config)}
          onDashboard={goToDashboard}
          dashboardActive={!selectedTable}
          {timeMode}
          onTimeModeChange={(mode) => (timeMode = mode)}
        />

        <main class="admin-content">
          <Router routes={adminRoutes} />
        </main>
      </div>
    {:else if error}
      <p class="error-message standalone">{error}</p>
    {/if}
  </div>
</Tooltip.Provider>

<style>
  .admin-root {
    height: 100%;
    min-height: 100vh;
    background-color: var(--background);
    color: var(--foreground);
  }

  .loading-state {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100%;
    padding: 2rem;
    color: var(--muted-foreground);
  }

  .admin-layout {
    display: grid;
    grid-template-columns: 280px 1fr;
    min-height: 100vh;
  }

  .admin-content {
    overflow: auto;
    display: flex;
    flex-direction: column;
    max-height: 100vh;
  }

  .error-message {
    color: var(--destructive);
    margin-bottom: 1.5rem;
    font-size: 0.875rem;
  }

  .error-message.standalone {
    padding: 2rem;
    margin-bottom: 0;
  }
</style>
