<script setup lang="ts">
import { getSettings } from '@/settings';
import { listen } from '@tauri-apps/api/event';
import { onMounted, ref } from 'vue';


const accent = ref("");
const onSettingsUpdate = ref();

const emit = defineEmits([
    "toggle"
]);

defineProps({
    text: {
        required: true,
        type: String
    },
    toggled: {
        required: true,
        type: Boolean
    },
    disabled: {
        default: false,
        type: Boolean
    }
})

onMounted(async () => {

    let settings = await getSettings();
    accent.value = settings.accent

    onSettingsUpdate.value = listen("on-settings-update", async (_event) => {
        let settings = await getSettings();
        accent.value = settings.accent
    });
})

</script>

<template>
    <button @click="emit('toggle', !toggled)" :disabled="disabled"
        class="w-full p-2 pl-4 pr-4 border border-skin-surface0 hover:enabled:opacity-80 disabled:bg-skin-crust disabled:text-skin-text rounded-full"
        :class="`bg-skin-${toggled ? accent.toLowerCase() : 'base'} text-skin-${toggled ? 'crust' : 'text'}`">
        {{ text }}
    </button>
</template>