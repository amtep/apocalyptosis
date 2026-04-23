use crate::funds::FundsAmount;

pub const STARTING_FUNDS: FundsAmount = 5000;

pub mod ui {
    use bevy::color::Srgba;

    pub const TEXTURE_EARTH_BACKGROUND: &str = "textures/earth_night.jpg";

    pub const FONT_DISPLAY_PATH: &str = "fonts/DancingScript-Variable.ttf";
    pub const FONT_PATH: &str = "fonts/Lora-Variable.ttf";
    // A font spanning more unicode code points than usual
    pub const UNICODE_FONT_PATH: &str = "fonts/DejaVuSans.ttf";

    pub const DARK_PURPLE: Srgba = Srgba::rgb(0.102, 0.055, 0.243); // #1A0E3E
    pub const INDIGO: Srgba = Srgba::rgb(0.122, 0.102, 0.439); // #1F1A70
    pub const MAGENTA: Srgba = Srgba::rgb(0.859, 0.282, 0.545); // #DB488B
    pub const LIGHT_PINK: Srgba = Srgba::rgb(1.000, 0.514, 0.965); // #FF83F6
    pub const CYAN: Srgba = Srgba::rgb(0.243, 0.816, 0.922); // #3ED0EB
    pub const WHITE: Srgba = Srgba::rgb(0.878, 0.878, 0.878);
    pub const YELLOW: Srgba = Srgba::rgb(1.00, 1.00, 0.384);
    pub const BLACK: Srgba = Srgba::rgb(0.071, 0.071, 0.071);

    pub const MENU_BACKGROUND: Srgba = DARK_PURPLE;
    pub const MENU_HOVER_BACKGROUND: Srgba = INDIGO;
    pub const MENU_PRESSED_BACKGROUND: Srgba = BLACK;

    pub const TEXT: Srgba = WHITE;
    pub const BORDER: Srgba = WHITE;
    pub const TEXT_HIGHLIGHT: Srgba = YELLOW;
}
