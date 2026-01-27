<script lang="ts">
	import type { HTMLAnchorAttributes, HTMLButtonAttributes } from "svelte/elements";
	import type { Snippet } from "svelte";

	type ButtonVariant = "default" | "secondary" | "outline" | "ghost" | "destructive" | "link";
	type ButtonSize = "default" | "sm" | "lg" | "icon" | "icon-sm" | "icon-lg";

	interface Props {
		variant?: ButtonVariant;
		size?: ButtonSize;
		href?: string;
		disabled?: boolean;
		type?: "button" | "submit" | "reset";
		class?: string;
		children?: Snippet;
		onclick?: (e: MouseEvent) => void;
	}

	let {
		variant = "default",
		size = "default",
		href,
		disabled = false,
		type = "button",
		class: className = "",
		children,
		onclick,
		...restProps
	}: Props & Partial<HTMLButtonAttributes> & Partial<HTMLAnchorAttributes> = $props();
</script>

{#if href && !disabled}
	<a
		{href}
		class="btn btn-{variant} btn-size-{size} {className}"
		{...restProps}
	>
		{@render children?.()}
	</a>
{:else}
	<button
		{type}
		{disabled}
		class="btn btn-{variant} btn-size-{size} {className}"
		{onclick}
		{...restProps}
	>
		{@render children?.()}
	</button>
{/if}

<style>
	.btn {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		border-radius: var(--radius-md, 0.375rem);
		font-size: 0.875rem;
		font-weight: 500;
		white-space: nowrap;
		transition: background-color 0.15s, color 0.15s, border-color 0.15s, opacity 0.15s;
		outline: none;
		cursor: pointer;
		border: 1px solid transparent;
		text-decoration: none;
	}

	.btn:focus-visible {
		outline: 2px solid var(--ring);
		outline-offset: 2px;
	}

	.btn:disabled,
	.btn[aria-disabled="true"] {
		pointer-events: none;
		opacity: 0.5;
	}

	/* Variants */
	.btn-default {
		background-color: var(--primary);
		color: var(--primary-foreground);
	}
	.btn-default:hover {
		background-color: oklch(from var(--primary) l c h / 0.9);
	}

	.btn-secondary {
		background-color: var(--secondary);
		color: var(--secondary-foreground);
	}
	.btn-secondary:hover {
		background-color: oklch(from var(--secondary) l c h / 0.8);
	}

	.btn-outline {
		background-color: var(--background);
		border-color: var(--border);
		color: var(--foreground);
	}
	.btn-outline:hover {
		background-color: var(--accent);
		color: var(--accent-foreground);
	}

	.btn-ghost {
		background-color: transparent;
		color: var(--foreground);
	}
	.btn-ghost:hover {
		background-color: var(--accent);
		color: var(--accent-foreground);
	}

	.btn-destructive {
		background-color: var(--destructive);
		color: var(--destructive-foreground);
	}
	.btn-destructive:hover {
		background-color: oklch(from var(--destructive) l c h / 0.9);
	}

	.btn-link {
		background-color: transparent;
		color: var(--primary);
		text-decoration: underline;
		text-underline-offset: 4px;
	}
	.btn-link:hover {
		text-decoration-thickness: 2px;
	}

	/* Sizes */
	.btn-size-default {
		height: 2.25rem;
		padding: 0.5rem 1rem;
	}
	.btn-size-default:has(> :global(svg:only-child)) {
		padding: 0.5rem 0.75rem;
	}

	.btn-size-sm {
		height: 2rem;
		padding: 0.375rem 0.75rem;
		gap: 0.375rem;
	}

	.btn-size-lg {
		height: 2.5rem;
		padding: 0.5rem 1.5rem;
	}

	.btn-size-icon {
		width: 2.25rem;
		height: 2.25rem;
		padding: 0;
	}

	.btn-size-icon-sm {
		width: 2rem;
		height: 2rem;
		padding: 0;
	}

	.btn-size-icon-lg {
		width: 2.5rem;
		height: 2.5rem;
		padding: 0;
	}

	/* SVG sizing */
	.btn :global(svg) {
		flex-shrink: 0;
		width: 1rem;
		height: 1rem;
	}
</style>
