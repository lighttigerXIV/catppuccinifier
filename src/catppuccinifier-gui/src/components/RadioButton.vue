<script setup  lang="ts">
import { getSettings } from '@/settings';
import { listen } from '@tauri-apps/api/event';
import { onMounted, ref } from 'vue';


const itemsAccent = ref("");
const onSettingsUpdate = ref();


const emit = defineEmits(["click"])

defineProps({
    checked: {
        required: true,
        type: Boolean
    }
});

onMounted(async () => {
    let settings = await getSettings();
    itemsAccent.value = `${settings.theme}-${settings.accent}`

    onSettingsUpdate.value = listen("on-settings-update", async (_event) => {
        let settings = await getSettings();
        itemsAccent.value = `${settings.theme}-${settings.accent}`
    });
});
</script>

<template>
    <input :checked="checked" type="radio" class="radio radio-sm radio-primary" :data-theme="itemsAccent"
        @change="emit('click')">
</template>