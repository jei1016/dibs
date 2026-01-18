<script lang="ts">
    import { connect, getClient } from "./lib/roam";
    import type {
        SchemaInfo,
        ListRequest,
        Row,
        DibsError,
        Value,
    } from "./lib/generated/squel-service";

    // Connection state
    let connected = $state(false);
    let connecting = $state(false);
    let error = $state<string | null>(null);

    // Schema state
    let schema = $state<SchemaInfo | null>(null);
    let selectedTable = $state<string | null>(null);

    // Data state
    let rows = $state<Row[]>([]);
    let loading = $state(false);
    let totalRows = $state<bigint | null>(null);

    // Pagination
    let limit = $state(25);
    let offset = $state(0);

    // Database URL - matches my-app-db/.env default
    const DATABASE_URL = "postgres://localhost/dibs_test";

    async function handleConnect() {
        connecting = true;
        error = null;
        try {
            await connect();
            connected = true;
            // Fetch schema after connecting
            const client = getClient()!;
            schema = await client.schema();
            // Select first table by default
            if (schema.tables.length > 0) {
                selectedTable = schema.tables[0].name;
                await loadData();
            }
        } catch (e) {
            error = e instanceof Error ? e.message : String(e);
        } finally {
            connecting = false;
        }
    }

    async function loadData() {
        if (!selectedTable) return;
        const client = getClient();
        if (!client) return;

        loading = true;
        error = null;
        try {
            const request: ListRequest = {
                database_url: DATABASE_URL,
                table: selectedTable,
                filters: [],
                sort: [],
                limit,
                offset,
                select: [],
            };
            const result = await client.list(request);
            if (result.ok) {
                rows = result.value.rows;
                totalRows = result.value.total ?? null;
            } else {
                error = formatError(result.error);
                rows = [];
            }
        } catch (e) {
            error = e instanceof Error ? e.message : String(e);
            rows = [];
        } finally {
            loading = false;
        }
    }

    function formatError(err: DibsError): string {
        return `${err.tag}: ${err.value}`;
    }

    function formatValue(value: Value): string {
        if (value.tag === "Null") return "null";
        // BigInt can't be JSON.stringify'd, so handle it specially
        if (typeof value.value === "bigint") {
            return value.value.toString();
        }
        return JSON.stringify(value.value);
    }

    function getRowValue(row: Row, colName: string): string {
        const field = row.fields.find((f) => f.name === colName);
        if (!field) return "null";
        return formatValue(field.value);
    }

    function selectTable(tableName: string) {
        selectedTable = tableName;
        offset = 0;
        loadData();
    }

    function getTableColumns(): string[] {
        if (!schema || !selectedTable) return [];
        const table = schema.tables.find((t) => t.name === selectedTable);
        return table?.columns.map((c) => c.name) ?? [];
    }

    function nextPage() {
        offset += limit;
        loadData();
    }

    function prevPage() {
        offset = Math.max(0, offset - limit);
        loadData();
    }
</script>

