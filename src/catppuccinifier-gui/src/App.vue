<script setup lang="ts">
import { ref, watch } from 'vue';
import { Algorithms, GenerateImagesData as GenerateImagesData, GenerationData, ImagePaths } from "@/data"
import GenerateImageSection from "@components/GenerateImageSection.vue"
import GeneratedImagesSection from "@components/GeneratedImagesSection.vue"
import { convertFileSrc, invoke } from '@tauri-apps/api/tauri';

const hald = ref(8);
const showAdvancedConversion = ref(false);
const gaussianShape = ref(Algorithms.gaussian.properties.shape.default);
const gaussianNearest = ref(Algorithms.gaussian.properties.nearest.default);
const gaussianSamplingMean = ref(Algorithms.gaussian_sampling.properties.mean.default);
const gaussianSamplingSTD = ref(Algorithms.gaussian_sampling.properties.std.default);
const gaussianSamplingIterations = ref(Algorithms.gaussian_sampling.properties.iterations.default);
const linearNearest = ref(Algorithms.linear.properties.nearest.default);
const sheppardPower = ref(Algorithms.sheppard.properties.power.default);
const sheppardNearest = ref(Algorithms.sheppard.properties.nearest.default);
const showImagesGrid = ref(false);
const generatingImages = ref(false);
const flavorsToGenerate = ref<string[]>([])
const imagePaths = ref<ImagePaths>({
  selected: {
    local: "",
    raw: ""
  },
  latte: {
    local: "",
    raw: ""
  },
  frappe: {
    local: "",
    raw: ""
  },
  macchiato: {
    local: "",
    raw: ""
  },
  mocha: {
    local: "",
    raw: ""
  },
  oled: {
    local: "",
    raw: ""
  }
});





watch(showAdvancedConversion, () => {
  if (!showAdvancedConversion.value) {
    //Resets Advanced Values
    gaussianShape.value = Algorithms.gaussian.properties.shape.default;
    gaussianNearest.value = Algorithms.gaussian.properties.nearest.default;
    gaussianSamplingMean.value = Algorithms.gaussian_sampling.properties.mean.default;
    gaussianSamplingSTD.value = Algorithms.gaussian_sampling.properties.std.default;
    gaussianSamplingIterations.value = Algorithms.gaussian_sampling.properties.iterations.default;
    linearNearest.value = Algorithms.linear.properties.nearest.default;
    sheppardPower.value = Algorithms.sheppard.properties.power.default;
    sheppardNearest.value = Algorithms.sheppard.properties.nearest.default;
  }
})

function clearGeneratedImagePaths() {
  imagePaths.value.latte.local = ""
  imagePaths.value.latte.raw = ""
  imagePaths.value.frappe.local = ""
  imagePaths.value.frappe.raw = ""
  imagePaths.value.macchiato.local = ""
  imagePaths.value.macchiato.raw = ""
  imagePaths.value.mocha.local = ""
  imagePaths.value.mocha.raw = ""
  imagePaths.value.oled.local = ""
  imagePaths.value.oled.raw = ""
}

async function generateImages(generateImagesData: GenerateImagesData, flavors: string[]) {

  clearGeneratedImagePaths()
  generatingImages.value = true;
  showImagesGrid.value = true
  flavorsToGenerate.value = flavors;
  hald.value = generateImagesData.hald;

  let generationData: GenerationData = {
    image_path: generateImagesData.imagePath,
    hald: generateImagesData.hald,
    luminosity: generateImagesData.luminosity,
    algorithm: generateImagesData.algorithm,
    shape: generateImagesData.shape,
    nearest: generateImagesData.nearest,
    mean: generateImagesData.mean,
    std: generateImagesData.std,
    iterations: generateImagesData.iterations,
    power: generateImagesData.power,
    flavor: ""
  }

  if (flavors.includes("latte")) {
    generationData.flavor = "latte";
    let path: string = await invoke("generate_image", { data: generationData });
    let localPath = `${convertFileSrc(path)}?${Math.random()}`;
    imagePaths.value.latte.raw = path;
    imagePaths.value.latte.local = localPath;
  }

  if (flavors.includes("frappe")) {
    generationData.flavor = "frappe";
    let path: string = await invoke("generate_image", { data: generationData });
    let localPath = `${convertFileSrc(path)}?${Math.random()}`;
    imagePaths.value.frappe.raw = path;
    imagePaths.value.frappe.local = localPath;
  }

  if (flavors.includes("macchiato")) {
    generationData.flavor = "macchiato";
    let path: string = await invoke("generate_image", { data: generationData });
    let localPath = `${convertFileSrc(path)}?${Math.random()}`;
    imagePaths.value.macchiato.raw = path;
    imagePaths.value.macchiato.local = localPath;
  }

  if (flavors.includes("mocha")) {
    generationData.flavor = "mocha";
    let path: string = await invoke("generate_image", { data: generationData });
    let localPath = `${convertFileSrc(path)}?${Math.random()}`;
    imagePaths.value.mocha.raw = path;
    imagePaths.value.mocha.local = localPath;
  }

  if (flavors.includes("oled")) {
    generationData.flavor = "oled";
    let path: string = await invoke("generate_image", { data: generationData });
    let localPath = `${convertFileSrc(path)}?${Math.random()}`;
    imagePaths.value.oled.raw = path;
    imagePaths.value.oled.local = localPath;
  }

  generatingImages.value = false;
}



</script>

<template>
  <div class="flex">
    <GenerateImageSection :image-paths="imagePaths" @generate="generateImages($event.data, $event.flavors)"
      :generating-images="generatingImages" @update-paths="imagePaths = $event" />

    <GeneratedImagesSection :image-paths="imagePaths" :show-images-grids="showImagesGrid" :generating-images="generatingImages"
      @update-paths="imagePaths = $event" :flavors="flavorsToGenerate" :hald="hald"/>
  </div>
</template>