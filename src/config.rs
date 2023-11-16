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

pub struct RapierConfig {
    pub pixels_per_meter: f32
}

pub struct PhysicsSettings {
    pub gravity: f32,
}

pub struct PlayerSettings {
    pub half_height: f32,
    pub radius: f32,
    pub friction: f32,
    pub x_velocity: f32,
    pub jump_velocity: f32,
    pub base_loot_factor: f32,
}

pub struct CameraSettings {
    pub look_distance: f32,
    pub target_rate: f32,
    pub snap_time: f32,
}

pub struct Constants {
    pub nano_per_mili: i32,
}

pub struct LdtkIntCellValues {
    pub walls: i32,
}

pub struct GameRules {
    pub starting_coins: i32,
}

// ------------------ DECLARATION END -----------------


// ------------------ SETTINGS BELOW ------------------

pub const WINDOW_SETTINGS: WindowSettings = WindowSettings {
    title: "One Bit Jam",
    width: 800.0,
    height: 600.0,
};

pub const INT_GRID_SETTINGS: IntGridSettings = IntGridSettings {
    ground: 1,
};

pub const RAPIER_CONFIG: RapierConfig = RapierConfig {
    pixels_per_meter: 100.0,
};

pub const PHYSICS_SETTINGS: PhysicsSettings = PhysicsSettings {
    gravity: -1000.0
};

pub const PLAYER_SETTINGS: PlayerSettings = PlayerSettings {
    half_height: 8.0,
    radius: 5.0,
    friction: 0.0,
    x_velocity: 150.0,
    jump_velocity: 300.0,
    base_loot_factor: 0.5,
};

pub const CAMERA_SETTINGS: CameraSettings = CameraSettings {
    look_distance: 32.0,
    target_rate: 0.01,
    snap_time: 0.5,
};

pub const CONSTANTS: Constants = Constants {
    nano_per_mili: 1000000,
};

pub const LDTK_INT_CELL_VALUES: LdtkIntCellValues = LdtkIntCellValues {
    walls: 1,
};

pub const LDTK_PLAYER_LEVEL: i32 = 0;

pub const GAME_RULES: GameRules = GameRules {
    starting_coins: 200,
};