<script lang="ts">
    import { cn } from "../../utils.js";
    import type { HTMLLabelAttributes } from "svelte/elements";

    interface Props extends HTMLLabelAttributes {
        class?: string;
    }

    let { class: className, children, ...rest }: Props = $props();
</script>

<label
    class={cn(
        "text-sm font-medium text-neutral-300 leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-40",
        className,
    )}
    {...rest}
>
    {@render children?.()}
</label>
