<script lang="ts">
    import { ArrowRight } from "phosphor-svelte";
    import type { QuickLinksTile } from "../../types/config.js";
    import { Card } from "../../lib/components/ui/index.js";

    interface Props {
        config: QuickLinksTile;
        onSelectTable: (tableName: string) => void;
    }

    let { config, onSelectTable }: Props = $props();

    let title = $derived(config.title ?? "Quick Links");
</script>

<Card.Root>
    <Card.Header class="pb-3">
        <Card.Title class="text-sm font-medium">{title}</Card.Title>
    </Card.Header>
    <Card.Content class="pt-0">
        <ul class="space-y-2">
            {#each config.links as link}
                <li>
                    <button
                        class="w-full flex items-center justify-between text-sm text-left hover:text-primary transition-colors"
                        onclick={() => onSelectTable(link.table)}
                    >
                        <span>{link.label}</span>
                        <ArrowRight size={14} class="text-muted-foreground" />
                    </button>
                </li>
            {/each}
        </ul>
    </Card.Content>
</Card.Root>
