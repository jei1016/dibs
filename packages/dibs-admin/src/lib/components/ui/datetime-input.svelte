<script lang="ts">
    import { cn } from "../../utils.js";

    interface Props {
        value?: string;
        id?: string;
        disabled?: boolean;
        class?: string;
        onchange?: (value: string) => void;
    }

    let { value = $bindable(""), id, disabled = false, class: className, onchange }: Props = $props();

    // Convert PostgreSQL timestamp to datetime-local format
    // Input: "2026-01-18 15:31:19.006542+01" or ISO format
    // Output: "2026-01-18T15:31:19" (for datetime-local input)
    function toDatetimeLocal(pg: string): string {
        if (!pg) return "";
        // Remove timezone and microseconds, replace space with T
        const cleaned = pg
            .replace(/\.\d+/, "") // remove microseconds
            .replace(/[+-]\d{2}$/, "") // remove timezone offset like +01
            .replace(" ", "T"); // space to T for ISO
        return cleaned;
    }

    // Convert datetime-local back to a format we can send
    // Input: "2026-01-18T15:31:19"
    // Output: "2026-01-18 15:31:19" (PostgreSQL-friendly)
    function fromDatetimeLocal(dt: string): string {
        if (!dt) return "";
        return dt.replace("T", " ");
    }

    let localValue = $derived(toDatetimeLocal(value));

    function handleInput(e: Event) {
        const target = e.target as HTMLInputElement;
        const newValue = fromDatetimeLocal(target.value);
        value = newValue;
        onchange?.(newValue);
    }
</script>

<input
    type="datetime-local"
    {id}
    value={localValue}
    oninput={handleInput}
    {disabled}
    step="1"
    class={cn(
        "flex h-9 w-full bg-neutral-900 px-3 py-1 text-sm text-white focus-visible:outline-none focus-visible:bg-neutral-800 disabled:cursor-not-allowed disabled:opacity-40 [color-scheme:dark]",
        className,
    )}
/>
