<script setup lang="ts">
import { Accents, Themes } from "../Constants";
import CloseSVG from "../assets/images/close.svg"
import SunflowerSVG from "../assets/images/sunflower.svg"
import PineappleSVG from "../assets/images/pineapple.svg"
import LotusSVG from "../assets/images/lotus.svg"
import BranchSVG from "../assets/images/branch.svg"
import { ref } from "vue";

const theme = ref(localStorage.getItem("theme") ?? Themes.mocha);
const accent = ref(localStorage.getItem("accent") ?? Accents.blue);
const tailwindTheme = ref("Theme-" + theme.value);


function themesSettingClasses(setting: string) {

    if (setting === theme.value) {
        return "bg-skin-base border-skin-surface1"
    } else {
        return "bg-skin-mantle border-skin-surface0"
    }
}

function accentSettingClasses(setting: string) {

    if (setting === accent.value) {
        return "bg-skin-base border-skin-surface1 col-span-1"
    } else {
        return "bg-skin-mantle border-skin-surface0 col-span-1"
    }
}

function updateTheme(v: string) {
    localStorage.setItem("theme", v);
    theme.value = v;
    tailwindTheme.value = "Theme-" + theme.value;
}

function updateAccent(v: string) {
    localStorage.setItem("accent", v);
    accent.value = v;
}
</script>

