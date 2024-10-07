# Toggle fullscreen in Tauri v2 (bug)

This is a minimal working example where `window.set_fullscreen(true)` fails to
enter fullscreen and freezes the app, at least on Windows 11.

Steps to test:

 1. Run the app (`cargo tauri dev`)
 2. Click the "Toggle fullscreen button"

Expected behaviour: the app goes full screen

Actual behaviour: the window decoration vanishes, and the app stops responding.
