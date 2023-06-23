<script setup lang="ts">
import { ConversionMethods } from '../Constants';
import { open as openLink } from '@tauri-apps/api/shell';
import SettingsSVG from "../assets/images/settings.svg"
import GitHubSVG from "../assets/images/github.svg"


defineEmits([
    "update:haldLevel",
    "update:conversionMethod",
    "update:showAdvancedConversion",
    "update:gaussianShape",
    "update:gaussianNearest",
    "update:gaussianSamplingMean",
    "update:gaussianSamplingSTD",
    "update:gaussianSamplingIterations",
    "update:linearNearest",
    "update:sheppardPower",
    "update:sheppardNearest",
    "function:selectImage",
    "function:generateImages"
])

defineProps({
    itemsAccent: {
        required: true,
        type: String
    },
    haldLevel: {
        required: true,
        type: Number
    },
    conversionMethod: {
        required: true,
        type: String
    },
    showAdvancedConversion: {
        required: true,
        type: Boolean
    },
    gaussianShape: {
        required: true,
        type: Number
    },
    gaussianNearest: {
        required: true,
        type: Number
    },
    gaussianSamplingMean: {
        required: true,
        type: Number
    },
    gaussianSamplingSTD: {
        required: true,
        type: Number
    },
    gaussianSamplingIterations: {
        required: true,
        type: Number
    },
    linearNearest: {
        required: true,
        type: Number
    },
    sheppardPower: {
        required: true,
        type: Number
    },
    sheppardNearest: {
        required: true,
        type: Number

    },
    accent: {
        required: true,
        type: String
    },
    generatingImages: {
        required: true,
        type: Boolean
    },
    selectedImagePath: {
        required: true,
        type: String
    }
})


</script>

