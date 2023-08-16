<script setup lang="ts">
import { listen } from '@tauri-apps/api/event';
import { onMounted, ref } from 'vue';
import { getSettings } from '@/settings';


const updateSettingsEmit = ref();
const theme = ref("");
const tailwindTheme = ref("");

onMounted(async () => {

    let settings = await getSettings();

    theme.value = settings.theme;
    tailwindTheme.value = `theme-${settings.theme}`

    updateSettingsEmit.value = listen("on-settings-update", async (_event) => {

        let settings = await getSettings();

        theme.value = settings.theme;
        tailwindTheme.value = `theme-${settings.theme}`;
    })
})

</script>

<template>
    <div id="main" class="h-screen w-screen max-h-screen text-skin-text" :class="tailwindTheme">
        <RouterView />
    </div>
</template>./settings