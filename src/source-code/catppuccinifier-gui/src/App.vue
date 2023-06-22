<script lang="ts">
import { defineComponent, ref, watch } from 'vue';
import { Themes, Accents, ConversionMethods } from "./Constants"
import GenerateImageSection from "./components/GenerateImageSection.vue"
import GeneratedImageBox from './components/GeneratedImageBox.vue';
import { open, save } from "@tauri-apps/api/dialog"
import { convertFileSrc, invoke } from '@tauri-apps/api/tauri';
import { WebviewWindow } from "@tauri-apps/api/window"
import MenuSVG from "./assets/images/menu.svg"
import { appLocalDataDir } from '@tauri-apps/api/path';

export default defineComponent({
  name: "App",
  components: {
    GenerateImageSection,
    GeneratedImageBox
  },
  setup() {

    const theme = ref(localStorage.getItem("theme") ?? Themes.mocha);
    const accent = ref(localStorage.getItem("accent") ?? Accents.blue);
    const tailwindTheme = ref("Theme-" + theme.value);
    const itemsAccent = ref(theme.value.toLowerCase() + "-" + accent.value.toLowerCase());
    const haldLevel = ref(8);
    const conversionMethod = ref(ConversionMethods.gaussian.codename);
    const showAdvancedConversion = ref(false);
    const gaussianEuclide = ref(ConversionMethods.gaussian.properties.euclide.default);
    const gaussianNearest = ref(ConversionMethods.gaussian.properties.nearest.default);
    const gaussianSamplingMean = ref(ConversionMethods.gaussian_sampling.properties.mean.default);
    const gaussianSamplingSTD = ref(ConversionMethods.gaussian_sampling.properties.std.default);
    const gaussianSamplingIterations = ref(ConversionMethods.gaussian_sampling.properties.iterations.default);
    const linearNearest = ref(ConversionMethods.linear.properties.nearest.default);
    const sheppardPower = ref(ConversionMethods.sheppard.properties.power.default);
    const sheppardNearest = ref(ConversionMethods.sheppard.properties.nearest.default);
    const selectedImageRawPath = ref("");
    const selectedImagePath = ref("");
    const showGeneratedGrids = ref(false);
    const generatedLatteRawPath = ref("");
    const generatedLattePath = ref("");
    const generatedFrappeRawPath = ref("");
    const generatedFrappePath = ref("");
    const generatedMacchiatoRawPath = ref("");
    const generatedMacchiatoPath = ref("");
    const generatedMochaRawPath = ref("");
    const generatedMochaPath = ref("");
    const generatedOledRawPath = ref("");
    const generatedOledPath = ref("");
    const generatingImages = ref(false)
    const showSettings = ref(false);
    const showSideNav = ref(false);
    const dir = ref("");

    function updateHaldLevel(v: number) { haldLevel.value = v }
    function updateConversionMethod(v: string) { conversionMethod.value = v }
    function updateShowAdvancedConversion(v: boolean) { showAdvancedConversion.value = v }
    function updateGaussianEuclide(v: number) { gaussianEuclide.value = v }
    function updateGaussianNearest(v: number) { gaussianNearest.value = v }
    function updateGaussianSamplingMean(v: number) { gaussianSamplingMean.value = v }
    function updateGaussianSamplingSTD(v: number) { gaussianSamplingSTD.value = v }
    function updateGaussianSamplingIterations(v: number) { gaussianSamplingIterations.value = v }
    function updateLinearNearest(v: number) { linearNearest.value = v }
    function updateSheppardPower(v: number) { sheppardPower.value = v }
    function updateSheppardNearest(v: number) { sheppardNearest.value = v }
    function updateShowSideNav(v: boolean) { showSideNav.value = v }


    function getThemesSettingClasses(setting: string) {

      if (setting == theme.value) {
        return "bg-skin-base border-skin-surface1"
      } else {
        return "bg-skin-mantle border-skin-surface0"
      }
    }

    function getAccentSettingClasses(setting: string) {

      if (setting == accent.value) {
        return "bg-skin-base border-skin-surface1 col-span-1"
      } else {
        return "bg-skin-mantle border-skin-surface0 col-span-1"
      }
    }

    function getRadioColor() {

      return "checked:bg-skin-" + accent.value
    }

    watch(showAdvancedConversion, () => {
      if (!showAdvancedConversion.value) {
        //Resets Advanced Values
        gaussianEuclide.value = ConversionMethods.gaussian.properties.euclide.default;
        gaussianNearest.value = ConversionMethods.gaussian.properties.nearest.default;
        gaussianSamplingMean.value = ConversionMethods.gaussian_sampling.properties.mean.default;
        gaussianSamplingSTD.value = ConversionMethods.gaussian_sampling.properties.std.default;
        gaussianSamplingIterations.value = ConversionMethods.gaussian_sampling.properties.iterations.default;
        linearNearest.value = ConversionMethods.linear.properties.nearest.default;
        sheppardPower.value = ConversionMethods.sheppard.properties.power.default;
        sheppardNearest.value = ConversionMethods.sheppard.properties.nearest.default;
      }
    })

    async function selectImage() {

      const selected = await open({
        multiple: false,
        filters: [{
          name: "Images",
          extensions: ["png", "webp", "jpg", "jpeg"]
        }]
      });

      if (selected !== null && typeof (selected) === "string") {
        selectedImageRawPath.value = selected;
        selectedImagePath.value = convertFileSrc(selected);
        generatedLattePath.value = ""
        generatedFrappePath.value = ""
        generatedMacchiatoPath.value = ""
        generatedMochaPath.value = ""
        generatedOledPath.value = ""
        showGeneratedGrids.value = false
      }
    }

    async function generateImages() {

      generatedLattePath.value = "";
      generatedFrappePath.value = "";
      generatedMacchiatoPath.value = "";
      generatedMochaPath.value = "";
      generatedOledPath.value = "";
      generatingImages.value = true;
      showGeneratedGrids.value = true

      await invoke("clear_temp_folder")
        .catch(error => {
          console.error(error);
        });

      await invoke("generate_image", {
        image_path: selectedImageRawPath.value,
        hald_level: +haldLevel.value,
        flavor: "latte",
        conversion_method: conversionMethod.value,
        gaussian_euclide: +gaussianEuclide.value,
        gaussian_nearest: +gaussianNearest.value,
        gaussian_sampling_mean: +gaussianSamplingMean.value,
        gaussian_sampling_std: +gaussianSamplingSTD.value,
        gaussian_sampling_iterations: +gaussianSamplingIterations.value,
        linear_nearest: +linearNearest.value,
        sheppard_power: +sheppardPower.value,
        sheppard_nearest: +sheppardNearest.value
      })
        .then(path => {

          generatedLatteRawPath.value = path as string;
          generatedLattePath.value = convertFileSrc(path as string);
        })
        .catch((error) => { console.error(error) });

      await invoke("generate_image", {
        image_path: selectedImageRawPath.value,
        hald_level: +haldLevel.value,
        flavor: "frappe",
        conversion_method: conversionMethod.value,
        gaussian_euclide: +gaussianEuclide.value,
        gaussian_nearest: +gaussianNearest.value,
        gaussian_sampling_mean: +gaussianSamplingMean.value,
        gaussian_sampling_std: +gaussianSamplingSTD.value,
        gaussian_sampling_iterations: +gaussianSamplingIterations.value,
        linear_nearest: +linearNearest.value,
        sheppard_power: +sheppardPower.value,
        sheppard_nearest: +sheppardNearest.value
      })
        .then(path => {

          generatedFrappeRawPath.value = path as string;
          generatedFrappePath.value = convertFileSrc(path as string);
        })
        .catch((error) => { console.error(error) });

      await invoke("generate_image", {
        image_path: selectedImageRawPath.value,
        hald_level: +haldLevel.value,
        flavor: "macchiato",
        conversion_method: conversionMethod.value,
        gaussian_euclide: +gaussianEuclide.value,
        gaussian_nearest: +gaussianNearest.value,
        gaussian_sampling_mean: +gaussianSamplingMean.value,
        gaussian_sampling_std: +gaussianSamplingSTD.value,
        gaussian_sampling_iterations: +gaussianSamplingIterations.value,
        linear_nearest: +linearNearest.value,
        sheppard_power: +sheppardPower.value,
        sheppard_nearest: +sheppardNearest.value
      })
        .then(path => {

          generatedMacchiatoRawPath.value = path as string;
          generatedMacchiatoPath.value = convertFileSrc(path as string);
        })
        .catch((error) => { console.error(error) });

      await invoke("generate_image", {
        image_path: selectedImageRawPath.value,
        hald_level: +haldLevel.value,
        flavor: "mocha",
        conversion_method: conversionMethod.value,
        gaussian_euclide: +gaussianEuclide.value,
        gaussian_nearest: +gaussianNearest.value,
        gaussian_sampling_mean: +gaussianSamplingMean.value,
        gaussian_sampling_std: +gaussianSamplingSTD.value,
        gaussian_sampling_iterations: +gaussianSamplingIterations.value,
        linear_nearest: +linearNearest.value,
        sheppard_power: +sheppardPower.value,
        sheppard_nearest: +sheppardNearest.value
      })
        .then(path => {

          generatedMochaRawPath.value = path as string;
          generatedMochaPath.value = convertFileSrc(path as string);
        })
        .catch((error) => { console.error(error) });

      await invoke("generate_image", {
        image_path: selectedImageRawPath.value,
        hald_level: +haldLevel.value,
        flavor: "oled",
        conversion_method: conversionMethod.value,
        gaussian_euclide: +gaussianEuclide.value,
        gaussian_nearest: +gaussianNearest.value,
        gaussian_sampling_mean: +gaussianSamplingMean.value,
        gaussian_sampling_std: +gaussianSamplingSTD.value,
        gaussian_sampling_iterations: +gaussianSamplingIterations.value,
        linear_nearest: +linearNearest.value,
        sheppard_power: +sheppardPower.value,
        sheppard_nearest: +sheppardNearest.value
      })
        .then(path => {

          generatedOledRawPath.value = path as string;
          generatedOledPath.value = convertFileSrc(path as string)
        })
        .catch((error) => { console.error(error) });

      generatingImages.value = false;
    }

    function saveImage(flavor: string) {
      var fileName: string = "";

      invoke("get_os")
        .then((os) => {

          if (os === "windows") {
            fileName = flavor + "-hald" + haldLevel.value + "-" + selectedImageRawPath.value.split('\\').pop();
          } else {
            fileName = flavor + "-hald" + haldLevel.value + "-" + selectedImageRawPath.value.split('/').pop();
          }

          save({
            defaultPath: fileName,
            filters: [
              {
                name: "Images (png, jpg, jpeg, webp)",
                extensions: ["png", "webp", "jpg", "jpeg"]
              }
            ]
          }).then((savedPath) => {

            switch (flavor) {
              case "latte": {
                invoke("save_image", { image_path: generatedLatteRawPath.value, saved_path: savedPath })
                break;
              }
              case "frappe": {
                invoke("save_image", { image_path: generatedFrappeRawPath.value, saved_path: savedPath })
                break;
              }
              case "macchiato": {
                invoke("save_image", { image_path: generatedMacchiatoRawPath.value, saved_path: savedPath })
                break;
              }
              case "mocha": {
                invoke("save_image", { image_path: generatedMochaRawPath.value, saved_path: savedPath })
                break;
              }
              case "oled": {
                invoke("save_image", { image_path: generatedOledRawPath.value, saved_path: savedPath })
                break;
              }
            }
          })
        })
    }

    function previewImage(flavor: string, path: string) {
      const webview = new WebviewWindow('preview', {
        url: 'preview.html?path=' + path as string,
        title: flavor as string + " Preview",
      })

      webview.once('tauri://created', function () { })
      webview.once('tauri://error', function (e) {
        console.error(e);
      })

    }

    async function getAppDir(){
      dir.value = await appLocalDataDir();
    }

    return {
      theme, accent, tailwindTheme, itemsAccent, haldLevel, conversionMethod, showAdvancedConversion, gaussianEuclide, gaussianNearest, gaussianSamplingMean, gaussianSamplingSTD,
      gaussianSamplingIterations, linearNearest, sheppardPower, sheppardNearest, selectedImageRawPath, selectedImagePath, showGeneratedGrids,
      generatedLatteRawPath, generatedLattePath, generatedFrappeRawPath, generatedFrappePath, generatedMacchiatoRawPath, generatedMacchiatoPath,
      generatedMochaRawPath, generatedMochaPath, generatedOledRawPath, generatedOledPath, generatingImages, showSettings, showSideNav,
      getThemesSettingClasses, getAccentSettingClasses, getRadioColor, updateHaldLevel, updateConversionMethod, updateShowAdvancedConversion, updateGaussianEuclide,
      updateGaussianNearest, updateGaussianSamplingMean, updateGaussianSamplingSTD, updateGaussianSamplingIterations, updateLinearNearest,
      updateSheppardNearest, updateSheppardPower, selectImage, generateImages, updateShowSideNav, saveImage, previewImage, getAppDir, dir
    }
  }
});
</script>

