<script lang="ts">
    import { DibsAdmin } from "@bearcove/dibs-admin";
    import { connect, getClient } from "./lib/roam";
    import "./app.css";

    // Connection state
    let connected = $state(false);
    let connecting = $state(false);
    let error = $state<string | null>(null);

    // Database URL - matches my-app-db/.env default
    const DATABASE_URL = "postgres://localhost/dibs_test";

    async function handleConnect() {
        connecting = true;
        error = null;
        try {
            await connect();
            connected = true;
        } catch (e) {
            error = e instanceof Error ? e.message : String(e);
        } finally {
            connecting = false;
        }
    }

    // Auto-connect on mount
    $effect(() => {
        handleConnect();
    });
</script>

<svelte:head>
    <style>
        body {
            margin: 0;
        }
    </style>
</svelte:head>

<main class="min-h-screen flex flex-col bg-neutral-950 text-neutral-100">
    <header class="px-6 py-4 border-b border-neutral-900 flex items-center gap-3">
        <h1 class="text-sm font-medium uppercase tracking-widest text-neutral-400">dibs admin</h1>
        {#if connected}
            <span class="w-2 h-2 bg-green-500"></span>
        {:else if connecting}
            <span class="w-2 h-2 bg-yellow-500 animate-pulse"></span>
        {:else if error}
            <span class="w-2 h-2 bg-red-500"></span>
        {/if}
    </header>

    {#if !connected && !connecting}
        <div class="flex-1 flex flex-col items-center justify-center gap-6 p-8">
            {#if error}
                <p class="text-red-400 text-sm mb-4">
                    {error}
                </p>
            {/if}
            <button
                class="bg-white text-neutral-900 hover:bg-neutral-200 disabled:opacity-40 disabled:cursor-not-allowed px-6 py-3 text-sm font-medium transition-colors"
                onclick={handleConnect}
            >
                Retry connection
            </button>
        </div>
    {:else if connecting}
        <div class="flex-1 flex items-center justify-center text-neutral-500">
            Connecting...
        </div>
    {:else}
        {@const client = getClient()}
        {#if client}
            <div class="flex-1 min-h-0">
                <DibsAdmin {client} databaseUrl={DATABASE_URL} />
            </div>
        {/if}
    {/if}
</main>
