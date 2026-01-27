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

<div class="alert-dialog-header {className}">
	{@render children?.()}
</div>

<style>
	.alert-dialog-header {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
		text-align: center;
	}

	@media (min-width: 640px) {
		.alert-dialog-header {
			text-align: left;
		}
	}
</style>
