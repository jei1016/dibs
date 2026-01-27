<script lang="ts">
	import { AlertDialog as AlertDialogPrimitive } from "bits-ui";
	import type { Snippet } from "svelte";

	interface Props extends AlertDialogPrimitive.ActionProps {
		class?: string;
		children?: Snippet;
	}

	let {
		class: className = "",
		children,
		...restProps
	}: Props = $props();
</script>

<AlertDialogPrimitive.Action class="alert-dialog-action {className}" {...restProps}>
	{@render children?.()}
</AlertDialogPrimitive.Action>

<style>
	:global(.alert-dialog-action) {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		height: 2.25rem;
		padding: 0.5rem 1rem;
		border-radius: var(--radius-md, 0.375rem);
		font-size: 0.875rem;
		font-weight: 500;
		background-color: var(--primary);
		color: var(--primary-foreground);
		border: none;
		cursor: pointer;
		transition: background-color 0.15s;
	}

	:global(.alert-dialog-action:hover) {
		background-color: oklch(from var(--primary) l c h / 0.9);
	}

	:global(.alert-dialog-action:focus-visible) {
		outline: 2px solid var(--ring);
		outline-offset: 2px;
	}
</style>
