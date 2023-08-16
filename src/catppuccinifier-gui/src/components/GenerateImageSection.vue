<script setup lang="ts">
import { Algorithms, GenerateImagesData, ImagePaths } from '@/data';
import { open as openLink } from '@tauri-apps/api/shell';
import ImageSVG from "@icons/image.svg"
import PrimaryButton from '@components/PrimaryButton.vue';
import FlavorToggleButton from "@components/FlavorToggleButton.vue"
import { PropType, onMounted, ref } from 'vue';
import { open } from '@tauri-apps/api/dialog';
import { convertFileSrc } from '@tauri-apps/api/tauri';
import { listen } from '@tauri-apps/api/event';
import { WebviewWindow } from '@tauri-apps/api/window';
import Toggle from "@components/Toggle.vue"
import RadioButton from '@components/RadioButton.vue';
import Slider from "@components/Slider.vue"
import { getSettings } from '@/settings';


const fileDropHoverEmit = ref();
const fileDropCancelEmit = ref();
const fileDropEmit = ref();

const selectImageSectionBorder = ref("border-skin-surface0");

const generateLatte = ref(true);
const generateFrappe = ref(true);
const generateMacchiato = ref(true);
const generateMocha = ref(true);
const generateOled = ref(true);

const hald = ref(8);
const luminosity = ref(1.0);
const algorithm = ref(Algorithms.gaussian.codename);
const gaussianShape = ref<number>(Algorithms.gaussian.properties.shape.default);
const gaussianNearest = ref<number>(Algorithms.gaussian.properties.nearest.default);
const gaussianSamplingMean = ref<number>(Algorithms.gaussian_sampling.properties.mean.default);
const gaussianSamplingSTD = ref<number>(Algorithms.gaussian_sampling.properties.std.default);
const gaussianSamplingIterations = ref<number>(Algorithms.gaussian_sampling.properties.iterations.default);
const linearNearest = ref<number>(Algorithms.linear.properties.nearest.default);
const sheppardPower = ref<number>(Algorithms.sheppard.properties.power.default);
const sheppardNearest = ref<number>(Algorithms.sheppard.properties.nearest.default);
const showAdvancedConversion = ref(false);


const emit = defineEmits([
    "generate",
    "updatePaths"
]);

const props = defineProps({
    imagePaths: {
        required: true,
        type: Object as PropType<ImagePaths>
    },
    generatingImages: {
        required: true,
        type: Boolean,
    }
})

onMounted(async () => {

    fileDropHoverEmit.value = listen("tauri://file-drop-hover", async (_event) => {
        let settings = await getSettings();
        selectImageSectionBorder.value = `border-skin-${settings.accent}`;
    });

    fileDropCancelEmit.value = listen("tauri://file-drop-cancelled", (_event) => {
        selectImageSectionBorder.value = `border-skin-surface0`
    });

    fileDropEmit.value = listen("tauri://file-drop", (event) => {
        selectImageSectionBorder.value = `border-skin-surface0`;
        let paths = event.payload as string[];

        if (paths.length > 0) {
            props.imagePaths.selected.raw = paths[0];
            props.imagePaths.selected.local = `${convertFileSrc(paths[0])}?${Math.random()}`
            emit("updatePaths", props.imagePaths);
        }
    });
});

function isGenerateButtonDisabled(): boolean {
    return props.imagePaths.selected.raw === "" || (!generateLatte.value && !generateFrappe.value && !generateMacchiato.value && !generateMocha.value && !generateOled.value) || props.generatingImages
}

async function generateImage() {

    let nearest: number = -1;

    if (algorithm.value === Algorithms.gaussian.codename) {
        nearest = gaussianNearest.value
    } else if (algorithm.value === Algorithms.linear.codename) {
        nearest = linearNearest.value
    } else {
        nearest = sheppardNearest.value
    }

    let flavors: string[] = [];
    if (generateLatte.value) { flavors.push("latte") }
    if (generateFrappe.value) { flavors.push("frappe") }
    if (generateMacchiato.value) { flavors.push("macchiato") }
    if (generateMocha.value) { flavors.push("mocha") }
    if (generateOled.value) { flavors.push("oled") }

    let data: GenerateImagesData = {
        imagePath: props.imagePaths.selected.raw,
        hald: hald.value,
        luminosity: luminosity.value,
        algorithm: algorithm.value,
        shape: gaussianShape.value,
        nearest: nearest,
        mean: gaussianSamplingMean.value,
        std: gaussianSamplingSTD.value,
        iterations: gaussianSamplingIterations.value,
        power: sheppardPower.value,
    }

    emit("generate", {
        data: data,
        flavors: flavors
    });
}

