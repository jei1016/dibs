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

<AlertDialogPrimitive.Description class="alert-dialog-description {className}">
	{@render children?.()}
</AlertDialogPrimitive.Description>

<style>
	:global(.alert-dialog-description) {
		font-size: 0.875rem;
		color: var(--muted-foreground);
	}
</style>