<template>
    <div class="flex flex-col flex-grow p-4">

        <div class="p-2 bg-skin-mantle rounded-xl border border-skin-surface0">

            <div>Hald Level - {{ haldLevel }}</div>

            <input class=" range range-sm range-primary" type="range" :data-theme="itemsAccent" step="1" :value="haldLevel"
                :max="16" :min="2" @input="$emit('update:haldLevel', ($event.target as HTMLInputElement).value)">

        </div>

        <div class="mt-2 p-2 bg-skin-mantle rounded-xl border border-skin-surface0">

            <div>Conversion Method</div>

            <div class="flex items-center">

                <div class="flex-grow">Advanced</div>

                <input type="checkbox" class="toggle toggle-primary toggle-sm" :data-theme="itemsAccent"
                    :checked="showAdvancedConversion"
                    @change="$emit('update:showAdvancedConversion', ($event.target as HTMLInputElement).checked)">
            </div>

            <div class="mt-2 border border-skin-surface0 rounded-xl">

                <div class="flex items-center cursor-pointer p-2"
                    @click="$emit('update:conversionMethod', ConversionMethods.gaussian.codename)">

                    <div class="flex-grow font-semibold">
                        {{ ConversionMethods.gaussian.name }}
                    </div>

                    <input :checked="conversionMethod === ConversionMethods.gaussian.codename" type="radio"
                        class="radio radio-sm radio-primary" :data-theme="itemsAccent">
                </div>


                <div v-if="conversionMethod == ConversionMethods.gaussian.codename && showAdvancedConversion">
                    <div class="p-2">

                        <div>Shape - {{ gaussianShape }}</div>

                        <input class="range range-sm range-primary" type="range" :data-theme="itemsAccent" step="1"
                            :value="gaussianShape" :max="ConversionMethods.gaussian.properties.shape.max"
                            :min="ConversionMethods.gaussian.properties.shape.min"
                            @input="$emit('update:gaussianShape', ($event.target as HTMLInputElement).value)">

                        <div>Nearest - {{ gaussianNearest }}</div>

                        <input class="range range-sm range-primary" type="range" :data-theme="itemsAccent" step="1"
                            :value="gaussianNearest" :max="ConversionMethods.gaussian.properties.nearest.max"
                            :min="ConversionMethods.gaussian.properties.nearest.min"
                            @input="$emit('update:gaussianNearest', ($event.target as HTMLInputElement).value)">
                    </div>
                </div>
            </div>

            <div class="mt-2 border border-skin-surface0 rounded-xl">

                <div class="flex items-center cursor-pointer p-2"
                    @click="$emit('update:conversionMethod', ConversionMethods.gaussian_sampling.codename)">

                    <div class="flex-grow font-semibold">
                        {{ ConversionMethods.gaussian_sampling.name }}
                    </div>

                    <input :checked="conversionMethod === ConversionMethods.gaussian_sampling.codename" type="radio"
                        class="radio radio-sm radio-primary" :data-theme="itemsAccent">
                </div>


                <div v-if="conversionMethod == ConversionMethods.gaussian_sampling.codename && showAdvancedConversion">
                    <div class="p-2">

                        <div>Mean - {{ gaussianSamplingMean }}</div>

                        <input class="range range-sm range-primary" type="range" :data-theme="itemsAccent" step="1"
                            :value="gaussianSamplingMean" :max="ConversionMethods.gaussian_sampling.properties.mean.max"
                            :min="ConversionMethods.gaussian_sampling.properties.mean.min"
                            @input="$emit('update:gaussianSamplingMean', ($event.target as HTMLInputElement).value)">

                        <div>STD - {{ gaussianSamplingSTD }}</div>

                        <input class="range range-sm range-primary" type="range" :data-theme="itemsAccent" step="1"
                            :value="gaussianSamplingSTD" :max="ConversionMethods.gaussian_sampling.properties.std.max"
                            :min="ConversionMethods.gaussian_sampling.properties.std.min"
                            @input="$emit('update:gaussianSamplingSTD', ($event.target as HTMLInputElement).value)">

                        <div>Iterations - {{ gaussianSamplingIterations }}</div>

                        <input class="range range-sm range-primary" type="range" :data-theme="itemsAccent" step="1"
                            :value="gaussianSamplingIterations"
                            :max="ConversionMethods.gaussian_sampling.properties.iterations.max"
                            :min="ConversionMethods.gaussian_sampling.properties.iterations.min"
                            @input="$emit('update:gaussianSamplingIterations', ($event.target as HTMLInputElement).value)">
                    </div>
                </div>
            </div>

            <div class="mt-2 border border-skin-surface0 rounded-xl">

                <div class="flex items-center cursor-pointer p-2"
                    @click="$emit('update:conversionMethod', ConversionMethods.linear.codename)">

                    <div class="flex-grow font-semibold">
                        {{ ConversionMethods.linear.name }}
                    </div>

                    <input :checked="conversionMethod === ConversionMethods.linear.codename" type="radio"
                        class="radio radio-sm radio-primary" :data-theme="itemsAccent">
                </div>


                <div v-if="conversionMethod == ConversionMethods.linear.codename && showAdvancedConversion">
                    <div class="p-2">

                        <div>Nearest - {{ linearNearest }}</div>

                        <input class="range range-sm range-primary" type="range" :data-theme="itemsAccent" step="1"
                            :value="linearNearest" :max="ConversionMethods.linear.properties.nearest.max"
                            :min="ConversionMethods.linear.properties.nearest.min"
                            @input="$emit('update:linearNearest', ($event.target as HTMLInputElement).value)">
                    </div>
                </div>
            </div>

            <div class="mt-2 border border-skin-surface0 rounded-xl">

                <div class="flex items-center cursor-pointer p-2"
                    @click="$emit('update:conversionMethod', ConversionMethods.sheppard.codename)">

                    <div class="flex-grow font-semibold">
                        {{ ConversionMethods.sheppard.name }}
                    </div>

                    <input :checked="conversionMethod === ConversionMethods.sheppard.codename" type="radio"
                        class="radio radio-sm radio-primary" :data-theme="itemsAccent">
                </div>


                <div v-if="conversionMethod == ConversionMethods.sheppard.codename && showAdvancedConversion">
                    <div class="p-2">

                        <div>Power - {{ sheppardPower }}</div>

                        <input class="range range-sm range-primary" type="range" :data-theme="itemsAccent" step="1"
                            :value="sheppardPower" :max="ConversionMethods.sheppard.properties.power.max"
                            :min="ConversionMethods.sheppard.properties.power.min"
                            @input="$emit('update:sheppardPower', ($event.target as HTMLInputElement).value)">

                        <div>Nearest - {{ sheppardNearest }}</div>

                        <input class="range range-sm range-primary" type="range" :data-theme="itemsAccent" step="1"
                            :value="sheppardNearest" :max="ConversionMethods.sheppard.properties.nearest.max"
                            :min="ConversionMethods.sheppard.properties.nearest.min"
                            @input="$emit('update:sheppardNearest', ($event.target as HTMLInputElement).value)">
                    </div>
                </div>
            </div>

            <div class="mt-2 border border-skin-surface0 rounded-xl">

                <div class="flex items-center cursor-pointer p-2"
                    @click="$emit('update:conversionMethod', ConversionMethods.nearest_neighbor.codename)">

                    <div class="flex-grow font-semibold">
                        {{ ConversionMethods.nearest_neighbor.name }}
                    </div>

                    <input :checked="conversionMethod === ConversionMethods.nearest_neighbor.codename" type="radio"
                        class="radio radio-sm radio-primary" :data-theme="itemsAccent">
                </div>
            </div>
        </div>

        <div class="mt-4 bg-skin-mantle p-2 rounded-xl border border-skin-surface0">

            <button
                class="bg-skin-crust p-2 rounded-xl border border-skin-surface0 hover:enabled:rounded-full disabled:bg-skin-mantle disabled:border-skin-crust flex justify-center items-center w-full"
                @click="$emit('function:selectImage', '')" :disabled="generatingImages">
                Select Image
            </button>

            <div v-if="selectedImagePath !== ''">

                <div class="flex justify-center mt-2 mb-2 bg-skin-crust rounded-xl p-2 border border-skin-surface0">

                    <img :src="selectedImagePath" class="object-contain h-60 rounded-md">
                </div>

                <button
                    class="bg-skin-crust p-2 rounded-xl border border-skin-surface0 hover:enabled:rounded-full flex justify-center items-center w-full disabled:bg-skin-mantle disabled:border-skin-crust"
                    @click="$emit('function:generateImages')" :disabled="generatingImages">
                    Generate Images
                </button>
            </div>
        </div>

        <div class="flex-grow"></div>

        <div class="flex justify-start mt-4">

            <button
                class="mr-2 mb-2 flex items-center bg-skin-mantle p-2 border border-skin-surface0 rounded-xl hover:rounded-full"
                @click="$router.push('/settings')" :disabled="generatingImages">

                <div>
                    <SettingsSVG class="h-5 w-5 stroke-skin-text fill-transparent mr-1" />
                </div>
                <div>
                    Settings
                </div>
            </button>

            <button
                class="mr-2 mb-2 flex items-center bg-skin-mantle p-2 border border-skin-surface0 rounded-xl hover:rounded-full"
                @click="openLink('https://github.com/lighttigerXIV/catppuccinifier')" :disabled="generatingImages">

                <div>
                    <GitHubSVG class="h-5 w-5 fill-skin-text mr-1" />
                </div>
                <div>
                    About
                </div>
            </button>
        </div>
    </div>
</template>
