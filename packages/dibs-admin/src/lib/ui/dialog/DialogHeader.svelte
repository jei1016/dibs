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

<div class="dialog-header {className}">
	{@render children?.()}
</div>

<style>
	.dialog-header {
		display: flex;
		flex-direction: column;
		gap: 0.375rem;
		text-align: center;
	}

	@media (min-width: 640px) {
		.dialog-header {
			text-align: left;
		}
	}
</style>
