<script lang="ts">
    import { Tooltip as TooltipPrimitive } from "bits-ui";
    import type { Snippet } from "svelte";

    interface Props {
        class?: string;
        children?: Snippet<[{ props: Record<string, unknown> }]>;
    }

    let { class: className = "", children }: Props = $props();
</script>

<TooltipPrimitive.Trigger class={className}>
    {#snippet child({ props })}
        {@render children?.({ props })}
    {/snippet}
</TooltipPrimitive.Trigger>
