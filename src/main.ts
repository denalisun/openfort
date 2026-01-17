import { invoke } from "@tauri-apps/api/tauri";

let launchButton: HTMLButtonElement;
let launchEditorButton: HTMLButtonElement;
let fortnitePathInput: HTMLInputElement;

let usernameBox: HTMLInputElement;

async function validate_install(): Promise<boolean> {
  if (launchButton && fortnitePathInput) {
    console.log(`Path: ${fortnitePathInput.value}`);
    let val = await invoke("validate_install", { path: fortnitePathInput.value });
    return val as boolean;
  }
  return false;
}

async function launch_game(path: string, username: string) {
  await invoke("launch_install", { path: path, username: username });
}

async function launch_editor(path: string) {
  await invoke("launch_editor", { path: path });
}

window.addEventListener("DOMContentLoaded", () => {
  launchButton = document.querySelector("#launch-button") as HTMLButtonElement;
  launchEditorButton = document.querySelector("#launch-editor-button") as HTMLButtonElement;
  fortnitePathInput = document.querySelector("#fortnite-path-input") as HTMLInputElement;

  usernameBox = document.querySelector("#username-input") as HTMLInputElement;

  launchButton.addEventListener("click", () => {
    validate_install().then((validated) => {
      if (validated) {
        let username: string = usernameBox.value != "" ? "UnknownLooper" : usernameBox.value;
        launch_game(fortnitePathInput.value, username);
      } else {
        console.log("Could not find Fortnite path!");
      }
    });
  });

  launchEditorButton.addEventListener("click", () => {
    validate_install().then((validated) => {
      if (validated) {
        launch_editor(fortnitePathInput.value);
      } else {
        console.log("Could not validate UEFN!");
      }
    });
  });
});