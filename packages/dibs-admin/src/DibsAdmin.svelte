<script lang="ts">
    import { Plus } from "phosphor-svelte";
    import type {
        SquelClient,
        SchemaInfo,
        TableInfo,
        ColumnInfo,
        Row,
        Filter,
        Sort,
        Value,
        ListRequest,
    } from "./types.js";
    import TableList from "./components/TableList.svelte";
    import DataTable from "./components/DataTable.svelte";
    import FilterBar from "./components/FilterBar.svelte";
    import Pagination from "./components/Pagination.svelte";
    import RowEditor from "./components/RowEditor.svelte";
    import { Button } from "./lib/components/ui/index.js";

    interface Props {
        client: SquelClient;
        databaseUrl: string;
    }

    let { client, databaseUrl }: Props = $props();

    // Schema state
    let schema = $state<SchemaInfo | null>(null);
    let selectedTable = $state<string | null>(null);
    let loading = $state(false);
    let error = $state<string | null>(null);

    // Data state
    let rows = $state<Row[]>([]);
    let totalRows = $state<bigint | null>(null);

    // Query state
    let filters = $state<Filter[]>([]);
    let sort = $state<Sort | null>(null);
    let limit = $state(25);
    let offset = $state(0);

    // Editor state
    let editingRow = $state<Row | null>(null);
    let isCreating = $state(false);
    let saving = $state(false);
    let deleting = $state(false);

    // Derived
    let currentTable = $derived(schema?.tables.find((t) => t.name === selectedTable) ?? null);
    let columns = $derived(currentTable?.columns ?? []);

    // Load schema on mount
    $effect(() => {
        loadSchema();
    });

    async function loadSchema() {
        loading = true;
        error = null;
        try {
            schema = await client.schema();
            if (schema.tables.length > 0) {
                selectTable(schema.tables[0].name);
            }
        } catch (e) {
            error = e instanceof Error ? e.message : String(e);
        } finally {
            loading = false;
        }
    }

    async function loadData() {
        if (!selectedTable) return;

        loading = true;
        error = null;
        try {
            const request: ListRequest = {
                database_url: databaseUrl,
                table: selectedTable,
                filters,
                sort: sort ? [sort] : [],
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

    function formatError(err: { tag: string; value: string }): string {
        return `${err.tag}: ${err.value}`;
    }

    function selectTable(tableName: string) {
        selectedTable = tableName;
        filters = [];
        sort = null;
        offset = 0;
        loadData();
    }

    function handleSort(column: string) {
        if (sort && sort.field === column) {
            // Toggle direction
            sort = {
                field: column,
                dir: sort.dir.tag === "Asc" ? { tag: "Desc" } : { tag: "Asc" },
            };
        } else {
            sort = { field: column, dir: { tag: "Asc" } };
        }
        offset = 0;
        loadData();
    }

    function addFilter(filter: Filter) {
        filters = [...filters, filter];
        offset = 0;
        loadData();
    }

    function removeFilter(index: number) {
        filters = filters.filter((_, i) => i !== index);
        offset = 0;
        loadData();
    }

    function clearFilters() {
        filters = [];
        offset = 0;
        loadData();
    }

    function nextPage() {
        offset += limit;
        loadData();
    }

    function prevPage() {
        offset = Math.max(0, offset - limit);
        loadData();
    }

    function openEditor(row: Row) {
        editingRow = row;
        isCreating = false;
    }

    function openCreateDialog() {
        editingRow = null;
        isCreating = true;
    }

    function closeEditor() {
        editingRow = null;
        isCreating = false;
    }

    function getPrimaryKeyValue(row: Row): Value | null {
        if (!currentTable) return null;
        const pkCol = currentTable.columns.find((c) => c.primary_key);
        if (!pkCol) return null;
        const field = row.fields.find((f) => f.name === pkCol.name);
        return field?.value ?? null;
    }

    async function saveRow(data: Row) {
        if (!selectedTable) return;

        saving = true;
        error = null;

        try {
            if (isCreating) {
                const result = await client.create({
                    database_url: databaseUrl,
                    table: selectedTable,
                    data,
                });
                if (!result.ok) {
                    error = formatError(result.error);
                    return;
                }
            } else if (editingRow) {
                const pk = getPrimaryKeyValue(editingRow);
                if (!pk) {
                    error = "Could not determine primary key";
                    return;
                }
                const result = await client.update({
                    database_url: databaseUrl,
                    table: selectedTable,
                    pk,
                    data,
                });
                if (!result.ok) {
                    error = formatError(result.error);
                    return;
                }
            }
            closeEditor();
            loadData();
        } catch (e) {
            error = e instanceof Error ? e.message : String(e);
        } finally {
            saving = false;
        }
    }

    async function deleteRow() {
        if (!selectedTable || !editingRow) return;

        const pk = getPrimaryKeyValue(editingRow);
        if (!pk) {
            error = "Could not determine primary key";
            return;
        }

        deleting = true;
        error = null;

        try {
            const result = await client.delete({
                database_url: databaseUrl,
                table: selectedTable,
                pk,
            });
            if (!result.ok) {
                error = formatError(result.error);
                return;
            }
            closeEditor();
            loadData();
        } catch (e) {
            error = e instanceof Error ? e.message : String(e);
        } finally {
            deleting = false;
        }
    }
</script>

<div class="h-full bg-neutral-950 text-neutral-100">
    {#if loading && !schema}
        <div class="flex items-center justify-center h-full p-8 text-neutral-500">
            Loading schema...
        </div>
    {:else if schema}
        <div class="grid grid-cols-[200px_1fr] h-full">
            <TableList tables={schema.tables} selected={selectedTable} onSelect={selectTable} />

            <section class="p-8 overflow-auto flex flex-col">
                {#if selectedTable && currentTable}
                    <div class="flex justify-between items-center mb-8">
                        <h2 class="text-lg font-medium text-white uppercase tracking-wide">{selectedTable}</h2>
                        <Button onclick={openCreateDialog}>
                            <Plus size={16} />
                            New
                        </Button>
                    </div>

                    {#if error}
                        <p class="text-red-400 mb-6 text-sm">
                            {error}
                        </p>
                    {/if}

                    <FilterBar
                        {columns}
                        {filters}
                        onAddFilter={addFilter}
                        onRemoveFilter={removeFilter}
                        onClearFilters={clearFilters}
                    />

                    {#if loading}
                        <div class="flex-1 flex items-center justify-center text-neutral-500">
                            Loading...
                        </div>
                    {:else if rows.length === 0}
                        <div class="flex-1 flex items-center justify-center text-neutral-600">
                            No rows found.
                        </div>
                    {:else}
                        <DataTable
                            {columns}
                            {rows}
                            {sort}
                            onSort={handleSort}
                            onRowClick={openEditor}
                        />

                        <Pagination
                            {offset}
                            {limit}
                            rowCount={rows.length}
                            total={totalRows}
                            onPrev={prevPage}
                            onNext={nextPage}
                        />
                    {/if}
                {:else}
                    <div class="flex-1 flex items-center justify-center text-neutral-600">
                        Select a table
                    </div>
                {/if}
            </section>
        </div>
    {:else if error}
        <p class="text-red-400 p-8 text-sm">
            {error}
        </p>
    {/if}

    {#if editingRow || isCreating}
        <RowEditor
            {columns}
            row={editingRow}
            onSave={saveRow}
            onDelete={editingRow ? deleteRow : undefined}
            onClose={closeEditor}
            {saving}
            {deleting}
        />
    {/if}
</div>
