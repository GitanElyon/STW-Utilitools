const { invoke } = window.__TAURI__.core;

import { initializeTitlebarButtons } from './js/TitlebarButtons.js';


document.addEventListener("DOMContentLoaded", () => {
  initializeTitlebarButtons();
  // initializeDropdowns(); // Doesnt wanna work???
});

/*
let greetInputEl;
let greetMsgEl;

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
}

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  document.querySelector("#greet-form").addEventListener("submit", (e) => {
    e.preventDefault();
    greet();
  });
});
*/

/*
window.addEventListener("DOMContentLoaded", () => {
  

  document.querySelectorAll('input[name="devs"]').forEach((radio) => {
    radio.addEventListener('change', (event) => {
      const selectedValue = event.target.value;
      handleSelection(selectedValue);
    });
  });
});

async function handleSelection(selectedValue) {
  try {
    const response = await invoke('handle_selection', { selected: selectedValue });
    console.log('Success:', response);
  } catch (error) {
    console.error('Error:', error);
  }
}
*/

async function handleSelection(selectedValue) {
  try {
    const response = await invoke('handle_selection', { selected: selectedValue });
    console.log('Success:', response);
  } catch (error) {
    console.error('Error:', error);
  }
}

window.addEventListener("DOMContentLoaded", () => {
  document.querySelectorAll('input[name="devs"]').forEach((radio) => {
    radio.addEventListener('change', (event) => {
      const selectedValue = event.target.value;
      handleSelection(selectedValue);
    });
  });

  document.querySelector("#print-settings").addEventListener("click", () => {
    invoke("print_settings")
      .then((response) => {
        console.log("Success:", response);
      })
      .catch((error) => {
        console.error("Error:", error);
      });
  });
});
