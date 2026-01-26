import { invoke } from "@tauri-apps/api/tauri";

let launchButton: HTMLButtonElement;
let launchEditorButton: HTMLButtonElement;
let launchServerButton: HTMLButtonElement;

async function launch_game(is_server: boolean) {
    await invoke("launch_install", { isServer: is_server });
}

async function launch_editor() {
    await invoke("launch_editor");
}

async function does_build_have_uefn(): Promise<boolean> {
    await invoke("does_build_have_uefn").then((data) => {
        return data;
    });
    return false;
}

window.addEventListener("DOMContentLoaded", () => {
    launchButton = document.querySelector("#launch-button") as HTMLButtonElement;
    launchServerButton = document.querySelector("#launch-server-button") as HTMLButtonElement;
    launchEditorButton = document.querySelector("#launch-editor-button") as HTMLButtonElement;

    // Getting settings
    does_build_have_uefn().then((data) => {
        launchEditorButton.style.visibility = `${data}`;
    });

    launchButton.addEventListener("click", () => {
        launch_game(false);
    });

    launchEditorButton.addEventListener("click", () => {
        launch_editor();
    });

    launchServerButton.addEventListener("click", () => {
        launch_game(true);
    });
});