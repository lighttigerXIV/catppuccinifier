<script lang="ts">
	import { convertFileSrc } from "@tauri-apps/api/core";
	import { GeneratePageVM } from "./generate-page-vm";
	import TrashIcon from "$lib/icons/trash.svg?component";
	import SaveIcon from "$lib/icons/save.svg?component";
	import PreviewIcon from "$lib/icons/preview.svg?component";
	import { mainRepository } from "../../main-repository";
	import { onMount } from "svelte";

	let vm = new GeneratePageVM();
	let state = vm.state;
</script>

<div class="flex-grow">
	<div class="flex spacing-x-4 items-top">
		<div class="flex-grow">
			<h1 class="text-xl font-medium">Images</h1>

			<p class="t1">Select image/s to catppuccinify</p>
		</div>

		<button
			class="h-fit p-4 border border-n3 rounded-full hover-bg-n3 cursor-pointer"
			onclick={() => {
				vm.onAction({ action: "select-images-click" });
			}}
		>
			Select Images
		</button>
	</div>

	<div
		class={`p-4 bg-n1 mt-4 border rounded-2xl h-48 flex space-x-2 overflow-auto ${$state.hoveringImage ? "border-accent" : "border-n3"}`}
	>
		{#if $state.files.length === 0}
			<div class="flex-grow flex items-center justify-center h-full">
				No Selected Images
			</div>
		{/if}

		{#each $state.files as file}
			<div
				class="h-full w-[240px] flex-nowrap rounded-lg flex-none flex flex-col justify-items-end items-end"
				style="
					background-image: url('{convertFileSrc(file)}');
					background-size: cover;
					background-repeat: no-repeat;
				"
			>
				<button
					class=" p-4 bg-n0 hover-bg-n3 m-2 h-fit cursor-pointer rounded-full"
					onclick={() => {
						vm.onAction({
							action: "remove-image-click",
							file: file,
						});
					}}
					disabled={$state.generating}
				>
					<TrashIcon height="24" width="24" />
				</button>
			</div>
		{/each}
	</div>

	<div class="flex space-x-4">
		<h1 class="text-xl font-medium mt-4 flex-grow">Generated Images</h1>

		{#if $state.generating}
			<button
				class="h-fit p-4 border border-n3 rounded-full hover-bg-n3 enabled:cursor-pointer mt-2 disabled-bg-n1 disabled-border-n1 disabled-t2"
				onclick={() => {
					vm.onAction({ action: "stop-click" });
				}}
			>
				Stop
			</button>
		{/if}

		<button
			class="h-fit p-4 border border-n3 rounded-full hover-bg-n3 enabled:cursor-pointer mt-2 disabled-bg-n1 disabled-border-n1 disabled-t2"
			onclick={() => {
				vm.onAction({ action: "generate-click" });
			}}
			disabled={$state.files.length === 0 ||
				$state.generating ||
				$mainRepository.properties.flavors.length === 0}
		>
			Generate
		</button>
	</div>

	{#each $state.generated as generatedImage}
		<div class="p-4 bg-n1 mt-4 border border-n3 rounded-2xl">
			<h2 class="font-lg font-medium">Original</h2>

			<div
				class="h-[200px] w-[300px] flex-nowrap rounded-lg flex-none flex flex-col justify-items-end items-end"
				style="
					background-image: url('{convertFileSrc(generatedImage.original)}');
					background-size: cover;
					background-repeat: no-repeat;
				"
			></div>

			<h2 class="font-lg font-medium mt-4">Generated</h2>

			<div class="flex overflow-auto space-x-2">
				{#each generatedImage.outputs as output}
					<div
						class="h-[200px] w-[300px] flex-nowrap rounded-lg flex-none flex flex-col justify-items-end items-end"
						style="
					background-image: url('{convertFileSrc(output)}');
					background-size: cover;
					background-repeat: no-repeat;
				"
					>
						<button
							class=" p-4 bg-n0 hover-bg-n3 m-2 h-fit cursor-pointer rounded-full"
							onclick={() => {
								vm.onAction({
									action: "preview-click",
									file: output,
								});
							}}
						>
							<PreviewIcon height="24" width="24" />
						</button>

						<button
							class=" p-4 bg-n0 hover-bg-n3 m-2 h-fit cursor-pointer rounded-full"
							onclick={() => {
								vm.onAction({
									action: "save-click",
									output: output,
								});
							}}
						>
							<SaveIcon height="24" width="24" />
						</button>
					</div>
				{/each}
			</div>
		</div>
	{/each}
</div>
