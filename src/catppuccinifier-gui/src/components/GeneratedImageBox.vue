<script setup lang="ts">
import { WebviewWindow } from "@tauri-apps/api/window";
import { onMounted, ref } from "vue";
import { getSettings } from "@/settings";
import PrimaryButton from "./PrimaryButton.vue";
import LatteSVG from "@icons/sunflower.svg"
import FrappeSVG from "@icons/pineapple.svg"
import MacchiatoSVG from "@icons/lotus.svg"
import MochaSVG from "@icons/branch.svg"
import OledSVG from "@icons/moon.svg"

const tailwindTheme = ref("");

defineEmits([
    "save",
])

onMounted(async () => {
    let settings = await getSettings();
    tailwindTheme.value = `${settings.theme}-${settings.accent}`;
});

const props = defineProps({
    disabled: {
        required: true,
        type: Boolean
    },
    flavor: {
        required: true,
        type: String
    },
    imagePath: {
        required: true,
        type: String
    }
})

function preview() {

    new WebviewWindow('preview', {
        url: `preview?path=${props.imagePath}`,
        title: "Preview " + props.flavor,
        width: 800,
        height: 800,
    });
}


</script>

<template>
    <div class="p-4 bg-skin-crust border border-skin-surface0 rounded-3xl flex flex-col">
        <div class="flex items-center">
            <LatteSVG v-if="flavor === 'Latte'" class="w-6 h-6 fill-skin-text" />
            <FrappeSVG v-if="flavor === 'Frappe'" class="w-6 h-6 fill-skin-text" />
            <MacchiatoSVG v-if="flavor === 'Macchiato'" class="w-6 h-6 fill-none stroke-skin-text" />
            <MochaSVG v-if="flavor === 'Mocha'" class="w-6 h-6 fill-skin-text" />
            <OledSVG v-if="flavor === 'Oled'" class="w-6 h-6 stroke-skin-text stroke-2" />
            <div class="text-2xl font-bold ml-3">{{ flavor }}</div>
        </div>

        <div v-if="imagePath === ''">
            <progress :data-theme="tailwindTheme" class="progress progress-primary"></progress>
        </div>

        <div v-else class="">
            <div class="flex mt-4">
                <PrimaryButton icon="save" text="Save" :expand="true" :disabled="disabled"
                    @click="$emit('save', { flavor: flavor.toLowerCase() })" />
                <PrimaryButton @click="preview()" class="ml-4" icon="maximize" text="Preview" :disabled="disabled" />
            </div>

            <img :key="Math.random()" class="bg-skin-base rounded-3xl p-4 object-contain mt-4 aspect-square"
                :src="imagePath">
        </div>
    </div>
</template>