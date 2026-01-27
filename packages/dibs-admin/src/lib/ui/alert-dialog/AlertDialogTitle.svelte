<script lang="ts">
	import { AlertDialog as AlertDialogPrimitive } from "bits-ui";
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

<AlertDialogPrimitive.Title class="alert-dialog-title {className}">
	{@render children?.()}
</AlertDialogPrimitive.Title>

<style>
	:global(.alert-dialog-title) {
		font-size: 1.125rem;
		font-weight: 600;
		line-height: 1.25;
		color: var(--foreground);
	}
</style>
