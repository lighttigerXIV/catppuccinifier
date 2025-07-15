import { get, writable } from "svelte/store";
import { Algorithm, mainRepository, type Flavor, type Properties } from "../../main-repository";

export type PropertiesPageAction = {
	action: "flavor-toggle",
	flavor: Flavor
} | {
	action: "algorithm-click",
	algorithm: Algorithm
} | {
	action: "hald-level-set",
	value: number
} | {
	action: "luminosity-set",
	value: number
} | {
	action: "shape-set",
	value: number
} | {
	action: "nearest-set",
	value: number
} | {
	action: "mean-set",
	value: number
} | {
	action: "std-set",
	value: number
} | {
	action: "iterations-set",
	value: number
} | {
	action: "power-set",
	value: number
}

export class PropertiesPageVM {
	state = writable<{
		properties: Properties
	}>({
		properties: get(mainRepository).properties
	});

	onAction(action: PropertiesPageAction) {
		switch (action.action) {
			case "flavor-toggle":
				this.onFlavorToggle(action.flavor)
				break;
			case "algorithm-click":
				this.onAlgorithmClick(action.algorithm)
				break;
			case "hald-level-set":
				this.onHaldLevelSet(action.value)
				break;
			case "luminosity-set":
				this.onLuminositySet(action.value)
				break;
			case "shape-set":
				this.onShapeSet(action.value)
				break;
			case "nearest-set":
				this.onNearestSet(action.value)
				break;
			case "mean-set":
				this.onMeanSet(action.value)
				break;
			case "std-set":
				this.onStdSet(action.value)
				break;
			case "iterations-set":
				this.onIterationsSet(action.value)
				break;
			case "power-set":
				this.onPowerSet(action.value)
				break;
		}
	}

	private onFlavorToggle(flavor: Flavor) {
		let properties = get(this.state).properties;
		const index = properties.flavors.findIndex(f => f === flavor);

		if (index === -1) {
			properties.flavors.push(flavor);
			this.state.update(state => ({ ...state, properties: properties }));

			return;
		}

		properties.flavors = properties.flavors.filter(f => f !== flavor);

		this.state.update(state => ({ ...state, properties: properties }));

		get(mainRepository).setProperties(properties);
	}

	private onAlgorithmClick(algorithm: Algorithm) {
		let properties = get(this.state).properties;
		properties.algorithm = algorithm;
		properties.hald_level = 8;
		properties.luminosity = 1;
		properties.shape = 96;
		properties.nearest = algorithm === Algorithm.LinearRBF ? 5 : 26;
		properties.mean = 0;
		properties.std = 20;
		properties.iterations = 512
		properties.power = 4;

		this.state.update(state => ({ ...state, properties: properties }));
		get(mainRepository).setProperties(properties);
	}

	private onHaldLevelSet(value: number) {
		let properties = get(this.state).properties;
		properties.hald_level = value;

		this.state.update(state => ({ ...state, properties: properties }));
		get(mainRepository).setProperties(properties);
	}

	private onLuminositySet(value: number) {
		let properties = get(this.state).properties;
		properties.luminosity = value;

		this.state.update(state => ({ ...state, properties: properties }));
		get(mainRepository).setProperties(properties);
	}

	private onShapeSet(value: number) {
		let properties = get(this.state).properties;
		properties.shape = value;

		this.state.update(state => ({ ...state, properties: properties }));
		get(mainRepository).setProperties(properties);
	}

	private onNearestSet(value: number) {
		let properties = get(this.state).properties;
		properties.nearest = value;

		this.state.update(state => ({ ...state, properties: properties }));
		get(mainRepository).setProperties(properties);
	}

	private onMeanSet(value: number) {
		let properties = get(this.state).properties;
		properties.mean = value;

		this.state.update(state => ({ ...state, properties: properties }));
		get(mainRepository).setProperties(properties);
	}

	private onStdSet(value: number) {
		let properties = get(this.state).properties;
		properties.std = value;

		this.state.update(state => ({ ...state, properties: properties }));
		get(mainRepository).setProperties(properties);
	}

	private onIterationsSet(value: number) {
		let properties = get(this.state).properties;
		properties.iterations = value;

		this.state.update(state => ({ ...state, properties: properties }));
		get(mainRepository).setProperties(properties);
	}

	private onPowerSet(value: number) {
		let properties = get(this.state).properties;
		properties.power = value;

		this.state.update(state => ({ ...state, properties: properties }));
		get(mainRepository).setProperties(properties);
	}
}
