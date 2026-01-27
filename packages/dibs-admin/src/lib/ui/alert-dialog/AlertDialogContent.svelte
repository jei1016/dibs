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

<AlertDialogPrimitive.Portal>
	<AlertDialogPrimitive.Overlay class="alert-dialog-overlay" />
	<AlertDialogPrimitive.Content class="alert-dialog-content {className}">
		{@render children?.()}
	</AlertDialogPrimitive.Content>
</AlertDialogPrimitive.Portal>

<style>
	:global(.alert-dialog-overlay) {
		position: fixed;
		inset: 0;
		z-index: 50;
		background-color: rgb(0 0 0 / 0.5);
	}

	:global(.alert-dialog-overlay[data-state="open"]) {
		animation: fade-in 0.15s ease-out;
	}

	:global(.alert-dialog-overlay[data-state="closed"]) {
		animation: fade-out 0.15s ease-in;
	}

	:global(.alert-dialog-content) {
		position: fixed;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		z-index: 50;
		display: grid;
		width: 100%;
		max-width: calc(100% - 2rem);
		gap: 1rem;
		border-radius: var(--radius-lg, 0.5rem);
		border: 1px solid var(--border);
		background-color: var(--background);
		padding: 1.5rem;
		box-shadow: 0 25px 50px -12px rgb(0 0 0 / 0.25);
	}

	@media (min-width: 640px) {
		:global(.alert-dialog-content) {
			max-width: 32rem;
		}
	}

	:global(.alert-dialog-content[data-state="open"]) {
		animation: dialog-in 0.2s ease-out;
	}

	:global(.alert-dialog-content[data-state="closed"]) {
		animation: dialog-out 0.15s ease-in;
	}

	@keyframes fade-in {
		from { opacity: 0; }
		to { opacity: 1; }
	}

	@keyframes fade-out {
		from { opacity: 1; }
		to { opacity: 0; }
	}

	@keyframes dialog-in {
		from {
			opacity: 0;
			transform: translate(-50%, -50%) scale(0.95);
		}
		to {
			opacity: 1;
			transform: translate(-50%, -50%) scale(1);
		}
	}

	@keyframes dialog-out {
		from {
			opacity: 1;
			transform: translate(-50%, -50%) scale(1);
		}
		to {
			opacity: 0;
			transform: translate(-50%, -50%) scale(0.95);
		}
	}
</style>
