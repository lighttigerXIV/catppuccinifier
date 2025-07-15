<script lang="ts">
	import SettingsIcon from "$lib/icons/settings.svg?component";
	import SparklesIcon from "$lib/icons/sparkles.svg?component";
	import WandIcon from "$lib/icons/wand.svg?component";
	import { Tab } from "../main-repository";

	type Props = {
		tab: Tab;
		onclick: (tab: Tab) => void;
	};

	let { tab = $bindable(), onclick }: Props = $props();
</script>

<div
	class=" flex-nowrap space-y-4 p-5 bg-n1 flex flex-col items-center"
	id="navbar"
>
	<div class="w-full">
		<button
			class={`nav-button text-sm w-full flex flex-col items-center justify-center space-y-1 cursor-pointer ${tab === Tab.Generate ? "active" : ""}`}
			onclick={() => {
				onclick(Tab.Generate);
			}}
		>
			<div class="nav-icon w-full p-1 flex justify-center rounded-full">
				<SparklesIcon height="24" width="24" />
			</div>

			<p>Generate</p>
		</button>
	</div>

	<div class="w-full">
		<button
			class={`nav-button text-sm w-full flex flex-col items-center justify-center space-y-1 cursor-pointer ${tab === Tab.Properties ? "active" : ""}`}
			onclick={() => {
				onclick(Tab.Properties);
			}}
		>
			<div class="nav-icon w-full p-1 flex justify-center rounded-full">
				<WandIcon height="24" width="24" />
			</div>

			<p>Properties</p>
		</button>
	</div>

	<div class="w-full">
		<button
			class={`nav-button text-sm w-full flex flex-col items-center justify-center space-y-1 cursor-pointer ${tab === Tab.Settings ? "active" : ""}`}
			onclick={() => {
				onclick(Tab.Settings);
			}}
		>
			<div class="nav-icon w-full p-1 flex justify-center rounded-full">
				<SettingsIcon height="24" width="24" />
			</div>

			<p>Settings</p>
		</button>
	</div>
</div>

<style scoped>
	#navbar {
		border-right: 1px;
		border-style: solid;
		border-color: var(--n3);
	}

	.active .nav-icon,
	.nav-button:hover .nav-icon {
		background-color: var(--n3);
	}
</style>
