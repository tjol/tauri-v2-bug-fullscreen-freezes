function toggleFullscreen() {
  window.__TAURI__.event.emit("toggle-fullscreen");
}

window.addEventListener("DOMContentLoaded", () => {
  document.querySelector("#fullscreen-form").addEventListener("submit", (e) => {
    e.preventDefault();
    toggleFullscreen();
  });
});
