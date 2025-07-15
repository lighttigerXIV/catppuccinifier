<script lang="ts">
	import RangeSlider from "$lib/components/RangeSlider.svelte";
	import { Algorithm, Flavor } from "../../main-repository";
	import { PropertiesPageVM } from "./properties-page-vm";

	const vm = new PropertiesPageVM();
	const state = vm.state;
</script>

<div>
	<h2 class="text-lg font-medium">Flavors</h2>

	<div class={`flex space-x-2 mt-2`}>
		<button
			class={`p-3 pl-6 pr-6 rounded-full cursor-pointer ${$state.properties.flavors.includes(Flavor.Oled) ? "bg-accent text-n0" : "bg-n1 t0"}`}
			onclick={() => {
				vm.onAction({ action: "flavor-toggle", flavor: Flavor.Oled });
			}}
		>
			Oled
		</button>

		<button
			class={`p-3 pl-6 pr-6 rounded-full cursor-pointer ${$state.properties.flavors.includes(Flavor.Mocha) ? "bg-accent text-n0" : "bg-n1 t0"}`}
			onclick={() => {
				vm.onAction({ action: "flavor-toggle", flavor: Flavor.Mocha });
			}}
		>
			Mocha
		</button>

		<button
			class={`p-3 pl-6 pr-6 rounded-full cursor-pointer ${$state.properties.flavors.includes(Flavor.Macchiato) ? "bg-accent text-n0" : "bg-n1 t0"}`}
			onclick={() => {
				vm.onAction({
					action: "flavor-toggle",
					flavor: Flavor.Macchiato,
				});
			}}
		>
			Macchiato
		</button>

		<button
			class={`p-3 pl-6 pr-6 rounded-full cursor-pointer ${$state.properties.flavors.includes(Flavor.Frappe) ? "bg-accent text-n0" : "bg-n1 t0"}`}
			onclick={() => {
				vm.onAction({ action: "flavor-toggle", flavor: Flavor.Frappe });
			}}
		>
			Frappe
		</button>

		<button
			class={`p-3 pl-6 pr-6 rounded-full cursor-pointer ${$state.properties.flavors.includes(Flavor.Latte) ? "bg-accent text-n0" : "bg-n1 t0"}`}
			onclick={() => {
				vm.onAction({ action: "flavor-toggle", flavor: Flavor.Latte });
			}}
		>
			Latte
		</button>
	</div>

	<h2 class="text-lg font-medium mt-4">Algorithm</h2>

	<div class="rounded-full space-x-1 bg-n1 flex w-fit mt-2">
		<button
			class={`p-4 cursor-pointer tab-begin hover-bg-accent hover-text-n0 ${$state.properties.algorithm === Algorithm.GaussianRBF ? "bg-accent text-n0" : ""}`}
			onclick={() => {
				vm.onAction({
					action: "algorithm-click",
					algorithm: Algorithm.GaussianRBF,
				});
			}}>Gaussian RBF</button
		>

		<button
			class={`p-4 cursor-pointer tab-middle hover-bg-accent hover-text-n0 ${$state.properties.algorithm === Algorithm.GaussianSampling ? "bg-accent text-n0" : ""}`}
			onclick={() => {
				vm.onAction({
					action: "algorithm-click",
					algorithm: Algorithm.GaussianSampling,
				});
			}}>Gaussian Sampling</button
		>

		<button
			class={`p-4 cursor-pointer tab-middle hover-bg-accent hover-text-n0 ${$state.properties.algorithm === Algorithm.LinearRBF ? "bg-accent text-n0" : ""}`}
			onclick={() => {
				vm.onAction({
					action: "algorithm-click",
					algorithm: Algorithm.LinearRBF,
				});
			}}>Linear RBF</button
		>

		<button
			class={`p-4 cursor-pointer tab-middle hover-bg-accent hover-text-n0 ${$state.properties.algorithm === Algorithm.ShepardsMethod ? "bg-accent text-n0" : ""}`}
			onclick={() => {
				vm.onAction({
					action: "algorithm-click",
					algorithm: Algorithm.ShepardsMethod,
				});
			}}>Shepard's Metod</button
		>

		<button
			class={`p-4 cursor-pointer tab-end hover-bg-accent hover-text-n0 ${$state.properties.algorithm === Algorithm.NearestNeighbor ? "bg-accent text-n0" : ""}`}
			onclick={() => {
				vm.onAction({
					action: "algorithm-click",
					algorithm: Algorithm.NearestNeighbor,
				});
			}}>Nearest Neighbour</button
		>
	</div>

	<h2 class="text-lg font-medium mt-4">Properties</h2>

	<div class="space-y-4">
		<div class="border border-n3 p-4 rounded-2xl bg-n1">
			<h2 class="font-medium">Hald Level</h2>
			<RangeSlider
				value={$state.properties.hald_level}
				min={2}
				max={16}
				step={1}
				onchange={(value) => {
					vm.onAction({ action: "hald-level-set", value: value });
				}}
			/>
		</div>

		<div class="border border-n3 p-4 rounded-2xl bg-n1">
			<h2 class="font-medium">Luminosity</h2>
			<RangeSlider
				value={$state.properties.luminosity}
				min={0.1}
				max={1}
				step={0.1}
				onchange={(value) => {
					vm.onAction({ action: "luminosity-set", value: value });
				}}
			/>
		</div>

		{#if $state.properties.algorithm === Algorithm.GaussianRBF}
			<div class="border border-n3 p-4 rounded-2xl bg-n1">
				<h2 class="font-medium">Shape</h2>
				<RangeSlider
					value={$state.properties.shape}
					min={0}
					max={512}
					step={1}
					onchange={(value) => {
						vm.onAction({ action: "shape-set", value: value });
					}}
				/>
			</div>
		{/if}

		{#if [Algorithm.GaussianRBF, Algorithm.LinearRBF, Algorithm.ShepardsMethod].includes($state.properties.algorithm)}
			<div class="border border-n3 p-4 rounded-2xl bg-n1">
				<h2 class="font-medium">Nearest</h2>
				<RangeSlider
					value={$state.properties.nearest}
					min={2}
					max={26}
					step={1}
					onchange={(value) => {
						vm.onAction({ action: "nearest-set", value: value });
					}}
				/>
			</div>
		{/if}

		{#if $state.properties.algorithm === Algorithm.GaussianSampling}
			<div class="border border-n3 p-4 rounded-2xl bg-n1">
				<h2 class="font-medium">Mean</h2>
				<RangeSlider
					value={$state.properties.mean}
					min={0}
					max={255}
					step={1}
					onchange={(value) => {
						vm.onAction({ action: "mean-set", value: value });
					}}
				/>
			</div>
		{/if}

		{#if $state.properties.algorithm === Algorithm.GaussianSampling}
			<div class="border border-n3 p-4 rounded-2xl bg-n1">
				<h2 class="font-medium">STD</h2>
				<RangeSlider
					value={$state.properties.std}
					min={0}
					max={255}
					step={1}
					onchange={(value) => {
						vm.onAction({ action: "std-set", value: value });
					}}
				/>
			</div>
		{/if}

		{#if $state.properties.algorithm === Algorithm.GaussianSampling}
			<div class="border border-n3 p-4 rounded-2xl bg-n1">
				<h2 class="font-medium">Iterations</h2>
				<RangeSlider
					value={$state.properties.iterations}
					min={0}
					max={1024}
					step={1}
					onchange={(value) => {
						vm.onAction({ action: "iterations-set", value: value });
					}}
				/>
			</div>
		{/if}

		{#if $state.properties.algorithm === Algorithm.ShepardsMethod}
			<div class="border border-n3 p-4 rounded-2xl bg-n1">
				<h2 class="font-medium">Power</h2>
				<RangeSlider
					value={$state.properties.power}
					min={0}
					max={64}
					step={1}
					onchange={(value) => {
						vm.onAction({ action: "power-set", value: value });
					}}
				/>
			</div>
		{/if}
	</div>
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
