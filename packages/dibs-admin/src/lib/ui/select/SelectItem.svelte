<script lang="ts">
    import { Select as SelectPrimitive } from "bits-ui";
    import { Check } from "phosphor-svelte";
    import type { Snippet } from "svelte";

    interface Props {
        value: string;
        disabled?: boolean;
        class?: string;
        children?: Snippet;
    }

    let { value, disabled = false, class: className = "", children }: Props = $props();
</script>

<SelectPrimitive.Item {value} {disabled} class="select-item {className}">
    {#snippet child({ props, selected })}
        <div {...props} class="select-item {className}">
            <span class="select-item-indicator">
                {#if selected}
                    <Check size={14} weight="bold" />
                {/if}
            </span>
            {@render children?.()}
        </div>
    {/snippet}
</SelectPrimitive.Item>

<style>
    :global(.select-item) {
        display: flex;
        align-items: center;
        gap: 0.5rem;
        padding: 0.375rem 0.5rem 0.375rem 1.75rem;
        border-radius: var(--radius-sm, 0.25rem);
        font-size: 0.875rem;
        cursor: pointer;
        outline: none;
        position: relative;
        user-select: none;
    }

    :global(.select-item:focus),
    :global(.select-item[data-highlighted]) {
        background-color: var(--accent);
        color: var(--accent-foreground);
    }

    :global(.select-item[data-disabled]) {
        pointer-events: none;
        opacity: 0.5;
    }

    :global(.select-item-indicator) {
        position: absolute;
        left: 0.5rem;
        display: flex;
        align-items: center;
        justify-content: center;
        width: 1rem;
    }
</style>
