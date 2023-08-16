import { invoke } from "@tauri-apps/api/tauri"

export interface Settings {
    theme: string,
    accent: string
}


export async function getSettings(): Promise<Settings> {

    let settings: Settings = {
        theme: await invoke("get_setting", { setting: "theme", default: "mocha" }),
        accent: await invoke("get_setting", { setting: "accent", default: "blue" })
    }

    return settings
}

export async function updateSetting(setting: string, value: string){

    invoke("update_setting", {setting: setting, value: value})
}