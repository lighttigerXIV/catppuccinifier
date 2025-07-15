<script lang="ts">
	type Props = {
		value: number;
		min: number;
		max: number;
		step: number;
		onchange: (value: number) => void;
	};

	let {
		value = $bindable(),
		min = $bindable(),
		max = $bindable(),
		step = $bindable(),
		onchange,
	}: Props = $props();

	let innerValue = $state(value);
</script>

<div class="slidecontainer">
	<input
		type="range"
		class="slider"
		{min}
		{max}
		{step}
		{value}
		oninput={(e) => {
			if (e.target instanceof HTMLInputElement)
				innerValue = +e.target.value;
		}}
		onchange={(e) => {
			if (e.target instanceof HTMLInputElement) {
				onchange(+e.target.value);
			}
		}}
	/>

	<div class="flex items-center">
		<div class="bg-n3 p-1 pl-3 pr-3 rounded-full">{min} ... {max}</div>
		<div class="flex-grow"></div>
		<div class="bg-n3 p-1 pl-3 pr-3 rounded-full min-w-[50px] text-center">
			{innerValue}
		</div>
	</div>
</div>

<style scoped>
	.slidecontainer {
		width: 100%;
	}

	.slider {
		-webkit-appearance: none;
		appearance: none;
		width: 100%;
		height: 24px;
		border-radius: 99px;
		background: var(--n3);
		outline: none;
	}

	.slider::-webkit-slider-thumb {
		-webkit-appearance: none;
		appearance: none;
		width: 12px;
		height: 32px;
		background: var(--accent);
		border-radius: 32px;
		cursor: pointer;
	}

	.slider:focus {
		border: 1px solid var(--accent);
	}

	.slider::-moz-range-thumb {
		width: 25px;
		height: 25px;
		border-radius: 50%;
		background: #04aa6d;
		cursor: pointer;
	}
</style>
