<script setup lang="ts">
import { ImagePaths } from '@/data';
import { PropType } from 'vue';
import GeneratedImageBox from '@components/GeneratedImageBox.vue';
import { invoke } from '@tauri-apps/api/tauri';
import { save } from '@tauri-apps/api/dialog';
import { pictureDir, sep } from '@tauri-apps/api/path';

const props = defineProps({
    imagePaths: {
        required: true,
        type: Object as PropType<ImagePaths>,
    },
    generatingImages: {
        required: true,
        type: Boolean
    },
    flavors: {
        required: true,
        type: Array as PropType<string[]>
    },
    showImagesGrids: {
        required: true,
        type: Boolean
    },
    hald: {
        required: true,
        type: Number
    }
});


async function saveImage(flavor: string, path: string) {

    let defaultPath: string = `${await pictureDir()}${sep}${flavor}-hald${props.hald}-${props.imagePaths.selected.raw.split(sep).pop()}`;


    const savePath = await save({
        defaultPath: defaultPath,
        filters: [
            {
                name: "Images (png, jpg, jpeg, webp)",
                extensions: ["png", "webp", "jpg", "jpeg"]
            }
        ]
    });

    invoke("save_image", { image_path: path, save_path: savePath });
}

</script>
<template>
    <div class="flex-grow bg-skin-base overflow-auto h-screen p-4">
        <div class="text-3xl font-bold ml-1">Results</div>

        <div class="grid 2xl:grid-cols-2 xl:grid-cols-2 grid-cols-1 mt-4 gap-2">
            <GeneratedImageBox v-if="flavors.includes('latte')" :disabled="imagePaths.latte.local === ''" flavor="Latte"
                :image-path="imagePaths.latte.local" @save="saveImage($event.flavor, imagePaths.selected.raw)" />

            <GeneratedImageBox v-if="flavors.includes('frappe')" :disabled="imagePaths.frappe.local === ''" flavor="Frappe"
                :image-path="imagePaths.frappe.local" @save="saveImage($event.flavor, imagePaths.frappe.raw)" />

            <GeneratedImageBox v-if="flavors.includes('macchiato')" :disabled="imagePaths.macchiato.local === ''"
                flavor="Macchiato" :image-path="imagePaths.macchiato.local"
                @save="saveImage($event.flavor, imagePaths.macchiato.raw)" />

            <GeneratedImageBox v-if="flavors.includes('mocha')" :disabled="imagePaths.mocha.local === ''" flavor="Mocha"
                :image-path="imagePaths.mocha.local" @save="saveImage($event.flavor, imagePaths.mocha.raw)" />

            <GeneratedImageBox v-if="flavors.includes('oled')" :disabled="imagePaths.oled.local === ''" flavor="Oled"
                :image-path="imagePaths.oled.local" @save="saveImage($event.flavor, imagePaths.oled.raw)" />
        </div>
    </div>
</template>