<script lang="ts">
	import { Accent, Theme } from "$lib/models/Settings";
	import { SettingsPageVM } from "./settings-page-vm";

	const vm = new SettingsPageVM();
	const state = vm.state;
</script>

<h2 class="text-lg font-medium">Flavor</h2>

<div class="rounded-full space-x-1 bg-n1 flex w-fit mt-2">
	<button
		class={`p-4 cursor-pointer tab-begin hover-bg-accent hover-text-n0 ${$state.theme === Theme.Mocha ? "bg-accent text-n0" : ""}`}
		onclick={() => {
			vm.onAction({
				action: "theme-click",
				theme: Theme.Mocha,
			});
		}}>Mocha</button
	>

	<button
		class={`p-4 cursor-pointer tab-middle hover-bg-accent hover-text-n0 ${$state.theme === Theme.Macchiato ? "bg-accent text-n0" : ""}`}
		onclick={() => {
			vm.onAction({
				action: "theme-click",
				theme: Theme.Macchiato,
			});
		}}>Macchiato</button
	>

	<button
		class={`p-4 cursor-pointer tab-middle hover-bg-accent hover-text-n0 ${$state.theme === Theme.Frappe ? "bg-accent text-n0" : ""}`}
		onclick={() => {
			vm.onAction({
				action: "theme-click",
				theme: Theme.Frappe,
			});
		}}>Frappe</button
	>

	<button
		class={`p-4 cursor-pointer tab-end hover-bg-accent hover-text-n0 ${$state.theme === Theme.Latte ? "bg-accent text-n0" : ""}`}
		onclick={() => {
			vm.onAction({
				action: "theme-click",
				theme: Theme.Latte,
			});
		}}>Latte</button
	>
</div>

<h2 class="text-lg font-medium mt-4">Accent</h2>

