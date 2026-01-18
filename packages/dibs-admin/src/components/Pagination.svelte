<script lang="ts">
    import { CaretLeft, CaretRight } from "phosphor-svelte";
    import { Button } from "../lib/components/ui/index.js";

    interface Props {
        offset: number;
        limit: number;
        rowCount: number;
        total: bigint | null;
        onPrev: () => void;
        onNext: () => void;
    }

    let { offset, limit, rowCount, total, onPrev, onNext }: Props = $props();

    let hasPrev = $derived(offset > 0);
    let hasNext = $derived(rowCount >= limit);
    let start = $derived(offset + 1);
    let end = $derived(offset + rowCount);
</script>

<div class="flex justify-center items-center gap-6 mt-8 pt-6">
    <Button variant="ghost" size="sm" onclick={onPrev} disabled={!hasPrev}>
        <CaretLeft size={14} />
        Prev
    </Button>
    <span class="text-neutral-500 text-sm">
        {start}–{end}
        {#if total !== null}
            / {total.toString()}
        {/if}
    </span>
    <Button variant="ghost" size="sm" onclick={onNext} disabled={!hasNext}>
        Next
        <CaretRight size={14} />
    </Button>
</div>
