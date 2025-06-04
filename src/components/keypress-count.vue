<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { onMounted, onUnmounted, ref } from "vue";

const POLL_INTERVAL_MS = 250;
const blockedKeypressCount = ref(0);
let pollInterval: number | null = null;


async function getBlockedKeypressCount() {
    try {
        blockedKeypressCount.value = await invoke("get_blocked_keypress_count");
    } catch (error) {
        console.error("Error getting blocked keypress count:", error);
    }
}

function startPolling() {
    getBlockedKeypressCount();
    pollInterval = setInterval(async () => {
        await getBlockedKeypressCount();
    }, POLL_INTERVAL_MS);
}

// Stop polling
function stopPolling() {
    if (pollInterval !== null) {
        clearInterval(pollInterval);
        pollInterval = null;
    }
}

// Lifecycle hooks
onMounted(() => {
    startPolling();
});

onUnmounted(() => {
    stopPolling();
});
</script>

<template>
    <p class="setting-description blocked-keypresses">Blocked keypresses: {{ blockedKeypressCount }}</p>
</template>

<style scoped>
.blocked-keypresses {
    font-weight: 700;
    text-align: center;
}
</style>