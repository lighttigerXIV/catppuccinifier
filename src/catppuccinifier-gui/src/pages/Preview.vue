<script setup lang="ts">
import { getSettings } from '@/settings';
import { listen } from '@tauri-apps/api/event';
import { onMounted, ref } from 'vue';
import { useRoute } from 'vue-router';

const theme = ref("");
const imagePath = useRoute().query.path?.toString();
const onSettingsUpdate = ref();

onMounted(async () => {
    let settings = await getSettings();
    theme.value = `theme=${settings.theme}`

    onSettingsUpdate.value = listen("on-settings-update", async (_event) => {
        let settings = await getSettings();
        theme.value = `theme=${settings.theme}`
    });
});

</script>

<template>
    <div class="h-screen min-w-full w-full bg-skin-base text-text p-4 flex justify-center" :class="theme">
        <img class="object-contain rounded-md h-full" :src="imagePath">
    </div>
</template>