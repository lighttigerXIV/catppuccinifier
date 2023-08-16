<script setup lang="ts">
import { getSettings } from '@/settings';
import { listen } from '@tauri-apps/api/event';
import { onMounted, ref } from 'vue';


const daisyTheme = ref("");
const onSettingsUpdate = ref();


const emit = defineEmits(["change"]);

defineProps({
    checked: {
        required: true,
        type: Boolean
    }
});

onMounted(async () => {
    let settings = await getSettings();
    daisyTheme.value = `${settings.theme}-${settings.accent}`

    onSettingsUpdate.value = listen("on-settings-update", async (_event) => {
        let settings = await getSettings();
        daisyTheme.value = `${settings.theme}-${settings.accent}`;
    });
});

</script>

<template>
    <input type="checkbox" class="toggle toggle-primary " :data-theme="daisyTheme" :checked="checked"
        @change="emit('change', ($event.target as HTMLInputElement).checked)">
</template>