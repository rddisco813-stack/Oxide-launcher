# Next patches (need upstream source open)
1. TEST MODE (no Microsoft): hide the Microsoft login button; default to the offline/local account.
2. RUST EDITION: add a "Rust Edition" entry in the version list that launches a native Rust client binary instead of the JVM.
3. LATER: re-enable Microsoft login when you say so.

## Rust edition performance
- rust-edition/Cargo.profile.toml + .cargo/config.toml: release build flags (LTO, abort-on-panic, A75 tuning).
- rust-edition/low-end.toml: default low-end settings; launcher loads it automatically on devices with <=6 GB RAM.

## Mods (both editions)
- oxide-mods/: Rust mod API + built-in mods (Zoom, Food Info, CPS, Keystrokes, Toggle Sprint, FPS). Mods can only touch HUD, camera and input, never the world. Run `cargo test` first.
- java-edition/: mod-catalog.json (Sodium, Iris shaders, AppleSkin, Zoomify...), ModrinthClient.kt (search/install/disable), ModsScreen.kt (Compose toggle list + Add button).
- oxide-mods/video-settings.layout.json: Sodium-style tabbed video settings.
Still to wire: launcher menu entry for ModsScreen, loading user-built Rust mods (manifest in oxide-mods/mod-manifest.example.json), shader pipeline for the Rust renderer.

## Optional Microsoft login
- java-edition/app/AccountPolicy.kt: AccountPolicy (saved on/off setting, default ON for testing) and MicrosoftOptionalToggle (Compose switch for the Accounts screen).
- To wire in: show MicrosoftOptionalToggle on the Accounts screen, and gate the Play button with AccountPolicy.canLaunch(...). Upstream's real Microsoft sign-in code stays, so turning the switch off brings it back.
