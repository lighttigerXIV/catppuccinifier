import { open, save } from "@tauri-apps/plugin-dialog";
import { get, writable } from "svelte/store";
import { mainRepository } from "../../main-repository";
import { invoke } from "@tauri-apps/api/core";
import { downloadDir } from "@tauri-apps/api/path";
import { WebviewWindow } from "@tauri-apps/api/webviewWindow";
import { listen } from "@tauri-apps/api/event";

export type GeneratePageAction = { action: "select-images-click" }
	| { action: "remove-image-click", file: string }
	| { action: "generate-click" }
	| { action: "stop-click" }
	| { action: "save-click", output: string }
	| { action: "preview-click", file: string };

export type GeneratedImage = {
	original: string,
	outputs: string[]
}

export class GeneratePageVM {

	state = writable<{
		files: string[],
		generated: GeneratedImage[],
		generating: boolean,
		hoveringImage: boolean
	}>({
		files: get(mainRepository).properties.files,
		generated: [],
		generating: false,
		hoveringImage: false
	})

	private stopGenerating = false;

	constructor() {

		listen("tauri://drag-enter", (_e) => {
			this.state.update(state => ({ ...state, hoveringImage: true }));
		})

		listen<{ paths: string[], position: {} }>("tauri://drag-drop", (e) => {
			if (e.payload.paths !== null) {
				for (const file of e.payload.paths) {
					let extension = file.split('.').pop() ?? "";

					if (!['png', 'jpeg', 'jpg', 'webp'].includes(extension)) {
						continue;
					}

					get(mainRepository).addFiles([file]);
				}
			}

			this.state.update(state => ({ ...state, hoveringImage: false }));
		})

		listen("tauri://drag-leave", (_e) => {
			this.state.update(state => ({ ...state, hoveringImage: false }));
		})
	}

	onAction(action: GeneratePageAction) {
		switch (action.action) {
			case "select-images-click":
				this.onSelectImagesClick();
				break;

			case "remove-image-click":
				this.onRemoveImageClick(action.file);
				break;

			case "generate-click":
				this.onGenerateClick();
				break;

			case "stop-click":
				this.onStopClick();
				break;

			case "save-click":
				this.onSaveClick(action.output);
				break;

			case "preview-click":
				this.onPreviewClick(action.file);
				break;
		}
	}

	private async onSelectImagesClick() {
		const selectedFiles = await open({
			multiple: true,
			filters: [{
				name: 'Image',
				extensions: ['png', 'jpeg', 'jpg', 'webp']
			}]
		});

		if (selectedFiles !== null) {
			get(mainRepository).addFiles(selectedFiles);

			this.state.update(state => ({ ...state, files: get(mainRepository).properties.files }));
		}
	}

	private onRemoveImageClick(file: string) {
		get(mainRepository).removeFile(file);

		this.state.update(state => ({ ...state, files: get(mainRepository).properties.files }));
	}

	private async onGenerateClick() {

		this.state.update(state => ({ ...state, generated: [], generating: true }));

		await invoke("vm_clear_outputs");

		let properties = get(mainRepository).properties;
		let generatedImages: GeneratedImage[] = [];

		for (const file of properties.files) {

			if (this.stopGenerating) {
				this.stopGenerating = false;
				return;
			}

			let generatedImage: GeneratedImage = {
				original: file,
				outputs: []
			};

			for (const flavor of properties.flavors) {

				if (this.stopGenerating) {
					this.stopGenerating = false;
					return;
				}


				let output: string = await invoke("vm_generate_image", {
					properties: {
						file: file,
						flavor: flavor,
						hald_level: properties.hald_level,
						luminosity: properties.luminosity,
						algorithm: properties.algorithm,
						shape: properties.shape,
						nearest: properties.nearest,
						mean: properties.mean,
						std: properties.std,
						iterations: properties.iterations,
						power: properties.power
					}
				});

				generatedImage.outputs.push(output);

				let generatedImageIndex = generatedImages.findIndex(gi => gi.original === file);

				if (generatedImageIndex === -1) {
					generatedImages.push(generatedImage);
				} else {
					generatedImages[generatedImageIndex] = generatedImage;
				}

				this.state.update(state => ({ ...state, generated: generatedImages }));
			}
		}

		this.state.update(state => ({ ...state, generating: false }));
	}

	private async onStopClick() {
		this.stopGenerating = true;

		this.state.update(state => ({ ...state, generating: false }));
	}

	async onSaveClick(output: string) {

		let fileExtension = output.split(".").pop();

		const filePath = await save({
			filters: [{
				name: 'Image',
				extensions: ['png', 'jpeg', 'jpg', 'webp']
			}],
			title: 'Save',
			defaultPath: await downloadDir() + "/image." + fileExtension
		});

		if (filePath !== null) {
			await invoke("vm_save_image", { save_path: filePath, output: output });
		}
	}

	onPreviewClick(file: string) {
		new WebviewWindow("preview", {
			url: "/preview/" + encodeURIComponent(file),
			decorations: false,
			alwaysOnTop: true,
			fullscreen: true
		});
	}
}