<template>
  <div class='h-screen grid grid-cols-12 min-w-full w-full bg-skin-crust text-skin-text'
    :class="tailwindTheme">

    <div>{{ dir }}</div>
    <div class="hidden">{{ getAppDir() }}</div>

    <div
      class=" 2xl:col-span-3 xl:col-span-3 lg:col-span-5 md:col-span-6 2xl:flex xl:flex lg:flex md:flex  hidden h-screen max-h-screen overflow-auto">

      <GenerateImageSection :items-accent="itemsAccent" v-model:hald-level="haldLevel" @update:haldLevel="updateHaldLevel"
        v-model:conversion-method="conversionMethod" @update:conversion-method="updateConversionMethod"
        v-model:show-advanced-conversion="showAdvancedConversion"
        @update:show-advanced-conversion="updateShowAdvancedConversion" v-model:gaussian-euclide="gaussianEuclide"
        @update:gaussian-euclide="updateGaussianEuclide" v-model:gaussian-nearest="gaussianNearest"
        @update:gaussian-nearest="updateGaussianNearest" v-model:gaussian-sampling-mean="gaussianSamplingMean"
        @update:gaussian-sampling-mean="updateGaussianSamplingMean" v-model:gaussian-sampling-s-t-d="gaussianSamplingSTD"
        @update:gaussian-sampling-s-t-d="updateGaussianSamplingSTD"
        v-model:gaussian-sampling-iterations="gaussianSamplingIterations"
        @update:gaussian-sampling-iterations="updateGaussianSamplingIterations" v-model:linear-nearest="linearNearest"
        @update:linear-nearest="updateLinearNearest" v-model:sheppard-power="sheppardPower"
        @update:sheppard-power="updateSheppardPower" v-model:sheppard-nearest="sheppardNearest"
        @update:sheppard-nearest="updateSheppardNearest" :accent="accent" :generating-images="generatingImages"
        @function:select-image="selectImage" :selected-image-path="selectedImagePath"
        @function:generate-images="generateImages" />
    </div>

    <div class="2xl:hidden xl:hidden lg:hidden md:hidden md-w-60 flex p-4">

      <button class=" hover:bg-skin-surface0 p-1 rounded-full" @click="updateShowSideNav(true)">

        <MenuSVG class="w-5 h-5 fill-skin-text" />
      </button>
    </div>

    <div v-if="showSideNav"
      class="h-screen w-screen absolute flex flex-col 2xl:hidden xl:hidden lg:hidden md:hidden md-w-60">

      <div
        class="w-5/6 bg-skin-crust flex-grow flex flex-col rounded-r-2xl border-r border-t border-b border-skin-surface0 overflow-auto">

        <div class="flex justify-end mt-4 mr-4">

          <button class="hover:bg-skin-surface0 rounded-full p-1 mb-2" @click="updateShowSideNav(false)">

            <CloseSVG class="w-5 h-5 fill-skin-text" />
          </button>
        </div>
      </div>
    </div>

    <div class="2xl:col-span-9 xl:col-span-9 lg:col-span-7 md:col-span-6  col-span-12 flex overflow-auto">

      <div
        class="bg-skin-base flex-grow 2xl:rounded-l-2xl xl:rounded-l-2xl lg:rounded-l-2xl md:rounded-l-2xl h-screen overflow-auto ">

        <div v-if="showGeneratedGrids">

          <div class="p-3">

            <div class="grid 2xl:grid-cols-3 xl:grid-cols-3 lg:grid-cols-1 md:grid-cols-1 grid-cols-1 gap-2">

              <div>

                <GeneratedImageBox :items-accent="itemsAccent" :disabled="generatingImages" flavor="latte"
                  :image-path="generatedLattePath" @function:preview-image="previewImage('latte', generatedLatteRawPath)"
                  @function:save-image="saveImage('latte')" />
              </div>

              <div>

                <GeneratedImageBox :items-accent="itemsAccent" :disabled="generatingImages" flavor="frappe"
                  :image-path="generatedFrappePath"
                  @function:preview-image="previewImage('frappe', generatedLatteRawPath)"
                  @function:save-image="saveImage('frappe')" />
              </div>

              <div>

                <GeneratedImageBox :items-accent="itemsAccent" :disabled="generatingImages" flavor="macchiato"
                  :image-path="generatedMacchiatoPath"
                  @function:preview-image="previewImage('macchiato', generatedLatteRawPath)"
                  @function:save-image="saveImage('macchiato')" />
              </div>

              <div>
                <GeneratedImageBox :items-accent="itemsAccent" :disabled="generatingImages" flavor="mocha"
                  :image-path="generatedMochaPath" @function:preview-image="previewImage('mocha', generatedLatteRawPath)"
                  @function:save-image="saveImage('mocha')" />
              </div>

              <div>
                <GeneratedImageBox :items-accent="itemsAccent" :disabled="generatingImages" flavor="oled"
                  :image-path="generatedOledPath" @function:preview-image="previewImage('oled', generatedLatteRawPath)"
                  @function:save-image="saveImage('oled')" />
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
