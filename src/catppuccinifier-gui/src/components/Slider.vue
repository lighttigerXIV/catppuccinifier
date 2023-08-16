<script setup lang="ts">
import { getSettings } from '@/settings';
import { listen } from '@tauri-apps/api/event';
import { onMounted, ref } from 'vue';


const daisyTheme = ref("");
const onSettingsUpdate = ref();

const emit = defineEmits(["change"]);

defineProps({
    value: {
        required: true,
        type: Number
    },
    min: {
        required: true,
        type: Number
    },
    max: {
        required: true,
        type: Number
    },
    step: {
        type: Number,
        default: 1
    }
})

onMounted(async () => {
    let settings = await getSettings();
    daisyTheme.value = `${settings.theme}-${settings.accent}`;

    onSettingsUpdate.value = listen("on-settings-update", async (_event) => {
        let settings = await getSettings();
        daisyTheme.value = `${settings.theme}-${settings.accent}`;
    });
})
</script>

<template>
    <input class="range range-sm range-primary" type="range" :data-theme="daisyTheme" :step="step" :value="value" :max="max"
        :min="min" @input="emit('change', +($event.target as HTMLInputElement).value)">
</template>