import { invoke } from "@tauri-apps/api/tauri";

let launchButton: HTMLButtonElement;
let launchEditorButton: HTMLButtonElement;
let launchServerButton: HTMLButtonElement;
let fortnitePathInput: HTMLInputElement;

async function launch_game(is_server: boolean) {
    await invoke("launch_install", { isServer: is_server });
}

async function launch_editor(path: string) {
    await invoke("launch_editor", { path: path });
}

window.addEventListener("DOMContentLoaded", () => {
    launchButton = document.querySelector("#launch-button") as HTMLButtonElement;
    launchServerButton = document.querySelector("#launch-server-button") as HTMLButtonElement;
    launchEditorButton = document.querySelector("#launch-editor-button") as HTMLButtonElement;
    fortnitePathInput = document.querySelector("#fortnite-path-input") as HTMLInputElement;

    launchButton.addEventListener("click", () => {
        launch_game(false);
    });

    launchEditorButton.addEventListener("click", () => {
        // launch_editor();
    });

    launchServerButton.addEventListener("click", () => {
        launch_game(true);
    });
});