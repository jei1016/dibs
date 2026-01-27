<script lang="ts">
    import { Tooltip as TooltipPrimitive } from "bits-ui";
    import type { Snippet } from "svelte";

    interface Props {
        open?: boolean;
        children?: Snippet;
    }

    let { open = $bindable(false), children }: Props = $props();
</script>

<TooltipPrimitive.Provider>
    <TooltipPrimitive.Root bind:open>
        {@render children?.()}
    </TooltipPrimitive.Root>
</TooltipPrimitive.Provider>
