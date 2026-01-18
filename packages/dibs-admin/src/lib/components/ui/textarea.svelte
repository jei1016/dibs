<script lang="ts">
    import { cn } from "../../utils.js";
    import type { HTMLTextareaAttributes } from "svelte/elements";

    interface Props extends HTMLTextareaAttributes {
        class?: string;
    }

    let { class: className, ...rest }: Props = $props();
</script>

<textarea
    class={cn(
        "flex min-h-[80px] w-full bg-neutral-900 px-3 py-2 text-sm text-white placeholder:text-neutral-600 focus-visible:outline-none focus-visible:bg-neutral-800 disabled:cursor-not-allowed disabled:opacity-40",
        className,
    )}
    {...rest}
></textarea>
