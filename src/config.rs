// ------------ SETTINGS DECLARATION START ------------
#[derive(Debug)]
pub struct WindowSettings {
    pub title: &'static str,
    pub width: f32,
    pub height: f32,
}

pub struct IntGridSettings {
    pub ground: i32,
}

pub struct PhysicsSettings {
    pub gravity: f32,
}
// ------------------ DECLARATION END -----------------


// ------------------ SETTINGS BELOW ------------------

pub const WINDOW_SETTINGS: WindowSettings = WindowSettings {
    title: "One Bit Jam",
    width: 680.0,
    height: 680.0,
};

pub const INT_GRID_SETTINGS: IntGridSettings = IntGridSettings {
    ground: 1,
};

pub const PHYSICS_SETTINGS: PhysicsSettings = PhysicsSettings {
    gravity: -1000.0
};