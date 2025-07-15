<script lang="ts">
	import { settingsRepository } from "$lib/repositories/settings-repository";
	import { mainRepository, Tab } from "./main-repository";
	import { onMount } from "svelte";
	import Navbar from "./components/Navbar.svelte";
	import GeneratePage from "./pages/generate/GeneratePage.svelte";
	import PropertiesPage from "./pages/properties/PropertiesPage.svelte";
	import SettingsPage from "./pages/settings/SettingsPage.svelte";
	import { listen } from "@tauri-apps/api/event";

	let settingsLoaded = $state(false);
	let css = $state("");

	onMount(async () => {
		$settingsRepository.init(() => {
			css = $settingsRepository.getStyle();
			settingsLoaded = true;
		});

		listen("settings-saved", (_event) => {
			css = $settingsRepository.getStyle();
		});
	});
</script>

{#if settingsLoaded}
	{@html css}
	<div class="h-screen bg-n2 t0 flex w-full">
		<Navbar
			tab={$mainRepository.tab}
			onclick={(tab) => {
				$mainRepository.tab = tab;
			}}
		/>
		<div class="flex-grow p-5 overflow-auto">
			{#if $mainRepository.tab === Tab.Generate}
				<GeneratePage />
			{/if}

			{#if $mainRepository.tab === Tab.Properties}
				<PropertiesPage />
			{/if}

			{#if $mainRepository.tab === Tab.Settings}
				<SettingsPage />
			{/if}
		</div>
	</div>
{/if}
