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

<h3 class="card-title {className}">
	{@render children?.()}
</h3>

<style>
	.card-title {
		font-size: 1.125rem;
		font-weight: 600;
		line-height: 1.25;
		letter-spacing: -0.025em;
	}
</style>
