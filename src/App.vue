<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";


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

</script>

<template>
  <main class="container">
    <h1>Keyfix Settings</h1>

    <div class="row">
      <input type="checkbox" id="enable-keyfix" v-model="enableKeyfix" @change="toggleKeyfix" />
      <label for="enable-keyfix">Enable Keyfix</label>
    </div>

    <div class="col">
      <div class="row">
        <input type="range" id="debounce-interval" v-model="intervalMs" min="5" max="1000"
          @change="setDebounceInterval" />
        <label for="debounce-interval">Debounce Interval (ms)</label>
      </div>

      <div class="row">
        <p>{{ intervalMs }}</p>
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
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

.row {
  display: flex;
  justify-content: center;
}

.col {
  display: flex;
  flex-direction: column;
  align-items: center;
}

h1 {
  text-align: center;
}


@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  a:hover {
    color: #24c8db;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }

  button:active {
    background-color: #0f0f0f69;
  }
}
</style>