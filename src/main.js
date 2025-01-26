const { invoke } = window.__TAURI__.core;

import { initializeTitlebarButtons } from './js/TitlebarButtons.js';

// frontend functions
document.addEventListener("DOMContentLoaded", () => {
  initializeTitlebarButtons();
  // initializeDropdowns(); // Doesnt wanna work???
});

document.getElementById('settings-icon').addEventListener('click', function() {
  document.getElementById('main').style.display = 'none';
  document.getElementById('settings').style.display = 'block';
});

document.getElementById('back-button').addEventListener('click', function() {
  document.getElementById('main').style.display = 'block';
  document.getElementById('settings').style.display = 'none';
});





// backend functions
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
    print_settings();
  });
});

async function print_settings() {
  try {
    const response = await invoke('print_settings');
    console.log("Success:", response);
  } catch (error) {
    console.error("Error:", error);
  }
}