<template>
    <div v-bind:class="tailwindTheme">
        <div
            class="h-screen w-screen bg-skin-base flex items-start justify-center max-h-screen overflow-auto text-skin-text">

            <div class="p-4 bg-skin-crust m-4 rounded-2xl">

                <div class="flex justify-end">

                    <div class="rounded-full hover:bg-skin-surface0 cursor-pointer" @click="$router.back()">

                        <CloseSVG class="w-5 h-5 fill-skin-text m-1" />
                    </div>
                </div>

                <div className="text-lg">Theme</div>

                <div class="grid 2xl:grid-cols-4 xl:grid-cols-4 lg:grid-cols-4 md:grid-cols-2 grid-cols-2 gap-2">

                    <div class="cursor-pointer p-2 rounded-xl flex items-center justify-center border hover:bg-skin-base"
                        :class="themesSettingClasses(Themes.latte)" @click="updateTheme(Themes.latte)">

                        <SunflowerSVG class=" w-5 h-5 fill-skin-text" />

                        <div class="ml-2 truncate">
                            Latte
                        </div>
                    </div>

                    <div class="cursor-pointer p-2 rounded-xl flex items-center justify-center border hover:bg-skin-base"
                        :class="themesSettingClasses(Themes.frappe)" @click="updateTheme(Themes.frappe)">

                        <PineappleSVG class=" w-5 h-5 fill-skin-text" />

                        <div class="ml-2 truncate">
                            Frappe
                        </div>
                    </div>

                    <div class="cursor-pointer p-2 rounded-xl flex items-center justify-center border hover:bg-skin-base"
                        :class="themesSettingClasses(Themes.macchiato)" @click="updateTheme(Themes.macchiato)">

                        <LotusSVG class=" w-5 h-5 fill-skin-text" />

                        <div class="ml-2 truncate">
                            macchiato
                        </div>
                    </div>

                    <div class="cursor-pointer p-2 rounded-xl flex items-center justify-center border hover:bg-skin-base"
                        :class="themesSettingClasses(Themes.mocha)" @click="updateTheme(Themes.mocha)">

                        <BranchSVG class=" w-5 h-5 fill-skin-text" />

                        <div class="ml-2 truncate">
                            Mocha
                        </div>
                    </div>
                </div>

                <div class="mt-2 text-lg">
                    Accent
                </div>

                <div class="grid 2xl:grid-cols-4 xl:grid-cols-4 lg:grid-cols-4 md:grid-cols-4 grid-cols-2 gap-2">

                    <div class="p-2 rounded-xl border hover:bg-skin-base cursor-pointer flex items-center justify-center"
                        :class="accentSettingClasses(Accents.rosewater)" @click="updateAccent(Accents.rosewater)">

                        <div class="text-skin-rosewater">Rosewater</div>
                    </div>

                    <div class="p-2 rounded-xl border hover:bg-skin-base cursor-pointer flex items-center justify-center"
                        :class="accentSettingClasses(Accents.flamingo)" @click="updateAccent(Accents.flamingo)">

                        <div class="text-skin-flamingo">Flamingo</div>
                    </div>

                    <div class="p-2 rounded-xl border hover:bg-skin-base cursor-pointer flex items-center justify-center"
                        :class="accentSettingClasses(Accents.pink)" @click="updateAccent(Accents.pink)">

                        <div class="text-skin-pink">Pink</div>
                    </div>

                    <div class="p-2 rounded-xl border hover:bg-skin-base cursor-pointer flex items-center justify-center"
                        :class="accentSettingClasses(Accents.mauve)" @click="updateAccent(Accents.mauve)">

                        <div class="text-skin-mauve">Mauve</div>
                    </div>

                    <div class="p-2 rounded-xl border hover:bg-skin-base cursor-pointer flex items-center justify-center"
                        :class="accentSettingClasses(Accents.red)" @click="updateAccent(Accents.red)">

                        <div class="text-skin-red">Red</div>
                    </div>

                    <div class="p-2 rounded-xl border hover:bg-skin-base cursor-pointer flex items-center justify-center"
                        :class="accentSettingClasses(Accents.maroon)" @click="updateAccent(Accents.maroon)">

                        <div class="text-skin-maroon">Maroon</div>
                    </div>

                    <div class="p-2 rounded-xl border hover:bg-skin-base cursor-pointer flex items-center justify-center"
                        :class="accentSettingClasses(Accents.peach)" @click="updateAccent(Accents.peach)">

                        <div class="text-skin-peach">Peach</div>
                    </div>

                    <div class="p-2 rounded-xl border hover:bg-skin-base cursor-pointer flex items-center justify-center"
                        :class="accentSettingClasses(Accents.yellow)" @click="updateAccent(Accents.yellow)">

                        <div class="text-skin-yellow">Yellow</div>
                    </div>

                    <div class="p-2 rounded-xl border hover:bg-skin-base cursor-pointer flex items-center justify-center"
                        :class="accentSettingClasses(Accents.green)" @click="updateAccent(Accents.green)">

                        <div class="text-skin-green">Green</div>
                    </div>

                    <div class="p-2 rounded-xl border hover:bg-skin-base cursor-pointer flex items-center justify-center"
                        :class="accentSettingClasses(Accents.teal)" @click="updateAccent(Accents.teal)">

                        <div class="text-skin-teal">Teal</div>
                    </div>

                    <div class="p-2 rounded-xl border hover:bg-skin-base cursor-pointer flex items-center justify-center"
                        :class="accentSettingClasses(Accents.sky)" @click="updateAccent(Accents.sky)">

                        <div class="text-skin-sky">Sky</div>
                    </div>

                    <div class="p-2 rounded-xl border hover:bg-skin-base cursor-pointer flex items-center justify-center"
                        :class="accentSettingClasses(Accents.sapphire)" @click="updateAccent(Accents.sapphire)">

                        <div class="text-skin-sapphire">Sapphire</div>
                    </div>

                    <div class="p-2 rounded-xl border hover:bg-skin-base cursor-pointer flex items-center justify-center"
                        :class="accentSettingClasses(Accents.blue)" @click="updateAccent(Accents.blue)">

                        <div class="text-skin-blue">Blue</div>
                    </div>

                    <div class="p-2 rounded-xl border hover:bg-skin-base cursor-pointer flex items-center justify-center"
                        :class="accentSettingClasses(Accents.lavender)" @click="updateAccent(Accents.lavender)">

                        <div class="text-skin-lavender">Lavender</div>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>