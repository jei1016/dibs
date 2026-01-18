<script lang="ts">
    import { cn } from "../../utils.js";
    import type { HTMLInputAttributes } from "svelte/elements";

    interface Props extends HTMLInputAttributes {
        class?: string;
        value?: string;
    }

    let { class: className, value = $bindable(""), ...rest }: Props = $props();
</script>

<input
    bind:value
    class={cn(
        "flex h-9 w-full bg-neutral-900 px-3 py-1 text-sm text-white placeholder:text-neutral-600 focus-visible:outline-none focus-visible:bg-neutral-800 disabled:cursor-not-allowed disabled:opacity-40",
        className,
    )}
    {...rest}
/>
