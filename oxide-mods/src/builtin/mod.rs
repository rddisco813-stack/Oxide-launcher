use crate::{Capability as C, Ctx, HudItem, Mod};

pub fn all() -> Vec<Box<dyn Mod>> {
    vec![Box::new(Zoom { cur: 1.0, target: 0.25 }), Box::new(AppleSkin), Box::new(Cps { clicks: vec![] }),
         Box::new(Keystrokes), Box::new(ToggleSprint { on: false, was: false }), Box::new(FpsCounter)]
}

/// Hold the zoom key; scroll to adjust; smooth in/out.
pub struct Zoom { cur: f32, target: f32 }
impl Mod for Zoom {
    fn id(&self) -> &'static str { "zoom" }
    fn name(&self) -> &'static str { "Zoom" }
    fn description(&self) -> &'static str { "Hold the zoom button to zoom in. Scroll to adjust." }
    fn capabilities(&self) -> &'static [C] { &[C::Camera, C::Input] }
    fn on_tick(&mut self, c: &mut Ctx) {
        if c.zoom_held { self.target = (self.target - c.scroll * 0.05).clamp(0.05, 0.9); }
        let goal = if c.zoom_held { self.target } else { 1.0 };
        self.cur += (goal - self.cur) * (1.0 - (-12.0 * c.dt).exp());
        c.fov_scale *= self.cur;
    }
}

/// Shows how much hunger/saturation the held food gives.
pub struct AppleSkin;
impl Mod for AppleSkin {
    fn id(&self) -> &'static str { "appleskin" }
    fn name(&self) -> &'static str { "Food Info" }
    fn description(&self) -> &'static str { "Shows hunger and saturation restored by held food." }
    fn capabilities(&self) -> &'static [C] { &[C::Hud] }
    fn on_tick(&mut self, c: &mut Ctx) {
        if let Some((h, s)) = c.held_food {
            c.hud.push(HudItem { x: 10, y: 60, text: format!("Hunger +{}  Saturation +{:.1}", h, s) });
        }
    }
}

pub struct Cps { clicks: Vec<u64> }
impl Mod for Cps {
    fn id(&self) -> &'static str { "cps" }
    fn name(&self) -> &'static str { "CPS Counter" }
    fn description(&self) -> &'static str { "Clicks per second (PvP)." }
    fn capabilities(&self) -> &'static [C] { &[C::Hud, C::Input] }
    fn on_tick(&mut self, c: &mut Ctx) {
        if c.left_click { self.clicks.push(c.now_ms); }
        self.clicks.retain(|t| c.now_ms.saturating_sub(*t) < 1000);
        c.hud.push(HudItem { x: 10, y: 30, text: format!("CPS: {}", self.clicks.len()) });
    }
}

pub struct Keystrokes;
impl Mod for Keystrokes {
    fn id(&self) -> &'static str { "keystrokes" }
    fn name(&self) -> &'static str { "Keystrokes" }
    fn description(&self) -> &'static str { "Shows W A S D while pressed." }
    fn capabilities(&self) -> &'static [C] { &[C::Hud] }
    fn on_tick(&mut self, c: &mut Ctx) {
        let l = ['W', 'A', 'S', 'D'];
        let s: String = (0..4).map(|i| if c.keys[i] { format!("[{}]", l[i]) } else { " . ".into() }).collect();
        c.hud.push(HudItem { x: 10, y: 90, text: s });
    }
}

pub struct ToggleSprint { on: bool, was: bool }
impl Mod for ToggleSprint {
    fn id(&self) -> &'static str { "toggle_sprint" }
    fn name(&self) -> &'static str { "Toggle Sprint" }
    fn description(&self) -> &'static str { "Tap sprint once to keep sprinting." }
    fn capabilities(&self) -> &'static [C] { &[C::Input] }
    fn default_enabled(&self) -> bool { false }
    fn on_tick(&mut self, c: &mut Ctx) {
        if c.sprint_held && !self.was { self.on = !self.on; }
        self.was = c.sprint_held;
        c.sprint_held = self.on;
    }
}

pub struct FpsCounter;
impl Mod for FpsCounter {
    fn id(&self) -> &'static str { "fps" }
    fn name(&self) -> &'static str { "FPS Counter" }
    fn description(&self) -> &'static str { "Shows frames per second." }
    fn capabilities(&self) -> &'static [C] { &[C::Hud] }
    fn on_tick(&mut self, c: &mut Ctx) {
        c.hud.push(HudItem { x: 10, y: 10, text: format!("{} FPS", c.fps) });
    }
}
