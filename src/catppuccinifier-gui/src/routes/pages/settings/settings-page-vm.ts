import { get, writable } from "svelte/store";
import { settingsRepository } from "$lib/repositories/settings-repository";
import type { Accent, Theme } from "$lib/models/Settings";

export type SettingsPageAction = { action: "theme-click", theme: Theme } | { action: "accent-click", accent: Accent };

export class SettingsPageVM {
	state = writable<{
		theme: Theme,
		accent: Accent
	}>({
		theme: get(settingsRepository).getSettings().theme,
		accent: get(settingsRepository).getSettings().accent
	});

	onAction(action: SettingsPageAction) {
		switch (action.action) {
			case "theme-click":
				this.onThemeClick(action.theme);
				break;
			case "accent-click":
				this.onAccentClick(action.accent);
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
}
