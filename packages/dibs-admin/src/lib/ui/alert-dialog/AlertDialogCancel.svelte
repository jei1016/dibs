<script lang="ts">
	import { AlertDialog as AlertDialogPrimitive } from "bits-ui";
	import type { Snippet } from "svelte";

	interface Props extends AlertDialogPrimitive.CancelProps {
		class?: string;
		children?: Snippet;
	}

	let {
		class: className = "",
		children,
		...restProps
	}: Props = $props();
</script>

<AlertDialogPrimitive.Cancel class="alert-dialog-cancel {className}" {...restProps}>
	{@render children?.()}
</AlertDialogPrimitive.Cancel>

<style>
	:global(.alert-dialog-cancel) {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		height: 2.25rem;
		padding: 0.5rem 1rem;
		border-radius: var(--radius-md, 0.375rem);
		font-size: 0.875rem;
		font-weight: 500;
		background-color: var(--secondary);
		color: var(--secondary-foreground);
		border: 1px solid var(--border);
		cursor: pointer;
		transition: background-color 0.15s;
	}

	:global(.alert-dialog-cancel:hover) {
		background-color: oklch(from var(--secondary) l c h / 0.8);
	}

	:global(.alert-dialog-cancel:focus-visible) {
		outline: 2px solid var(--ring);
		outline-offset: 2px;
	}
</style>
