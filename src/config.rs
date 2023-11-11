// ------------ SETTINGS DECLARATION START ------------
#[derive(Debug)]
pub struct WindowSettings {
    pub title: &'static str,
    pub width: f32,
    pub height: f32,
}
// ------------------ DECLARATION END -----------------


// ------------------ SETTINGS BELOW ------------------

pub const WINDOW_SETTINGS: WindowSettings = WindowSettings {
    title: "One Bit Jam",
    width: 680.0,
    height: 680.0,
};