<script lang="ts">
	import { Tooltip as TooltipPrimitive } from "bits-ui";
	import type { Snippet } from "svelte";

	interface Props {
		children?: Snippet;
		delayDuration?: number;
		disableHoverableContent?: boolean;
	}

	let {
		children,
		delayDuration = 200,
		disableHoverableContent = false,
	}: Props = $props();
</script>

<TooltipPrimitive.Provider {delayDuration} {disableHoverableContent}>
	{@render children?.()}
</TooltipPrimitive.Provider>
