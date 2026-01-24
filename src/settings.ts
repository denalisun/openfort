import { invoke } from "@tauri-apps/api/tauri";
import * as utils from "./utils"

let fortnitePathInput: HTMLInputElement;
let usernameInput: HTMLInputElement;
let applyButton: HTMLButtonElement;

async function change_settings(username: string, fortnitePath: string) {
    await invoke("change_settings", { username, fortnitePath });
}

document.addEventListener("DOMContentLoaded", () => {
    fortnitePathInput = document.querySelector("#fortnite-path-input") as HTMLInputElement;
    usernameInput = document.querySelector("#username-input") as HTMLInputElement;
    applyButton = document.querySelector("#apply-settings-button") as HTMLButtonElement;

    utils.read_settings().then((data) => { 
        let dt = data as utils.AppSettings;
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