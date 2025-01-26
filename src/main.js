const { invoke } = window.__TAURI__.core;

import { initializeTitlebarButtons } from './js/TitlebarButtons.js';

// frontend functions
document.addEventListener("DOMContentLoaded", () => {
  initializeTitlebarButtons();
  // initializeDropdowns(); // Doesnt wanna work???
});

// Call the function when the app starts to set the correct radio button
document.addEventListener('DOMContentLoaded', get_devs_value);

document.getElementById('settings-icon').addEventListener('click', function () {
  document.getElementById('main').style.display = 'none';
  document.getElementById('settings').style.display = 'block';
});

document.getElementById('back-button').addEventListener('click', function () {
  document.getElementById('main').style.display = 'block';
  document.getElementById('settings').style.display = 'none';
});







// backend functions

// dev selector
async function handleSelection(selectedValue) {
  try {
    const response = await invoke('handle_selection', { selected: selectedValue });
    console.log('Success:', response);
  } catch (error) {
    console.error('Error:', error);
  }
}

// open files to select fn path
async function open_files() {
  try {
    const response = await invoke('open_files');
    console.log("Success:", response);
  } catch (error) {
    console.error("Error:", error);
  }
}

// print settings (debug)
async function print_settings() {
  try {
    const response = await invoke('print_settings');
    console.log("Success:", response);
  } catch (error) {
    console.error("Error:", error);
  }
}

// Called when app starts to get the devs value
async function get_devs_value() {
  try {
    const response = await invoke('get_devs_value');
    console.log("Success:", response);

    // Set the correct radio button
    const radioButton = document.querySelector(`input[name="devs"][value="${response}"]`);
    if (radioButton) {
      radioButton.checked = true;
    }
  } catch (error) {
    console.error("Error:", error);
  }
}

window.addEventListener("DOMContentLoaded", () => {
  document.querySelectorAll('input[name="devs"]').forEach((radio) => {
    radio.addEventListener('change', (event) => {
      const selectedValue = event.target.value;
      handleSelection(selectedValue);
    });
  });


  // open files to select fn path
  document.getElementById('open-files').addEventListener('click', function () {
    open_files();
  });

  document.querySelector("#print-settings").addEventListener("click", () => {
    print_settings();
  });

  document.querySelector("#get-devs-value").addEventListener("click", () => {
    get_devs_value();
  });
});


