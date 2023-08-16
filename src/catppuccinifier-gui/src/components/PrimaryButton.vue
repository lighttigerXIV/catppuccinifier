<script setup lang="ts">
import WandSVG from "@icons/wand.svg"
import FolderSVG from "@icons/folder.svg"
import SettingsSVG from "@icons/settings.svg"
import GitHubSVG from "@icons/github.svg"
import SaveSVG from "@icons/save.svg"
import MaximizeSVG from "@icons/maximize.svg"

const emit = defineEmits([
    "click"
])

const props = defineProps({
    text: {
        required: true,
        type: String
    },
    icon: {
        type: String
    },
    expand: {
        type: Boolean,
        default: false
    },
    disabled:{
        type: Boolean,
        default: false
    }
})

const hasIcon = props.icon !== undefined;

</script>

<template>
    <button @click="emit('click')" :disabled="disabled"
        class="p-2 pr-4 pl-4 bg-skin-base disabled:bg-skin-crust hover:enabled:opacity-80 rounded-full border border-skin-surface0" :class="expand ? 'w-full' : ''">
        <div class="flex items-center justify-center">
            <div v-if="hasIcon" class="mr-2">
                <WandSVG v-if="icon === 'wand'" class="h-5 w-5 stroke-skin-text" />
                <FolderSVG v-if="icon === 'folder'" class="h-5 w-5 stroke-skin-text"/>
                <SettingsSVG v-if="icon === 'settings'" class="h-5 w-5 fill-none stroke-skin-text"/>
                <GitHubSVG v-if="icon === 'github'" class="h-5 w-5 fill-skin-text"/>
                <SaveSVG v-if="icon === 'save'" class="h-5 w-5 stroke-skin-text fill-none"/>
                <MaximizeSVG v-if="icon === 'maximize'" class="h-5 w-5 stroke-skin-text fill-none" />
            </div>
            <div>
                {{ text }}
            </div>
        </div>
    </button>
</template>