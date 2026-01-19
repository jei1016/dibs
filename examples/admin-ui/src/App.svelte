<script lang="ts">
    import { DibsAdmin, type DibsAdminConfig } from "@bearcove/dibs-admin";
    import { connect, getClient } from "./lib/roam";
    import "./app.css";

    // Connection state
    let connected = $state(false);
    let connecting = $state(false);
    let error = $state<string | null>(null);

    // Database URL - matches my-app-db/.env default
    const DATABASE_URL = "postgres://localhost/dibs_test";

    // Admin configuration
    const config: DibsAdminConfig = {
        dashboard: {
            title: "Blog Admin",
            tiles: [
                { type: "latest", table: "post", title: "Recent Posts", limit: 5 },
                { type: "latest", table: "user", title: "New Users", limit: 5 },
                { type: "latest", table: "comment", title: "Recent Comments", limit: 5 },
                { type: "count", table: "post", title: "Total Posts", icon: "article" },
                { type: "count", table: "user", title: "Total Users", icon: "users" },
                { type: "count", table: "comment", title: "Total Comments", icon: "chat-circle" },
                {
                    type: "links",
                    title: "Quick Links",
                    links: [
                        { label: "Published Posts", table: "post" },
                        { label: "All Categories", table: "category" },
                        { label: "All Tags", table: "tag" },
                    ],
                },
            ],
        },

        tables: {
            user: {
                label: "Users",
                list: {
                    columns: ["id", "avatar_url", "name", "email", "is_admin", "created_at"],
                    defaultSort: { field: "created_at", direction: "desc" },
                    imageColumns: ["avatar_url"],
                },
                detail: {
                    fields: [
                        { title: "Profile", fields: ["name", "email", "bio", "avatar_url"] },
                        { title: "Settings", fields: ["is_admin", "last_login_at"], collapsed: true },
                        { title: "Metadata", fields: ["id", "created_at"], collapsed: true },
                    ],
                    readOnly: ["id", "created_at"],
                },
                relations: [
                    { table: "post", via: "author_id", label: "Posts", limit: 10 },
                    { table: "comment", via: "author_id", label: "Comments", limit: 10 },
                ],
            },

            post: {
                label: "Posts",
                list: {
                    columns: ["id", "title", "author_id", "published", "view_count", "created_at"],
                    defaultSort: { field: "created_at", direction: "desc" },
                    pageSize: 20,
                    rowExpand: {
                        field: "excerpt",
                        render: "text",
                        previewLines: 2,
                    },
                },
                detail: {
                    fields: [
                        { title: "Content", fields: ["title", "slug", "excerpt", "body"] },
                        { title: "Publishing", fields: ["published", "published_at", "category_id", "featured_image_url"] },
                        { title: "Stats", fields: ["view_count", "author_id"], collapsed: true },
                        { title: "Timestamps", fields: ["created_at", "updated_at"], collapsed: true },
                    ],
                    readOnly: ["id", "created_at", "updated_at", "view_count"],
                },
                relations: [
                    { table: "comment", via: "post_id", label: "Comments", limit: 20 },
                ],
            },

            comment: {
                label: "Comments",
                list: {
                    columns: ["id", "post_id", "author_id", "is_approved", "created_at"],
                    defaultSort: { field: "created_at", direction: "desc" },
                    rowExpand: {
                        field: "body",
                        render: "markdown",
                        previewLines: 2,
                    },
                },
                detail: {
                    readOnly: ["id", "created_at"],
                },
            },

            category: {
                label: "Categories",
                list: {
                    columns: ["id", "name", "slug", "parent_id", "sort_order"],
                    defaultSort: { field: "sort_order", direction: "asc" },
                },
                detail: {
                    readOnly: ["id"],
                },
            },

            tag: {
                label: "Tags",
                list: {
                    columns: ["id", "name", "slug", "color"],
                    defaultSort: { field: "name", direction: "asc" },
                },
                detail: {
                    readOnly: ["id"],
                },
            },

            // Hide junction tables from sidebar
            post_tag: { hidden: true },
            post_like: { hidden: true },
            user_follow: { hidden: true },
        },

        defaults: {
            pageSize: 25,
            relationLimit: 10,
        },
    };

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

<main class="min-h-screen flex flex-col bg-background text-foreground">
    <header class="px-6 py-4 border-b border-border flex items-center gap-3">
        <h1 class="text-sm font-medium uppercase tracking-widest text-muted-foreground">dibs admin</h1>
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
                <p class="text-destructive text-sm mb-4">
                    {error}
                </p>
            {/if}
            <button
                class="bg-primary text-primary-foreground hover:bg-primary/90 disabled:opacity-40 disabled:cursor-not-allowed px-6 py-3 text-sm font-medium transition-colors rounded-md"
                onclick={handleConnect}
            >
                Retry connection
            </button>
        </div>
    {:else if connecting}
        <div class="flex-1 flex items-center justify-center text-muted-foreground">
            Connecting...
        </div>
    {:else}
        {@const client = getClient()}
        {#if client}
            <div class="flex-1 min-h-0">
                <DibsAdmin {client} databaseUrl={DATABASE_URL} {config} />
            </div>
        {/if}
    {/if}
</main>
