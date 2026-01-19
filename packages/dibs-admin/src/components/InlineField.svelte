<script lang="ts">
    import { Pencil, Check, X } from "phosphor-svelte";
    import { Input, NumberInput, Checkbox, Textarea, Select, DatetimeInput } from "../lib/components/ui/index.js";
    import CodeMirrorEditor from "./CodeMirrorEditor.svelte";

    type FieldType = "text" | "number" | "boolean" | "datetime" | "enum" | "textarea" | "codemirror";

    interface Props {
        value: string;
        type?: FieldType;
        readOnly?: boolean;
        disabled?: boolean;
        placeholder?: string;
        enumOptions?: string[];
        lang?: string | null;
        onSave?: (newValue: string) => void | Promise<void>;
    }

    let {
        value,
        type = "text",
        readOnly = false,
        disabled = false,
        placeholder = "",
        enumOptions = [],
        lang = null,
        onSave,
    }: Props = $props();

    let isEditing = $state(false);
    let editValue = $state("");
    let saving = $state(false);

    // Keep editValue in sync with value when not editing
    $effect(() => {
        if (!isEditing) {
            editValue = value;
        }
    });

    function startEdit() {
        if (readOnly || disabled) return;
        editValue = value;
        isEditing = true;
    }

    async function save() {
        if (editValue === value) {
            isEditing = false;
            return;
        }

        saving = true;
        try {
            await onSave?.(editValue);
            isEditing = false;
        } catch (e) {
            console.error("Failed to save:", e);
            // Keep editing on error
        } finally {
            saving = false;
        }
    }

    function cancel() {
        editValue = value;
        isEditing = false;
    }

    function handleKeydown(e: KeyboardEvent) {
        if (e.key === "Enter" && type !== "textarea" && type !== "codemirror") {
            e.preventDefault();
            save();
        } else if (e.key === "Escape") {
            e.preventDefault();
            cancel();
        }
    }

    function handleBlur() {
        // Don't auto-save on blur for textarea/codemirror (user might want to cancel)
        if (type === "textarea" || type === "codemirror") return;
        if (isEditing && !saving) {
            save();
        }
    }

    function formatDisplayValue(val: string): string {
        if (val === "" || val === "null") return placeholder || "—";
        if (type === "boolean") return val === "true" ? "Yes" : "No";
        if (type === "datetime") {
            const date = new Date(val);
            if (!isNaN(date.getTime())) {
                return date.toLocaleString();
            }
        }
        // Truncate long values for display
        if (val.length > 100) return val.slice(0, 100) + "…";
        return val;
    }

    function getBoolValue(): boolean {
        return editValue.toLowerCase() === "true" || editValue === "1";
    }

    function setBoolValue(checked: boolean) {
        editValue = checked ? "true" : "false";
    }
</script>

<div class="inline-field group min-h-[36px] flex items-center">
    {#if isEditing}
        <div class="flex-1 flex items-center gap-2">
            {#if type === "boolean"}
                <div class="flex items-center gap-3 h-9 px-3">
                    <Checkbox
                        checked={getBoolValue()}
                        onCheckedChange={(checked) => {
                            setBoolValue(checked === true);
                            save();
                        }}
                        {disabled}
                    />
                    <span class="text-sm">{getBoolValue() ? "Yes" : "No"}</span>
                </div>
            {:else if type === "number"}
                <NumberInput
                    value={editValue}
                    oninput={(e) => (editValue = e.currentTarget.value)}
                    onkeydown={handleKeydown}
                    onblur={handleBlur}
                    {placeholder}
                    {disabled}
                    class="flex-1"
                />
            {:else if type === "datetime"}
                <DatetimeInput
                    value={editValue}
                    onchange={(v: string) => {
                        editValue = v;
                        save();
                    }}
                    {disabled}
                />
            {:else if type === "enum"}
                <Select.Root
                    type="single"
                    value={editValue}
                    {disabled}
                    onValueChange={(v) => {
                        editValue = v;
                        save();
                    }}
                >
                    <Select.Trigger class="w-full">
                        {editValue || placeholder || "— Select —"}
                    </Select.Trigger>
                    <Select.Content>
                        <Select.Item value="">— None —</Select.Item>
                        {#each enumOptions as option}
                            <Select.Item value={option}>{option}</Select.Item>
                        {/each}
                    </Select.Content>
                </Select.Root>
            {:else if type === "textarea"}
                <div class="flex-1 flex flex-col gap-2">
                    <Textarea
                        value={editValue}
                        oninput={(e) => (editValue = e.currentTarget.value)}
                        onkeydown={handleKeydown}
                        {placeholder}
                        disabled={disabled || false}
                        rows={4}
                    />
                    <div class="flex justify-end gap-2">
                        <button
                            type="button"
                            class="text-muted-foreground hover:text-foreground p-1"
                            onclick={cancel}
                        >
                            <X size={16} />
                        </button>
                        <button
                            type="button"
                            class="text-primary hover:text-primary/80 p-1"
                            onclick={save}
                            disabled={saving}
                        >
                            <Check size={16} />
                        </button>
                    </div>
                </div>
            {:else if type === "codemirror"}
                <div class="flex-1 flex flex-col gap-2">
                    <CodeMirrorEditor
                        value={editValue}
                        {lang}
                        {disabled}
                        {placeholder}
                        onchange={(v) => (editValue = v)}
                    />
                    <div class="flex justify-end gap-2">
                        <button
                            type="button"
                            class="text-muted-foreground hover:text-foreground p-1"
                            onclick={cancel}
                        >
                            <X size={16} />
                        </button>
                        <button
                            type="button"
                            class="text-primary hover:text-primary/80 p-1"
                            onclick={save}
                            disabled={saving}
                        >
                            <Check size={16} />
                        </button>
                    </div>
                </div>
            {:else}
                <Input
                    type="text"
                    value={editValue}
                    oninput={(e) => (editValue = e.currentTarget.value)}
                    onkeydown={handleKeydown}
                    onblur={handleBlur}
                    {placeholder}
                    {disabled}
                    class="flex-1"
                />
            {/if}
        </div>
    {:else}
        <!-- Display mode -->
        <button
            type="button"
            class="flex-1 text-left px-3 py-2 min-h-[36px] rounded-md text-sm transition-colors
                   {readOnly || disabled
                ? 'text-muted-foreground cursor-default'
                : 'hover:bg-accent/50 cursor-pointer group-hover:bg-accent/30'}"
            onclick={startEdit}
            disabled={readOnly || disabled}
        >
            <span class={value === "" || value === "null" ? "text-muted-foreground/60" : ""}>
                {formatDisplayValue(value)}
            </span>
        </button>
        {#if !readOnly && !disabled}
            <span class="opacity-0 group-hover:opacity-100 transition-opacity text-muted-foreground/60 pr-2">
                <Pencil size={14} />
            </span>
        {/if}
    {/if}
</div>
