<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import Checkbox from "./components/checkbox.vue";
import Slider from "./components/slider.vue";
import KeypressCount from "./components/keypress-count.vue";

const enableKeyfix = ref(true);
const intervalMs = ref(50);

async function setDebounceInterval() {
  try {
    await invoke("set_debounce_interval", { intervalMs: Number(intervalMs.value) });
    console.log("Successfully set debounce interval");
  } catch (error) {
    console.error("Error setting debounce interval:", error);
  }
}

async function toggleKeyfix() {
  try {
    await invoke("set_keyfix_enabled", { enabled: enableKeyfix.value });
  } catch (error) {
    console.error("Error toggling keyfix:", error);
  }
}

async function handleSliderChange() {
  await setDebounceInterval();
}
</script>

<template>
  <main class="container">
    <div class="header">
      <h1 class="title">Keyfix Settings</h1>
    </div>

    <div class="settings-card">
      <div class="setting-section">
        <Checkbox v-model="enableKeyfix" id="enable-keyfix" label="Enable Keyfix" @update:modelValue="toggleKeyfix" />
        <p class="setting-description">
          Toggle keyboard key debouncing to prevent duplicate key presses.
        </p>
      </div>

      <div class="setting-section">
        <Slider v-model="intervalMs" id="debounce-interval" label="Debounce Interval" :min="5" :max="1000" :step="5"
          unit="ms" @change="handleSliderChange" />
        <p class="setting-description">
          Set the debounce delay in milliseconds. Lower values are more responsive but may not filter out all double
          keypresses.
        </p>
      </div>

      <div class="setting-section">
        <KeypressCount />
      </div>
    </div>
  </main>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>

<style>
:root {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif;
  font-size: 16px;
  line-height: 1.6;
  font-weight: 400;

  color: #1f2937;
  background: linear-gradient(135deg, #f8fafc 0%, #e2e8f0 100%);
  min-height: 100vh;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

* {
  box-sizing: border-box;
}

body {
  margin: 0;
  padding: 0;
  min-height: 100vh;
}

.container {
  margin: 0 auto;
  padding: 40px 20px;
  max-width: 600px;
  min-height: 100vh;
  display: flex;
  flex-direction: column;
  justify-content: flex-start;
  align-items: center;
}

.header {
  text-align: center;
  margin-bottom: 40px;
}

.title {
  font-size: 2.5rem;
  font-weight: 700;
  margin: 0 0 12px 0;
  background: linear-gradient(135deg, #3b82f6 0%, #1d4ed8 100%);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
}

.settings-card {
  background: white;
  border-radius: 16px;
  padding: 32px;
  box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06);
  width: 100%;
  max-width: 500px;
  border: 1px solid rgba(0, 0, 0, 0.05);
}

.setting-section {
  margin-bottom: 32px;
}

.setting-section:last-child {
  margin-bottom: 0;
}

.setting-description {
  margin: 8px 0 0 0;
  font-size: 14px;
  color: #6b7280;
  line-height: 1.5;
}

.row {
  display: flex;
  justify-content: center;
  align-items: center;
  gap: 16px;
}

.col {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 16px;
}

@media (max-width: 640px) {
  .container {
    padding: 20px 16px;
  }

  .title {
    font-size: 2rem;
  }

  .settings-card {
    padding: 24px;
  }
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f9fafb;
    background: linear-gradient(135deg, #0f172a 0%, #1e293b 100%);
  }

  .settings-card {
    background: #1f2937;
    border-color: #374151;
    box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.3), 0 2px 4px -1px rgba(0, 0, 0, 0.2);
  }

  .subtitle {
    color: #9ca3af;
  }

  .setting-description {
    color: #9ca3af;
  }
}
</style>