use bevy::window::{PresentMode, WindowMode};

pub const WINDOW_NAME: &str = "Jsp";
pub const APP_ID: &str = "voxel_game_id";

pub const DEFAULT_WINDOW_MODE: WindowMode = WindowMode::Windowed;
pub const DEFAULT_PRESENT_MODE: PresentMode = PresentMode::Fifo;


pub mod graphic_settings {
    use bevy::core_pipeline::bloom::BloomSettings;

    pub const DEFAULT_BLOOM_SETTINGS: BloomSettings = BloomSettings::NATURAL;
}