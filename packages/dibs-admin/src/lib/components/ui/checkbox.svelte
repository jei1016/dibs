<script lang="ts">
    import { cn } from "../../utils.js";
    import { Check } from "phosphor-svelte";

    interface Props {
        checked?: boolean;
        onchange?: (checked: boolean) => void;
        disabled?: boolean;
        class?: string;
        id?: string;
    }

    let { checked = false, onchange, disabled = false, class: className, id }: Props = $props();

    function handleClick() {
        if (!disabled) {
            onchange?.(!checked);
        }
    }

    function handleKeydown(e: KeyboardEvent) {
        if (e.key === " " || e.key === "Enter") {
            e.preventDefault();
            handleClick();
        }
    }
</script>

<button
    type="button"
    role="checkbox"
    aria-checked={checked}
    {disabled}
    {id}
    class={cn(
        "peer h-5 w-5 shrink-0 bg-neutral-800 focus-visible:outline-none focus-visible:bg-neutral-700 disabled:cursor-not-allowed disabled:opacity-40",
        checked && "bg-white",
        className,
    )}
    onclick={handleClick}
    onkeydown={handleKeydown}
>
    {#if checked}
        <Check class="h-4 w-4 text-neutral-900 mx-auto" weight="bold" />
    {/if}
</button>
