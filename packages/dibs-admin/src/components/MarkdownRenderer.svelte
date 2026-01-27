<script lang="ts">
    interface Props {
        content: string;
    }

    let { content }: Props = $props();

    // Simple markdown-like formatting (basic support)
    function formatContent(text: string): string {
        return (
            text
                // Headers
                .replace(/^### (.+)$/gm, '<h4 class="md-h4">$1</h4>')
                .replace(/^## (.+)$/gm, '<h3 class="md-h3">$1</h3>')
                .replace(/^# (.+)$/gm, '<h2 class="md-h2">$1</h2>')
                // Bold and italic
                .replace(/\*\*(.+?)\*\*/g, "<strong>$1</strong>")
                .replace(/\*(.+?)\*/g, "<em>$1</em>")
                .replace(/__(.+?)__/g, "<strong>$1</strong>")
                .replace(/_(.+?)_/g, "<em>$1</em>")
                // Inline code
                .replace(/`([^`]+)`/g, '<code class="md-code">$1</code>')
                // Links
                .replace(
                    /\[([^\]]+)\]\(([^)]+)\)/g,
                    '<a href="$2" class="md-link" target="_blank" rel="noopener">$1</a>',
                )
                // Line breaks
                .replace(/\n/g, "<br />")
        );
    }
</script>

<div class="markdown-content">
    {@html formatContent(content)}
</div>

<style>
    .markdown-content :global(.md-h2) {
        font-weight: 700;
        font-size: 1.25rem;
        margin-top: 1rem;
        margin-bottom: 0.5rem;
    }

    .markdown-content :global(.md-h3) {
        font-weight: 600;
        font-size: 1.125rem;
        margin-top: 1rem;
        margin-bottom: 0.5rem;
    }

    .markdown-content :global(.md-h4) {
        font-weight: 600;
        margin-top: 0.75rem;
        margin-bottom: 0.25rem;
    }

    .markdown-content :global(.md-code) {
        background-color: var(--muted);
        padding: 0.125rem 0.25rem;
        border-radius: var(--radius-sm);
        font-size: 0.875rem;
        font-family: ui-monospace, monospace;
    }

    .markdown-content :global(.md-link) {
        color: var(--primary);
        text-decoration: underline;
    }
</style>
