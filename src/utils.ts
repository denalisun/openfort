import { invoke } from "@tauri-apps/api/tauri";

export async function read_settings() {
    const data = await invoke("read_settings", {});
    return data;
};

export interface AppSettings {
    fortnite_path: string,
    username: string,
    extra_launch_args: string,
};