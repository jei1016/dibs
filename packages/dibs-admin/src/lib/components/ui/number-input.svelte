<script lang="ts">
    import { cn } from "$lib/utils.js";
    import type { HTMLInputAttributes } from "svelte/elements";

    interface Props extends Omit<HTMLInputAttributes, "type"> {
        class?: string;
    }

    let { class: className, ...rest }: Props = $props();
</script>

<input
    type="number"
    class={cn(
        "flex h-9 w-full bg-input px-3 py-1 text-sm text-foreground placeholder:text-muted-foreground focus-visible:outline-none focus-visible:bg-accent disabled:cursor-not-allowed disabled:opacity-40 [appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none",
        className,
    )}
    {...rest}
/>
