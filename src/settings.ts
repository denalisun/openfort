import { invoke } from "@tauri-apps/api/tauri";

let fortnitePathInput: HTMLInputElement;
let usernameInput: HTMLInputElement;
let applyButton: HTMLButtonElement;

async function change_settings(username: string, fortnitePath: string) {
    await invoke("change_settings", { username, fortnitePath });
}

interface AppSettings {
    fortnite_path: string,
    username: string,
    extra_launch_args: string,
};

async function read_settings() {
    const data = await invoke("read_settings", {});
    return data;
}

document.addEventListener("DOMContentLoaded", () => {
    fortnitePathInput = document.querySelector("#fortnite-path-input") as HTMLInputElement;
    usernameInput = document.querySelector("#username-input") as HTMLInputElement;
    applyButton = document.querySelector("#apply-settings-button") as HTMLButtonElement;

    read_settings().then((data) => { 
        let dt = data as AppSettings;
        fortnitePathInput.value = dt.fortnite_path;
        usernameInput.value = dt.username;
    });

    applyButton.addEventListener('click', () => {
        let username = usernameInput.value;
        let fortnitePath = fortnitePathInput.value;

        console.log(`Username: ${username}`);
        console.log(`Fortnite Path: ${fortnitePath}`);

        change_settings(username, fortnitePath);
    });
});