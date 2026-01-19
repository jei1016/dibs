<script lang="ts">
    interface Props {
        content: string;
    }

    let { content }: Props = $props();

    // Simple markdown-like formatting (basic support)
    // For full markdown support, install and use a library like 'marked' or 'markdown-it'
    function formatContent(text: string): string {
        return text
            // Headers
            .replace(/^### (.+)$/gm, '<h4 class="font-semibold mt-3 mb-1">$1</h4>')
            .replace(/^## (.+)$/gm, '<h3 class="font-semibold text-lg mt-4 mb-2">$1</h3>')
            .replace(/^# (.+)$/gm, '<h2 class="font-bold text-xl mt-4 mb-2">$1</h2>')
            // Bold and italic
            .replace(/\*\*(.+?)\*\*/g, '<strong>$1</strong>')
            .replace(/\*(.+?)\*/g, '<em>$1</em>')
            .replace(/__(.+?)__/g, '<strong>$1</strong>')
            .replace(/_(.+?)_/g, '<em>$1</em>')
            // Inline code
            .replace(/`([^`]+)`/g, '<code class="bg-muted px-1 py-0.5 rounded text-sm font-mono">$1</code>')
            // Links
            .replace(/\[([^\]]+)\]\(([^)]+)\)/g, '<a href="$2" class="text-primary underline" target="_blank" rel="noopener">$1</a>')
            // Line breaks
            .replace(/\n/g, '<br />');
    }
</script>

<div class="markdown-content">
    {@html formatContent(content)}
</div>
