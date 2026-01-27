<script lang="ts">
	import { Dialog as DialogPrimitive } from "bits-ui";
	import type { Snippet } from "svelte";

	interface Props {
		class?: string;
		children?: Snippet;
	}

	let {
		class: className = "",
		children,
	}: Props = $props();
</script>

<DialogPrimitive.Title class="dialog-title {className}">
	{@render children?.()}
</DialogPrimitive.Title>

<style>
	:global(.dialog-title) {
		font-size: 1.125rem;
		font-weight: 600;
		line-height: 1.25;
		letter-spacing: -0.025em;
		color: var(--foreground);
	}
</style>
