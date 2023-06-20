<script setup lang="ts">
import { useRoute } from 'vue-router';
import { Themes } from '../Constants';


const theme = localStorage.getItem("theme") ?? Themes.mocha;
const tailwindTheme = "Theme-" + theme;
const imagePath = useRoute().query.path?.toString();

</script>

<template>
    <div class="h-screen min-w-full w-full bg-skin-base text-text p-4 flex justify-center" :class="tailwindTheme">
        <img
            class="object-contain rounded-md h-full"
            :src="imagePath"
        >
    </div>
</template>