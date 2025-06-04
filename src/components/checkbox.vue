<script setup lang="ts">
interface Props {
    modelValue: boolean;
    label: string;
    id: string;
}

defineProps<Props>();
defineEmits<{
    'update:modelValue': [value: boolean];
}>();
</script>

<template>
    <div class="checkbox-wrapper">
        <label class="checkbox-label" :for="id">
            <input type="checkbox" :id="id" :checked="modelValue"
                @change="$emit('update:modelValue', ($event.target as HTMLInputElement).checked)"
                class="checkbox-input" />
            <span class="checkmark"></span>
            <span class="label-text">{{ label }}</span>
        </label>
    </div>
</template>

<style scoped>
.checkbox-wrapper {
    margin: 16px 0;
}

.checkbox-label {
    display: flex;
    align-items: center;
    cursor: pointer;
    user-select: none;
    font-size: 16px;
    font-weight: 500;
    gap: 12px;
    transition: all 0.2s ease;
}

.checkbox-label:hover {
    color: #3b82f6;
}

.checkbox-input {
    display: none;
}

.checkmark {
    position: relative;
    width: 24px;
    height: 24px;
    border: 2px solid #d1d5db;
    border-radius: 6px;
    background-color: transparent;
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    flex-shrink: 0;
}

.checkmark::after {
    content: '';
    position: absolute;
    left: 6px;
    top: 2px;
    width: 6px;
    height: 12px;
    border: solid white;
    border-width: 0 2px 2px 0;
    transform: rotate(45deg) scale(0);
    transition: transform 0.2s cubic-bezier(0.4, 0, 0.2, 1);
}

.checkbox-input:checked+.checkmark {
    background-color: #3b82f6;
    border-color: #3b82f6;
}

.checkbox-input:checked+.checkmark::after {
    transform: rotate(45deg) scale(1);
}

.checkbox-label:hover .checkmark {
    border-color: #3b82f6;
    box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
}

.label-text {
    font-weight: 500;
    color: inherit;
}

@media (prefers-color-scheme: dark) {
    .checkmark {
        border-color: #6b7280;
        background-color: transparent;
    }

    .checkbox-input:checked+.checkmark {
        background-color: #3b82f6;
        border-color: #3b82f6;
    }

    .checkbox-label:hover .checkmark {
        border-color: #60a5fa;
        box-shadow: 0 0 0 3px rgba(96, 165, 250, 0.1);
    }
}
</style>
