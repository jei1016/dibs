<script lang="ts" module>
    import { tv, type VariantProps } from "tailwind-variants";

    export const buttonVariants = tv({
        base: "inline-flex items-center justify-center gap-2 whitespace-nowrap text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-neutral-400 disabled:pointer-events-none disabled:opacity-40",
        variants: {
            variant: {
                default: "bg-white text-neutral-900 hover:bg-neutral-200",
                destructive: "text-red-400 hover:text-red-300 hover:bg-neutral-800",
                outline: "border border-neutral-700 text-neutral-300 hover:bg-neutral-800 hover:text-white",
                secondary: "bg-neutral-800 text-neutral-200 hover:bg-neutral-700",
                ghost: "text-neutral-400 hover:text-white hover:bg-neutral-800",
                link: "text-neutral-400 underline-offset-4 hover:underline hover:text-white",
            },
            size: {
                default: "h-9 px-4 py-2",
                sm: "h-8 px-3 text-xs",
                lg: "h-10 px-8",
                icon: "h-9 w-9",
            },
        },
        defaultVariants: {
            variant: "default",
            size: "default",
        },
    });

    export type ButtonVariant = VariantProps<typeof buttonVariants>["variant"];
    export type ButtonSize = VariantProps<typeof buttonVariants>["size"];
</script>

<script lang="ts">
    import { cn } from "../../utils.js";
    import type { HTMLButtonAttributes } from "svelte/elements";

    interface Props extends HTMLButtonAttributes {
        variant?: ButtonVariant;
        size?: ButtonSize;
        class?: string;
    }

    let {
        variant = "default",
        size = "default",
        class: className,
        children,
        ...rest
    }: Props = $props();
</script>

<button class={cn(buttonVariants({ variant, size }), className)} {...rest}>
    {@render children?.()}
</button>
