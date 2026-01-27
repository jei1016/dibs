<script lang="ts">
	import { AlertDialog as AlertDialogPrimitive } from "bits-ui";
	import type { Snippet } from "svelte";

	interface Props extends AlertDialogPrimitive.TriggerProps {
		class?: string;
		children?: Snippet;
	}

	let {
		class: className = "",
		children,
		...restProps
	}: Props = $props();
</script>

<AlertDialogPrimitive.Trigger class={className} {...restProps}>
	{@render children?.()}
</AlertDialogPrimitive.Trigger>