<div class="rounded-full space-x-1 bg-n1 flex w-fit mt-2">
	<button
		class={`p-4 cursor-pointer tab-begin hover-bg-accent hover-text-n0 ${$state.accent === Accent.Rosewater ? "bg-accent text-n0" : ""}`}
		onclick={() => {
			vm.onAction({
				action: "accent-click",
				accent: Accent.Rosewater,
			});
		}}>Rosewater</button
	>

	<button
		class={`p-4 cursor-pointer tab-middle hover-bg-accent hover-text-n0 ${$state.accent === Accent.Flamingo ? "bg-accent text-n0" : ""}`}
		onclick={() => {
			vm.onAction({
				action: "accent-click",
				accent: Accent.Flamingo,
			});
		}}>Flamingo</button
	>

	<button
		class={`p-4 cursor-pointer tab-middle hover-bg-accent hover-text-n0 ${$state.accent === Accent.Pink ? "bg-accent text-n0" : ""}`}
		onclick={() => {
			vm.onAction({
				action: "accent-click",
				accent: Accent.Pink,
			});
		}}>Pink</button
	>

	<button
		class={`p-4 cursor-pointer tab-middle hover-bg-accent hover-text-n0 ${$state.accent === Accent.Mauve ? "bg-accent text-n0" : ""}`}
		onclick={() => {
			vm.onAction({
				action: "accent-click",
				accent: Accent.Mauve,
			});
		}}>Mauve</button
	>

	<button
		class={`p-4 cursor-pointer tab-middle hover-bg-accent hover-text-n0 ${$state.accent === Accent.Red ? "bg-accent text-n0" : ""}`}
		onclick={() => {
			vm.onAction({
				action: "accent-click",
				accent: Accent.Red,
			});
		}}>Red</button
	>

	<button
		class={`p-4 cursor-pointer tab-middle hover-bg-accent hover-text-n0 ${$state.accent === Accent.Maroon ? "bg-accent text-n0" : ""}`}
		onclick={() => {
			vm.onAction({
				action: "accent-click",
				accent: Accent.Maroon,
			});
		}}>Maroon</button
	>

	<button
		class={`p-4 cursor-pointer tab-middle hover-bg-accent hover-text-n0 ${$state.accent === Accent.Peach ? "bg-accent text-n0" : ""}`}
		onclick={() => {
			vm.onAction({
				action: "accent-click",
				accent: Accent.Peach,
			});
		}}>Peach</button
	>

	<button
		class={`p-4 cursor-pointer tab-middle hover-bg-accent hover-text-n0 ${$state.accent === Accent.Yellow ? "bg-accent text-n0" : ""}`}
		onclick={() => {
			vm.onAction({
				action: "accent-click",
				accent: Accent.Yellow,
			});
		}}>Yellow</button
	>

	<button
		class={`p-4 cursor-pointer tab-middle hover-bg-accent hover-text-n0 ${$state.accent === Accent.Green ? "bg-accent text-n0" : ""}`}
		onclick={() => {
			vm.onAction({
				action: "accent-click",
				accent: Accent.Green,
			});
		}}>Green</button
	>

	<button
		class={`p-4 cursor-pointer tab-middle hover-bg-accent hover-text-n0 ${$state.accent === Accent.Teal ? "bg-accent text-n0" : ""}`}
		onclick={() => {
			vm.onAction({
				action: "accent-click",
				accent: Accent.Teal,
			});
		}}>Teal</button
	>

	<button
		class={`p-4 cursor-pointer tab-middle hover-bg-accent hover-text-n0 ${$state.accent === Accent.Sky ? "bg-accent text-n0" : ""}`}
		onclick={() => {
			vm.onAction({
				action: "accent-click",
				accent: Accent.Sky,
			});
		}}>Sky</button
	>

	<button
		class={`p-4 cursor-pointer tab-middle hover-bg-accent hover-text-n0 ${$state.accent === Accent.Sapphire ? "bg-accent text-n0" : ""}`}
		onclick={() => {
			vm.onAction({
				action: "accent-click",
				accent: Accent.Sapphire,
			});
		}}>Sapphire</button
	>

	<button
		class={`p-4 cursor-pointer tab-middle hover-bg-accent hover-text-n0 ${$state.accent === Accent.Blue ? "bg-accent text-n0" : ""}`}
		onclick={() => {
			vm.onAction({
				action: "accent-click",
				accent: Accent.Blue,
			});
		}}>Blue</button
	>

	<button
		class={`p-4 cursor-pointer tab-end hover-bg-accent hover-text-n0 ${$state.accent === Accent.Lavender ? "bg-accent text-n0" : ""}`}
		onclick={() => {
			vm.onAction({
				action: "accent-click",
				accent: Accent.Lavender,
			});
		}}>Lavender</button
	>
</div>

<h2 class="text-lg font-medium mt-4">Titlebar</h2>

<div class="rounded-full space-x-1 bg-n1 flex w-fit mt-2">
	<button
		class={`p-4 cursor-pointer tab-begin hover-bg-accent hover-text-n0 ${$state.show_titlebar ? "bg-accent text-n0" : ""}`}
		onclick={() => {
			vm.onAction({
				action: "show-titlebar-click",
				value: true,
			});
		}}>On</button
	>

	<button
		class={`p-4 cursor-pointer tab-end hover-bg-accent hover-text-n0 ${!$state.show_titlebar ? "bg-accent text-n0" : ""}`}
		onclick={() => {
			vm.onAction({
				action: "show-titlebar-click",
				value: false,
			});
		}}>Off</button
	>
</div>

<style scoped>
	.tab-begin {
		border-radius: 4px;
		border-top-left-radius: 32px;
		border-bottom-left-radius: 32px;
	}

	.tab-middle {
		border-radius: 4px;
	}

	.tab-end {
		border-radius: 4px;
		border-top-right-radius: 32px;
		border-bottom-right-radius: 32px;
	}
</style>
