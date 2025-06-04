<script setup lang="ts">
import { computed } from 'vue';

interface Props {
    modelValue: number;
    label: string;
    id: string;
    min?: number;
    max?: number;
    step?: number;
    unit?: string;
    showInput?: boolean;
}

const props = withDefaults(defineProps<Props>(), {
    min: 0,
    max: 100,
    step: 1,
    showInput: false,
});

const emit = defineEmits<{
    'update:modelValue': [value: number];
    'change': [value: number];
}>();

const fillPercentage = computed(() => {
    return ((props.modelValue - props.min) / (props.max - props.min)) * 100;
});

const handleInput = (event: Event) => {
    const target = event.target as HTMLInputElement;
    emit('update:modelValue', Number(target.value));
};

const handleChange = (event: Event) => {
    const target = event.target as HTMLInputElement;
    emit('change', Number(target.value));
};

const handleInputFieldChange = (event: Event) => {
    const target = event.target as HTMLInputElement;
    let value = Number(target.value);

    if (value < props.min) value = props.min;
    if (value > props.max) value = props.max;

    value = Math.round(value / props.step) * props.step;
    emit('update:modelValue', value);
    emit('change', value);
    target.value = value.toString();
};
</script>

<template>
    <div class="slider-wrapper">
        <div class="slider-header">
            <label :for="id" class="slider-label">{{ label }}</label>
            <div class="value-display" v-if="!showInput">
                <span class="value-number">{{ modelValue }}</span>
                <span class="value-unit" v-if="unit">{{ unit }}</span>
            </div>
            <div class="input-wrapper" v-else>
                <input type="number" :value="modelValue" @input="handleInputFieldChange" @blur="handleInputFieldChange"
                    class="interval-input" :min="min" :max="max" :step="step" />
                <span class="input-unit" v-if="unit">{{ unit }}</span>
            </div>
        </div>
        <div class="slider-container">
            <input type="range" :id="id" :value="modelValue" :min="min" :max="max" :step="step" @input="handleInput"
                @change="handleChange" class="slider-input" />
            <div class="slider-track">
                <div class="slider-fill" :style="{ width: fillPercentage + '%' }"></div>
            </div>
            <div class="slider-thumb" :style="{ left: fillPercentage + '%' }"></div>
        </div>
        <div class="slider-range">
            <span class="range-min">{{ min }}</span>
            <span class="range-max">{{ max }}</span>
        </div>
    </div>
</template>

<style scoped>
.slider-wrapper {
    margin: 24px 0;
    width: 100%;
    max-width: 400px;
}

.slider-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 12px;
}

.slider-label {
    font-size: 16px;
    font-weight: 600;
    color: #374151;
}

.value-display {
    display: flex;
    align-items: baseline;
    gap: 2px;
    font-weight: 600;
}

.value-number {
    font-size: 18px;
    color: #3b82f6;
}

.value-unit {
    font-size: 14px;
    color: #6b7280;
}

.slider-container {
    position: relative;
    height: 24px;
    display: flex;
    align-items: center;
}

.slider-input {
    position: absolute;
    width: 100%;
    height: 100%;
    opacity: 0;
    cursor: pointer;
    z-index: 3;
    margin: 0;
}

.slider-track {
    position: absolute;
    width: 100%;
    height: 8px;
    background-color: #e5e7eb;
    border-radius: 4px;
    overflow: hidden;
}

.slider-fill {
    height: 100%;
    background: linear-gradient(90deg, #3b82f6 0%, #1d4ed8 100%);
    border-radius: 4px;
    transition: width 0.15s ease;
}

.slider-thumb {
    position: absolute;
    width: 20px;
    height: 20px;
    background: white;
    border: 3px solid #3b82f6;
    border-radius: 50%;
    top: 50%;
    transform: translate(-50%, -50%);
    transition: all 0.15s ease;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
    z-index: 2;
}

.slider-input:hover+.slider-track+.slider-thumb {
    transform: translate(-50%, -50%) scale(1.1);
    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.15);
}

.slider-input:active+.slider-track+.slider-thumb {
    transform: translate(-50%, -50%) scale(1.2);
    box-shadow: 0 0 0 4px rgba(59, 130, 246, 0.2);
}

.slider-range {
    display: flex;
    justify-content: space-between;
    margin-top: 8px;
    font-size: 12px;
    color: #6b7280;
}

.range-min,
.range-max {
    font-weight: 500;
}

.input-wrapper {
    display: flex;
    align-items: center;
    gap: 8px;
}

.interval-input {
    width: 80px;
    padding: 8px 12px;
    border: 2px solid #e5e7eb;
    border-radius: 8px;
    font-size: 14px;
    font-weight: 500;
    color: #374151;
    background: white;
    transition: all 0.2s ease;
    text-align: center;
}

.interval-input:focus {
    outline: none;
    border-color: #3b82f6;
    box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
}

.input-unit {
    font-size: 14px;
    font-weight: 500;
    color: #6b7280;
}

@media (prefers-color-scheme: dark) {
    .slider-label {
        color: #f9fafb;
    }

    .value-unit {
        color: #9ca3af;
    }

    .slider-track {
        background-color: #374151;
    }

    .slider-thumb {
        background: #1f2937;
        border-color: #3b82f6;
    }

    .slider-range {
        color: #9ca3af;
    }

    .interval-input {
        background: #374151;
        border-color: #4b5563;
        color: #f9fafb;
    }

    .interval-input:focus {
        border-color: #3b82f6;
        box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.2);
    }

    .input-unit {
        color: #9ca3af;
    }
}
</style>