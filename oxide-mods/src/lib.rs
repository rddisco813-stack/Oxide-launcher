//! Oxide Rust Edition mod API.
//! Mods are limited to HUD, camera and input. They cannot touch the world.
pub mod builtin;
use std::{collections::HashMap, fs, io, path::Path};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Capability { Hud, Camera, Input } // no World capability on purpose

pub struct HudItem { pub x: i32, pub y: i32, pub text: String }

/// Per-frame data the client hands to mods.
#[derive(Default)]
pub struct Ctx {
    pub dt: f32,
    pub fps: u32,
    pub now_ms: u64,
    pub fov_scale: f32,          // mods may change this (1.0 = normal)
    pub sprint_held: bool,       // mods may set this
    pub zoom_held: bool,
    pub scroll: f32,
    pub left_click: bool,
    pub keys: [bool; 4],         // W A S D
    pub held_food: Option<(i32, f32)>, // (hunger restored, saturation)
    pub hud: Vec<HudItem>,
}

pub trait Mod {
    fn id(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn capabilities(&self) -> &'static [Capability];
    fn default_enabled(&self) -> bool { true }
    fn on_tick(&mut self, ctx: &mut Ctx);
}

pub struct ModManager { mods: Vec<(Box<dyn Mod>, bool)> }

impl ModManager {
    pub fn new() -> Self { Self { mods: vec![] } }
    pub fn with_builtin() -> Self {
        let mut m = Self::new();
        for b in builtin::all() { m.register(b); }
        m
    }
    pub fn register(&mut self, m: Box<dyn Mod>) { let e = m.default_enabled(); self.mods.push((m, e)); }
    pub fn set_enabled(&mut self, id: &str, on: bool) -> bool {
        for (m, e) in self.mods.iter_mut() { if m.id() == id { *e = on; return true; } }
        false
    }
    pub fn is_enabled(&self, id: &str) -> bool { self.mods.iter().any(|(m, e)| m.id() == id && *e) }
    /// (id, name, description, enabled) for the Mods screen.
    pub fn list(&self) -> Vec<(&'static str, &'static str, &'static str, bool)> {
        self.mods.iter().map(|(m, e)| (m.id(), m.name(), m.description(), *e)).collect()
    }
    pub fn tick(&mut self, ctx: &mut Ctx) {
        for (m, e) in self.mods.iter_mut() { if *e { m.on_tick(ctx); } }
    }
    /// Saved as simple `id=true/false` lines.
    pub fn save(&self, p: &Path) -> io::Result<()> {
        let s: String = self.mods.iter().map(|(m, e)| format!("{}={}\n", m.id(), e)).collect();
        fs::write(p, s)
    }
    pub fn load(&mut self, p: &Path) -> io::Result<()> {
        let txt = fs::read_to_string(p)?;
        let map: HashMap<&str, bool> = txt.lines()
            .filter_map(|l| l.split_once('=')).map(|(k, v)| (k, v == "true")).collect();
        for (m, e) in self.mods.iter_mut() { if let Some(v) = map.get(m.id()) { *e = *v; } }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn toggle_and_zoom() {
        let mut m = ModManager::with_builtin();
        let mut c = Ctx { fov_scale: 1.0, dt: 0.016, zoom_held: true, ..Default::default() };
        for _ in 0..120 { m.tick(&mut c); }
        assert!(c.fov_scale < 0.5);
        m.set_enabled("zoom", false);
        let mut c2 = Ctx { fov_scale: 1.0, dt: 0.016, zoom_held: true, ..Default::default() };
        m.tick(&mut c2);
        assert_eq!(c2.fov_scale, 1.0);
    }
    #[test]
    fn appleskin_shows_food() {
        let mut m = ModManager::with_builtin();
        let mut c = Ctx { held_food: Some((6, 7.2)), ..Default::default() };
        m.tick(&mut c);
        assert!(c.hud.iter().any(|h| h.text.contains("+6")));
    }
}
