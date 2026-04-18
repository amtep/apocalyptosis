pub const STARTING_FUNDS: i64 = 5000;

pub mod ui {
    use bevy::color::Srgba;

    pub const FONT_DISPLAY_PATH: &str = "fonts/DancingScript-Variable.ttf";
    pub const FONT_PATH: &str = "fonts/Lora-Variable.ttf";
    // A font spanning more unicode code points than usual
    pub const UNICODE_FONT_PATH: &str = "fonts/DejaVuSans.ttf";

    /// A dark purple
    pub const MENU_BACKGROUND: Srgba = Srgba::rgb(0.2, 0.0, 0.2);
}
