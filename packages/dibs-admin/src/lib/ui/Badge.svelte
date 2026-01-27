<script lang="ts">
	import type { Snippet } from "svelte";

	type BadgeVariant = "default" | "secondary" | "destructive" | "outline";

	interface Props {
		variant?: BadgeVariant;
		class?: string;
		children?: Snippet;
	}

	let {
		variant = "default",
		class: className = "",
		children,
	}: Props = $props();
</script>

<span class="badge badge-{variant} {className}">
	{@render children?.()}
</span>

<style>
	.badge {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		border-radius: 9999px;
		padding: 0.125rem 0.625rem;
		font-size: 0.75rem;
		font-weight: 600;
		line-height: 1.25rem;
		transition: background-color 0.15s, color 0.15s;
		border: 1px solid transparent;
	}

	.badge-default {
		background-color: var(--primary);
		color: var(--primary-foreground);
	}

	.badge-secondary {
		background-color: var(--secondary);
		color: var(--secondary-foreground);
	}

	.badge-destructive {
		background-color: var(--destructive);
		color: var(--destructive-foreground);
	}

	.badge-outline {
		background-color: transparent;
		border-color: var(--border);
		color: var(--foreground);
	}
</style>