async function selectImage() {
    const selected = await open({
        multiple: false,
        filters: [{
            name: "Images",
            extensions: ["png", "webp", "jpg", "jpeg"]
        }]
    });

    if (selected !== null && typeof (selected) === "string") {
        props.imagePaths.selected.raw = selected;
        props.imagePaths.selected.local = `${convertFileSrc(selected)}?${Math.random()}`
    }

    emit("updatePaths", props.imagePaths)
}

async function openSettings() {

    new WebviewWindow("settings", {
        url: "settings",
        title: "Settings",
        width: 800,
        height: 600,
        center: true
    })
}

</script>

<template>
    <div class="flex flex-col h-screen overflow-auto p-4 bg-skin-crust w-[500px] min-w-[500px]">
        <div class="flex-grow">
            <div class="text-3xl font-bold ml-1">Properties</div>

            <PrimaryButton @click="generateImage()" text="Generate" icon="wand" :expand="true"
                :disabled="isGenerateButtonDisabled()" class="mt-4" />

            <div class=" bg-skin-crust mt-4 p-4 rounded-3xl border" :class="selectImageSectionBorder">
                <div v-if="props.imagePaths.selected.raw == ''" class="h-[200px] flex justify-center items-center">
                    <ImageSVG class="h-full w-full fill-skin-surface1" />
                </div>
                <img v-else :src="props.imagePaths.selected.local" class=" h-[200px] w-full object-contain">
                <div class="flex justify-center">
                    <PrimaryButton @click="selectImage()" icon="folder" text="Select or drop image" class="mt-4" :expand="true" :disabled="generatingImages"/>
                </div>
            </div>

            <div class=" bg-skin-crust border border-skin-surface0 rounded-3xl p-4 mt-4">
                <div class="ml-2 text-xl">Flavors</div>
                <div class="grid grid-cols-3 gap-2 mt-2">
                    <FlavorToggleButton text="Latte" :toggled="generateLatte" @toggle="generateLatte = $event"
                        :disabled="generatingImages" />
                    
                    <FlavorToggleButton text="Frappe" :toggled="generateFrappe" @toggle="generateFrappe = $event"
                        :disabled="generatingImages" />
                    
                    <FlavorToggleButton text="Macchiato" :toggled="generateMacchiato" @toggle="generateMacchiato = $event"
                        :disabled="generatingImages" />
                    
                    <FlavorToggleButton text="Mocha" :toggled="generateMocha" @toggle="generateMocha = $event"
                        :disabled="generatingImages" />
                    
                    <FlavorToggleButton text="Oled" :toggled="generateOled" @toggle="generateOled = $event"
                        :disabled="generatingImages" />
                </div>
            </div>



            <div class="mt-2 p-4 bg-skin-crust rounded-3xl border border-skin-surface0">

                <div class="ml-2 text-xl">Algorithm</div>

                <div class="mt-4 border border-skin-surface0 rounded-xl p-4">
                    <div class="ml-1">Hald Level - {{ hald }}</div>
                    <Slider :value="hald" :min="2" :max="16" @change="hald = $event" />
                </div>

                <div class="mt-1 border border-skin-surface0 rounded-xl p-4">
                    <div class="ml-1">Luminosity - {{ luminosity }}</div>
                    <Slider :value="luminosity" :min="0.1" :max="1" :step="0.1" @change="luminosity = $event" />
                </div>

                <div class="flex items-center mt-4 border p-4 rounded-xl border-skin-surface0">

                    <div class="flex-grow">Advanced</div>

                    <Toggle :checked="showAdvancedConversion" @change="showAdvancedConversion = $event" />

                </div>

                <div class="mt-1 border border-skin-surface0 rounded-xl">

                    <div class="flex items-center cursor-pointer p-4" @click="algorithm = Algorithms.gaussian.codename">

                        <div class="flex-grow" :class="algorithm == Algorithms.gaussian.codename ? 'font-bold' : ''">
                            {{ Algorithms.gaussian.name }}
                        </div>

                        <RadioButton :checked="algorithm === Algorithms.gaussian.codename"
                            @click="algorithm = Algorithms.gaussian.codename" />
                    </div>


                    <div v-if="algorithm == Algorithms.gaussian.codename && showAdvancedConversion">
                        <div class="p-4">

                            <div>Shape - {{ gaussianShape }}</div>

                            <Slider :value="gaussianShape" :max="Algorithms.gaussian.properties.shape.max"
                                :min="Algorithms.gaussian.properties.shape.min" @change="gaussianShape = $event" />

                            <div>Nearest - {{ gaussianNearest }}</div>

                            <Slider :value="gaussianNearest" :max="Algorithms.gaussian.properties.nearest.max"
                                :min="Algorithms.gaussian.properties.nearest.min" @change="gaussianNearest = $event" />

                        </div>
                    </div>
                </div>

                <div class="mt-1 border border-skin-surface0 rounded-xl">

                    <div class="flex items-center cursor-pointer p-4"
                        @click="algorithm = Algorithms.gaussian_sampling.codename">

                        <div class="flex-grow"
                            :class="algorithm == Algorithms.gaussian_sampling.codename ? 'font-bold' : ''">
                            {{ Algorithms.gaussian_sampling.name }}
                        </div>

                        <RadioButton :checked="algorithm === Algorithms.gaussian_sampling.codename"
                            @click="algorithm = Algorithms.gaussian_sampling.codename" />
                    </div>


                    <div v-if="algorithm == Algorithms.gaussian_sampling.codename && showAdvancedConversion">
                        <div class="p-4">

                            <div>Mean - {{ gaussianSamplingMean }}</div>

                            <Slider :value="gaussianSamplingMean" :max="Algorithms.gaussian_sampling.properties.mean.max"
                                :min="Algorithms.gaussian_sampling.properties.mean.min"
                                @change="gaussianSamplingMean = $event" />

                            <div>STD - {{ gaussianSamplingSTD }}</div>

                            <Slider :value="gaussianSamplingSTD" :max="Algorithms.gaussian_sampling.properties.std.max"
                                :min="Algorithms.gaussian_sampling.properties.std.min"
                                @change="gaussianSamplingSTD = $event" />

                            <div>Iterations - {{ gaussianSamplingIterations }}</div>

                            <Slider :value="gaussianSamplingIterations"
                                :max="Algorithms.gaussian_sampling.properties.iterations.max"
                                :min="Algorithms.gaussian_sampling.properties.iterations.min"
                                @change="gaussianSamplingIterations = $event" />

                        </div>
                    </div>
                </div>

                <div class="mt-1 border border-skin-surface0 rounded-xl">

                    <div class="flex items-center cursor-pointer p-4" @click="algorithm = Algorithms.linear.codename">

                        <div class="flex-grow" :class="algorithm == Algorithms.linear.codename ? 'font-bold' : ''">
                            {{ Algorithms.linear.name }}
                        </div>

                        <RadioButton :checked="algorithm === Algorithms.linear.codename"
                            @click="algorithm = Algorithms.linear.codename" />
                    </div>


                    <div v-if="algorithm == Algorithms.linear.codename && showAdvancedConversion">
                        <div class="p-4">

                            <div>Nearest - {{ linearNearest }}</div>

                            <Slider :value="linearNearest" :max="Algorithms.linear.properties.nearest.max"
                                :min="Algorithms.linear.properties.nearest.min" @change="linearNearest = $event" />

                        </div>
                    </div>
                </div>

                <div class="mt-1 border border-skin-surface0 rounded-xl">

                    <div class="flex items-center cursor-pointer p-4" @click="algorithm = Algorithms.sheppard.codename">

                        <div class="flex-grow" :class="algorithm == Algorithms.sheppard.codename ? 'font-bold' : ''">
                            {{ Algorithms.sheppard.name }}
                        </div>

                        <RadioButton :checked="algorithm === Algorithms.sheppard.codename"
                            @click="algorithm = Algorithms.sheppard.codename" />
                    </div>


                    <div v-if="algorithm == Algorithms.sheppard.codename && showAdvancedConversion">
                        <div class="p-4">

                            <div>Power - {{ sheppardPower }}</div>

                            <Slider :value="sheppardPower" :max="Algorithms.sheppard.properties.power.max"
                                :min="Algorithms.sheppard.properties.power.min" @change="sheppardPower = $event" />

                            <div>Nearest - {{ sheppardNearest }}</div>

                            <Slider :value="sheppardNearest" :max="Algorithms.sheppard.properties.nearest.max"
                                :min="Algorithms.sheppard.properties.nearest.min" @change="sheppardNearest = $event" />
                        </div>
                    </div>
                </div>

                <div class="mt-1 border border-skin-surface0 rounded-xl">

                    <div class="flex items-center cursor-pointer p-4"
                        @click="algorithm = Algorithms.nearest_neighbor.codename">

                        <div class="flex-grow"
                            :class="algorithm == Algorithms.nearest_neighbor.codename ? 'font-bold' : ''">
                            {{ Algorithms.nearest_neighbor.name }}
                        </div>

                        <RadioButton :checked="algorithm === Algorithms.nearest_neighbor.codename"
                            @click="algorithm = Algorithms.nearest_neighbor.codename" />
                    </div>
                </div>
            </div>

        </div>
        <div class="flex mt-4">
            <PrimaryButton @click="openSettings()" icon="settings" text="Settings" :expand="true" />
            <PrimaryButton @click="openLink('https://github.com/lighttigerXIV/catppuccinifier')" icon="github" text="About"
                :expand="true" class="ml-4" />
    </div>
</div></template>