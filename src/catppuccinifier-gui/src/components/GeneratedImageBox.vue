<script setup lang="ts">
import { WebviewWindow } from "@tauri-apps/api/window";
import SaveSVG from "../assets/images/save.svg"
import MaximizeSVG from "../assets/images/maximize.svg"

defineEmits([
    "function:saveImage",
])

const props = defineProps({
    itemsAccent: {
        required: true,
        type: String
    },
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

function openPreview(path: string) {
    const webview = new WebviewWindow('preview', {
        url: "preview?path=" + path,
        title: "Preview " + props.flavor
    })

    webview.once('tauri://created', function () {})
    webview.once('tauri://error', function (e) {
        console.error(e);
    })
}

</script>

<template>
    <div class="p-4 bg-skin-crust border border-skin-surface0 rounded-3xl aspect-square flex flex-col">

        <div class="flex">

            <button
                class="bg-skin-mantle border border-skin-surface0 p-2 rounded-xl h-10 flex-grow mr-2 capitalize truncate hover:enabled:rounded-full disabled:bg-skin-crust disabled:border-skin-mantle"
                :disabled="disabled" @click="$emit('function:saveImage', '')">

                <div class="flex items-center justify-center">

                    <div>

                        <SaveSVG class="mr-2 h-5 w-5 stroke-skin-text fill-transparent" />
                    </div>

                    <div>
                        Save {{ flavor }}
                    </div>
                </div>
            </button>

            <button
                class="bg-skin-mantle border border-skin-surface0 p-2 rounded-xl h-10 hover:enabled:rounded-full disabled:bg-skin-crust disabled:border-skin-mantle"
                :disabled="disabled" @click="openPreview(imagePath)">

                <div class="flex items-center justify-center">

                    <div>

                        <MaximizeSVG class="mr-2 h-5 w-5 stroke-skin-text fill-transparent" />
                    </div>

                    <div>

                        Preview
                    </div>
                </div>
            </button>
        </div>

        <div v-if="imagePath.length === 0">

            <progress :data-theme="itemsAccent" class="progress progress-primary"></progress>
        </div>

        <div v-else class="flex flex-grow mt-4 justify-center  rounded-lg bg-skin-mantle border border-skin-surface0 p-2">

            <img :key="Math.random()" class="rounded-md object-contain" :src="imagePath">
        </div>
    </div>
</template>