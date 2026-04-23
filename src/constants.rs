use crate::funds::FundsAmount;

pub const STARTING_FUNDS: FundsAmount = 5000;

pub mod ui {
    use bevy::color::Srgba;

    pub const TEXTURE_EARTH_BACKGROUND: &str = "textures/earth_night.jpg";

    pub const FONT_DISPLAY_PATH: &str = "fonts/DancingScript-Variable.ttf";
    pub const FONT_PATH: &str = "fonts/Lora-Variable.ttf";
    // A font spanning more unicode code points than usual
    pub const UNICODE_FONT_PATH: &str = "fonts/DejaVuSans.ttf";

    /// A dark purple
    pub const MENU_BACKGROUND: Srgba = Srgba::rgb(0.2, 0.0, 0.2);
    pub const MENU_HOVER_BACKGROUND: Srgba = Srgba::rgb(0.3, 0.0, 0.3);
    pub const MENU_PRESSED_BACKGROUND: Srgba = Srgba::rgb(0.0, 0.0, 0.0);

    pub const TEXT: Srgba = Srgba::rgb(0.8, 0.8, 0.8);
    pub const TEXT_HIGHLIGHT: Srgba = Srgba::rgb(0.85, 0.85, 0.25);
}
