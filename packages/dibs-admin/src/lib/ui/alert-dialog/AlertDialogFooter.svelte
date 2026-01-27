<script lang="ts">
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

<div class="alert-dialog-footer {className}">
	{@render children?.()}
</div>

<style>
	.alert-dialog-footer {
		display: flex;
		flex-direction: column-reverse;
		gap: 0.5rem;
	}

	@media (min-width: 640px) {
		.alert-dialog-footer {
			flex-direction: row;
			justify-content: flex-end;
		}
	}
</style>
