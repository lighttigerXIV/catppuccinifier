import { Accent, Theme, type Settings } from "$lib/models/Settings";
import { flavorEntries, type AccentName } from "@catppuccin/palette";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { writable } from "svelte/store";

export class SettingsRepository {
	private settings: Settings | null = null;

	async init(onSettingsLoad: (() => void)) {
		try {
			this.settings = await invoke("vm_get_settings");
			onSettingsLoad();
		} catch (e) {
			console.error("Error getting settings: " + e);
		}
	}

	getSettings(): Settings {
		if (this.settings)
			return this.settings;
		else
			throw "Settings haven't loaded yet";
	}

	getStyle(): string {
		const settings = this.getSettings();

		let flavor = flavorEntries.find(e => e[0] === settings.theme.toLowerCase())!![1];
		let colors = flavor.colorEntries;

		let accentColor = colors.find(e => e[0] === settings.accent.toLowerCase())!![1].hex;

		let textColor = colors.find(e => e[0] === "text")!![1].hex;
		let textOneColor = colors.find(e => e[0] === "subtext1")!![1].hex;
		let textTwoColor = colors.find(e => e[0] === "subtext0")!![1].hex;

		let neutralColor = colors.find(e => e[0] === "crust")!![1].hex;
		let neutralOneColor = colors.find(e => e[0] === "mantle")!![1].hex;
		let neutralTwoColor = colors.find(e => e[0] === "base")!![1].hex;
		let neutralThreeColor = colors.find(e => e[0] === "surface0")!![1].hex;
		let neutralFourColor = colors.find(e => e[0] === "surface1")!![1].hex;

		return `
<style>
:root{
	--t0: ${textColor};
	--t1: ${textOneColor};
	--t2: ${textTwoColor};
	--n0: ${neutralColor};
	--n1: ${neutralOneColor};
	--n2: ${neutralTwoColor};
	--n3: ${neutralThreeColor};
	--n4: ${neutralFourColor};
	--accent: ${accentColor};
}


/** ============================================== */
/** Background Colors */
/** ============================================== */

.bg-n0{
	background-color: var(--n0);
}

.bg-n1, .disabled-bg-n1:disabled{
	background-color: var(--n1);
}

.bg-n2{
	background-color: var(--n2);
}

.bg-n3, .hover-bg-n3:hover:enabled{
	background-color: var(--n3);
}

.bg-n4{
	background-color: var(--n4);
}

.bg-accent, .hover-bg-accent:hover{
	background-color: var(--accent);
}

/** ============================================== */
/** Border Colors */
/** ============================================== */

.border-n3{
	border-color: var(--n3);
}

.disabled-border-n1:disabled{
	border-color: var(--n1);
}

.border-accent{
	border-color: var(--accent);
}

/** ============================================== */
/** Text Colors */
/** ============================================== */

.t0{
	color: var(--t0);
}

.t1{
	color: var(--t1);
}

.t2, .disabled-t2:disabled{
	color: var(--t2);
}

.text-n0, .hover-text-n0:hover{
	color: var(--n0);
}

</style>
		`
	}

	setTheme(theme: Theme) {
		this.settings!!.theme = theme;
		this.save();
	}

	setAccent(accent: Accent) {
		this.settings!!.accent = accent;
		this.save();
	}

	private async save() {
		let settings = this.getSettings();
		await invoke("vm_save_settings", { settings: settings });

		getCurrentWindow().emit("settings-saved");
	}
}

export const settingsRepository = writable(new SettingsRepository());



