import { get, writable } from "svelte/store";
import { settingsRepository } from "$lib/repositories/settings-repository";
import type { Accent, Theme } from "$lib/models/Settings";

export type SettingsPageAction = { action: "theme-click", theme: Theme } | { action: "accent-click", accent: Accent } | { action: "show-titlebar-click", value: boolean };

export class SettingsPageVM {
	state = writable<{
		theme: Theme,
		accent: Accent,
		show_titlebar: boolean
	}>({
		theme: get(settingsRepository).getSettings().theme,
		accent: get(settingsRepository).getSettings().accent,
		show_titlebar: get(settingsRepository).getSettings().show_titlebar
	});

	onAction(action: SettingsPageAction) {
		switch (action.action) {
			case "theme-click":
				this.onThemeClick(action.theme);
				break;
			case "accent-click":
				this.onAccentClick(action.accent);
				break;
			case "show-titlebar-click":
				this.onShowTitlebarClick(action.value);
				break;
		}
	}

	private onThemeClick(theme: Theme) {
		get(settingsRepository).setTheme(theme);
		this.state.update(state => ({ ...state, theme: theme }));
	}

	private onAccentClick(accent: Accent) {
		get(settingsRepository).setAccent(accent);
		this.state.update(state => ({ ...state, accent: accent }));
	}

	private onShowTitlebarClick(value: boolean) {
		get(settingsRepository).setShowTitlebar(value);
		this.state.update(state => ({ ...state, show_titlebar: value }));
	}
}
