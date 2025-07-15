<script lang="ts">
	import { settingsRepository } from "$lib/repositories/settings-repository";
	import { onMount } from "svelte";
	import { listen } from "@tauri-apps/api/event";
	import { page } from "$app/state";
	import { convertFileSrc } from "@tauri-apps/api/core";

	let settingsLoaded = $state(false);
	let css = $state("");
	let file = $derived(page.params.file);

	onMount(async () => {
		$settingsRepository.init(() => {
			css = $settingsRepository.getStyle();
			settingsLoaded = true;
		});
	});
</script>

{#if settingsLoaded}
	{@html css}
	<div class="h-screen bg-n2 t0 w-full">
		<img
			class="object-contain h-screen w-full"
			alt=""
			src={convertFileSrc(file)}
		/>
	</div>
{/if}
