import { writable } from "svelte/store";

export class MainRepository {
	tab: Tab = Tab.Generate;
	properties: Properties = {
		files: [],
		flavors: [Flavor.Oled, Flavor.Mocha, Flavor.Macchiato, Flavor.Frappe, Flavor.Latte],
		hald_level: 8,
		luminosity: 1,
		algorithm: Algorithm.GaussianRBF,
		shape: 96,
		nearest: 0,
		mean: 0,
		std: 20,
		iterations: 512,
		power: 4
	};

	addFiles(files: string[]) {
		for (const file of files) {
			if (this.properties.files.includes(file)) {
				continue;
			}

			this.properties.files.push(file);
		};

		this.properties = this.properties;
	}

	removeFile(file: string) {
		let index = this.properties.files.findIndex(f => f === file);

		if (index === -1) throw "File not found";

		this.properties.files.splice(index, 1);
	}

	setProperties(properties: Properties) {
		this.properties = properties;
	}
}

export enum Tab {
	Generate,
	Properties,
	Settings
}

export type Properties = {
	files: string[];
	flavors: Flavor[];
	hald_level: number;
	luminosity: number;
	algorithm: Algorithm;
	shape: number;
	nearest: number;
	mean: number;
	std: number;
	iterations: number;
	power: number;
}

export enum Flavor {
	Oled = "Oled",
	Mocha = "Mocha",
	Macchiato = "Macchiato",
	Frappe = "Frappe",
	Latte = "Latte"
}

export enum Algorithm {
	ShepardsMethod = "ShepardsMethod",
	GaussianRBF = "GaussianRBF",
	LinearRBF = "LinearRBF",
	GaussianSampling = "GaussianSampling",
	NearestNeighbor = "NearestNeighbour",
}

export const mainRepository = writable(new MainRepository());