<main>
    <h1>dibs admin</h1>

    {#if !connected}
        <div class="connect-panel">
            <button onclick={handleConnect} disabled={connecting}>
                {connecting ? "Connecting..." : "Connect to ws://127.0.0.1:9000"}
            </button>
            {#if error}
                <p class="error">{error}</p>
            {/if}
        </div>
    {:else}
        <div class="layout">
            <!-- Sidebar with tables -->
            <aside class="sidebar">
                <h2>Tables</h2>
                {#if schema}
                    <ul>
                        {#each schema.tables as table}
                            <li>
                                <button
                                    class:selected={selectedTable === table.name}
                                    onclick={() => selectTable(table.name)}
                                >
                                    {table.name}
                                </button>
                            </li>
                        {/each}
                    </ul>
                {/if}
            </aside>

            <!-- Main content -->
            <section class="content">
                {#if selectedTable}
                    <h2>{selectedTable}</h2>

                    {#if error}
                        <p class="error">{error}</p>
                    {/if}

                    {#if loading}
                        <p>Loading...</p>
                    {:else if rows.length === 0}
                        <p>No rows found.</p>
                    {:else}
                        <div class="table-container">
                            <table>
                                <thead>
                                    <tr>
                                        {#each getTableColumns() as col}
                                            <th>{col}</th>
                                        {/each}
                                    </tr>
                                </thead>
                                <tbody>
                                    {#each rows as row}
                                        <tr>
                                            {#each getTableColumns() as col}
                                                <td>{getRowValue(row, col)}</td>
                                            {/each}
                                        </tr>
                                    {/each}
                                </tbody>
                            </table>
                        </div>

                        <!-- Pagination -->
                        <div class="pagination">
                            <button onclick={prevPage} disabled={offset === 0}>← Previous</button>
                            <span>
                                Showing {offset + 1} - {offset + rows.length}
                                {#if totalRows !== null}
                                    of {totalRows}
                                {/if}
                            </span>
                            <button onclick={nextPage} disabled={rows.length < limit}>Next →</button
                            >
                        </div>
                    {/if}
                {:else}
                    <p>Select a table from the sidebar.</p>
                {/if}
            </section>
        </div>
    {/if}
</main>

<style>
    :global(body) {
        margin: 0;
        font-family:
            system-ui,
            -apple-system,
            sans-serif;
        background: #1a1a2e;
        color: #eee;
    }

    main {
        min-height: 100vh;
        padding: 1rem;
    }

    h1 {
        margin: 0 0 1rem;
        font-size: 1.5rem;
        color: #fff;
    }

    h2 {
        margin: 0 0 0.5rem;
        font-size: 1.1rem;
        color: #aaa;
    }

    .connect-panel {
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 1rem;
        padding: 2rem;
    }

    button {
        background: #16213e;
        color: #eee;
        border: 1px solid #0f3460;
        padding: 0.5rem 1rem;
        border-radius: 4px;
        cursor: pointer;
        font-size: 0.9rem;
    }

    button:hover:not(:disabled) {
        background: #0f3460;
    }

    button:disabled {
        opacity: 0.5;
        cursor: not-allowed;
    }

    .error {
        color: #e94560;
        background: rgba(233, 69, 96, 0.1);
        padding: 0.5rem 1rem;
        border-radius: 4px;
        margin: 0;
    }

    .layout {
        display: grid;
        grid-template-columns: 200px 1fr;
        gap: 1rem;
        height: calc(100vh - 5rem);
    }

    .sidebar {
        background: #16213e;
        border-radius: 8px;
        padding: 1rem;
        overflow-y: auto;
    }

    .sidebar ul {
        list-style: none;
        padding: 0;
        margin: 0;
    }

    .sidebar li {
        margin: 0.25rem 0;
    }

    .sidebar button {
        width: 100%;
        text-align: left;
        background: transparent;
        border: none;
        padding: 0.5rem;
        border-radius: 4px;
    }

    .sidebar button:hover {
        background: #0f3460;
    }

    .sidebar button.selected {
        background: #0f3460;
        border-left: 3px solid #e94560;
    }

    .content {
        background: #16213e;
        border-radius: 8px;
        padding: 1rem;
        overflow: auto;
    }

    .table-container {
        overflow-x: auto;
    }

    table {
        width: 100%;
        border-collapse: collapse;
        font-size: 0.85rem;
    }

    th,
    td {
        padding: 0.5rem;
        text-align: left;
        border-bottom: 1px solid #0f3460;
    }

    th {
        background: #0f3460;
        color: #aaa;
        font-weight: 500;
        position: sticky;
        top: 0;
    }

    td {
        font-family: monospace;
        max-width: 300px;
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
    }

    tr:hover td {
        background: rgba(15, 52, 96, 0.5);
    }

    .pagination {
        display: flex;
        justify-content: center;
        align-items: center;
        gap: 1rem;
        margin-top: 1rem;
        padding-top: 1rem;
        border-top: 1px solid #0f3460;
    }

    .pagination span {
        color: #aaa;
        font-size: 0.85rem;
    }
</style>
