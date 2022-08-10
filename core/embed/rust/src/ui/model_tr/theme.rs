use crate::ui::{
    component::text::{formatted::FormattedFonts, TextStyle},
    display::{Color, Font, IconAndName},
};

// Color palette.
pub const FG: Color = Color::white(); // Default foreground (text & icon) color.
pub const BG: Color = Color::black(); // Default background color.

// Font constants.
pub const FONT_NORMAL: Font = Font::new(-1);
pub const FONT_MEDIUM: Font = Font::new(-5);
pub const FONT_BOLD: Font = Font::new(-2);
pub const FONT_MONO: Font = Font::new(-3);

pub const FONT_BUTTON: Font = FONT_MONO;
pub const FONT_HEADER: Font = FONT_MONO;

pub const TEXT_NORMAL: TextStyle = TextStyle::new(FONT_NORMAL, FG, BG, FG, FG);
pub const TEXT_MEDIUM: TextStyle = TextStyle::new(FONT_MEDIUM, FG, BG, FG, FG);
pub const TEXT_BOLD: TextStyle = TextStyle::new(FONT_BOLD, FG, BG, FG, FG);
pub const TEXT_MONO: TextStyle = TextStyle::new(FONT_MONO, FG, BG, FG, FG);

pub const FORMATTED: FormattedFonts = FormattedFonts {
    normal: FONT_NORMAL,
    medium: FONT_MEDIUM,
    bold: FONT_BOLD,
    mono: FONT_MONO,
};

// Icons with their names for debugging purposes
pub const ICON_SUCCESS: IconAndName =
    IconAndName::new(include_res!("model_tr/res/success.toif"), "success");
pub const ICON_FAIL: IconAndName = IconAndName::new(include_res!("model_tr/res/fail.toif"), "fail");
pub const ICON_CANCEL_OUTLINE: IconAndName = IconAndName::new(
    include_res!("model_tr/res/cancel_for_outline.toif"),
    "cancel_outline",
); // 8*8
pub const ICON_CANCEL: IconAndName = IconAndName::new(
    include_res!("model_tr/res/cancel_no_outline.toif"),
    "cancel",
);
pub const ICON_ARM_LEFT: IconAndName =
    IconAndName::new(include_res!("model_tr/res/arm_left.toif"), "arm_left"); // 6*10
pub const ICON_ARM_RIGHT: IconAndName =
    IconAndName::new(include_res!("model_tr/res/arm_right.toif"), "arm_right"); // 6*10
pub const ICON_ARROW_LEFT: IconAndName =
    IconAndName::new(include_res!("model_tr/res/arrow_left.toif"), "arrow_left"); // 6*10
pub const ICON_ARROW_RIGHT: IconAndName =
    IconAndName::new(include_res!("model_tr/res/arrow_right.toif"), "arrow_right"); // 6*10
pub const ICON_ARROW_UP: IconAndName =
    IconAndName::new(include_res!("model_tr/res/arrow_up.toif"), "arrow_up"); // 10*6
pub const ICON_ARROW_DOWN: IconAndName =
    IconAndName::new(include_res!("model_tr/res/arrow_down.toif"), "arrow_down"); // 10*6
pub const ICON_BIN: IconAndName = IconAndName::new(include_res!("model_tr/res/bin.toif"), "bin"); // 10*10
pub const ICON_AMOUNT: IconAndName =
    IconAndName::new(include_res!("model_tr/res/amount.toif"), "amount"); // 10*10
pub const ICON_LOCK: IconAndName = IconAndName::new(include_res!("model_tr/res/lock.toif"), "lock"); // 10*10
pub const ICON_PARAM: IconAndName =
    IconAndName::new(include_res!("model_tr/res/param.toif"), "param"); // 10*10
pub const ICON_USER: IconAndName = IconAndName::new(include_res!("model_tr/res/user.toif"), "user"); // 10*10
pub const ICON_WALLET: IconAndName =
    IconAndName::new(include_res!("model_tr/res/wallet.toif"), "wallet"); // 10*10
pub const ICON_WARNING: IconAndName =
    IconAndName::new(include_res!("model_tr/res/warning.toif"), "warning"); // 12*12

// Button height is constant for both text and icon buttons.
// It is a combination of content and (optional) outline/border.
// It is not possible to have icons 7*7, therefore having 8*8
// with empty LEFT column and BOTTOM row.
pub const BUTTON_CONTENT_HEIGHT: i32 = 7;
pub const BUTTON_OUTLINE: i32 = 3;
pub const BUTTON_HEIGHT: i32 = BUTTON_CONTENT_HEIGHT + 2 * BUTTON_OUTLINE;
