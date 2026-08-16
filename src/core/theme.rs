use freya::prelude::*;

// ============================================================
// THEME
// ============================================================

#[derive(Clone, Copy, Debug)]
pub struct Theme {
    pub colors: Colors,
    pub spacing: Spacing,
    pub radius: Radius,
    pub typography: Typography,
    pub sizes: Sizes,
    pub borders: Borders,
    pub shadows: Shadows,
    pub opacity: Opacity,
    pub animation: Animation,
    pub breakpoints: Breakpoints,
    pub containers: Containers,
    pub layers: Layers,
}

impl Theme {
    pub fn light() -> Self {
        Self {
            colors: Colors::light(),
            spacing: Spacing::default(),
            radius: Radius::default(),
            typography: Typography::default(),
            sizes: Sizes::default(),
            borders: Borders::default(),
            shadows: Shadows::default(),
            opacity: Opacity::default(),
            animation: Animation::default(),
            breakpoints: Breakpoints::default(),
            containers: Containers::default(),
            layers: Layers::default(),
        }
    }

    pub fn dark() -> Self {
        Self {
            colors: Colors::dark(),
            spacing: Spacing::default(),
            radius: Radius::default(),
            typography: Typography::default(),
            sizes: Sizes::default(),
            borders: Borders::default(),
            shadows: Shadows::default(),
            opacity: Opacity::default(),
            animation: Animation::default(),
            breakpoints: Breakpoints::default(),
            containers: Containers::default(),
            layers: Layers::default(),
        }
    }
}

// ============================================================
// COLORS
// ============================================================

#[derive(Clone, Copy, Debug)]
pub struct Colors {
    // Base
    pub background: Color,
    pub foreground: Color,

    // Surfaces
    pub card: Color,
    pub card_foreground: Color,

    pub popover: Color,
    pub popover_foreground: Color,

    // Primary
    pub primary: Color,
    pub primary_foreground: Color,

    // Secondary
    pub secondary: Color,
    pub secondary_foreground: Color,

    // Muted
    pub muted: Color,
    pub muted_foreground: Color,

    // Accent
    pub accent: Color,
    pub accent_foreground: Color,

    // Destructive
    pub destructive: Color,
    pub destructive_foreground: Color,

    // Form
    pub border: Color,
    pub input: Color,
    pub ring: Color,

    // Status
    pub success: Color,
    pub success_foreground: Color,

    pub warning: Color,
    pub warning_foreground: Color,

    pub info: Color,
    pub info_foreground: Color,

    // Charts
    pub chart_1: Color,
    pub chart_2: Color,
    pub chart_3: Color,
    pub chart_4: Color,
    pub chart_5: Color,

    // Sidebar
    pub sidebar: Color,
    pub sidebar_foreground: Color,
    pub sidebar_primary: Color,
    pub sidebar_primary_foreground: Color,
    pub sidebar_accent: Color,
    pub sidebar_accent_foreground: Color,
    pub sidebar_border: Color,
    pub sidebar_ring: Color,
}

impl Colors {
    pub fn light() -> Self {
        Self {
            background: Color::from_rgb(255, 255, 255),
            foreground: Color::from_rgb(9, 9, 11),

            card: Color::from_rgb(255, 255, 255),
            card_foreground: Color::from_rgb(9, 9, 11),

            popover: Color::from_rgb(255, 255, 255),
            popover_foreground: Color::from_rgb(9, 9, 11),

            primary: Color::from_rgb(24, 24, 27),
            primary_foreground: Color::from_rgb(250, 250, 250),

            secondary: Color::from_rgb(244, 244, 245),
            secondary_foreground: Color::from_rgb(24, 24, 27),

            muted: Color::from_rgb(244, 244, 245),
            muted_foreground: Color::from_rgb(113, 113, 122),

            accent: Color::from_rgb(244, 244, 245),
            accent_foreground: Color::from_rgb(24, 24, 27),

            destructive: Color::from_rgb(220, 38, 38),
            destructive_foreground: Color::from_rgb(250, 250, 250),

            border: Color::from_rgb(228, 228, 231),
            input: Color::from_rgb(228, 228, 231),
            ring: Color::from_rgb(24, 24, 27),

            success: Color::from_rgb(22, 163, 74),
            success_foreground: Color::from_rgb(250, 250, 250),

            warning: Color::from_rgb(202, 138, 4),
            warning_foreground: Color::from_rgb(24, 24, 27),

            info: Color::from_rgb(37, 99, 235),
            info_foreground: Color::from_rgb(250, 250, 250),

            chart_1: Color::from_rgb(234, 88, 12),
            chart_2: Color::from_rgb(14, 165, 233),
            chart_3: Color::from_rgb(34, 197, 94),
            chart_4: Color::from_rgb(168, 85, 247),
            chart_5: Color::from_rgb(236, 72, 153),

            sidebar: Color::from_rgb(250, 250, 250),
            sidebar_foreground: Color::from_rgb(24, 24, 27),
            sidebar_primary: Color::from_rgb(24, 24, 27),
            sidebar_primary_foreground: Color::from_rgb(250, 250, 250),
            sidebar_accent: Color::from_rgb(244, 244, 245),
            sidebar_accent_foreground: Color::from_rgb(24, 24, 27),
            sidebar_border: Color::from_rgb(228, 228, 231),
            sidebar_ring: Color::from_rgb(24, 24, 27),
        }
    }

    pub fn dark() -> Self {
        Self {
            background: Color::from_rgb(9, 9, 11),
            foreground: Color::from_rgb(250, 250, 250),

            card: Color::from_rgb(9, 9, 11),
            card_foreground: Color::from_rgb(250, 250, 250),

            popover: Color::from_rgb(9, 9, 11),
            popover_foreground: Color::from_rgb(250, 250, 250),

            primary: Color::from_rgb(250, 250, 250),
            primary_foreground: Color::from_rgb(24, 24, 27),

            secondary: Color::from_rgb(39, 39, 42),
            secondary_foreground: Color::from_rgb(250, 250, 250),

            muted: Color::from_rgb(39, 39, 42),
            muted_foreground: Color::from_rgb(161, 161, 170),

            accent: Color::from_rgb(39, 39, 42),
            accent_foreground: Color::from_rgb(250, 250, 250),

            destructive: Color::from_rgb(127, 29, 29),
            destructive_foreground: Color::from_rgb(250, 250, 250),

            border: Color::from_rgb(63, 63, 70),
            input: Color::from_rgb(63, 63, 70),
            ring: Color::from_rgb(212, 212, 216),

            success: Color::from_rgb(34, 197, 94),
            success_foreground: Color::from_rgb(250, 250, 250),

            warning: Color::from_rgb(234, 179, 8),
            warning_foreground: Color::from_rgb(24, 24, 27),

            info: Color::from_rgb(59, 130, 246),
            info_foreground: Color::from_rgb(250, 250, 250),

            chart_1: Color::from_rgb(249, 115, 22),
            chart_2: Color::from_rgb(56, 189, 248),
            chart_3: Color::from_rgb(74, 222, 128),
            chart_4: Color::from_rgb(192, 132, 252),
            chart_5: Color::from_rgb(244, 114, 182),

            sidebar: Color::from_rgb(24, 24, 27),
            sidebar_foreground: Color::from_rgb(250, 250, 250),
            sidebar_primary: Color::from_rgb(250, 250, 250),
            sidebar_primary_foreground: Color::from_rgb(24, 24, 27),
            sidebar_accent: Color::from_rgb(39, 39, 42),
            sidebar_accent_foreground: Color::from_rgb(250, 250, 250),
            sidebar_border: Color::from_rgb(63, 63, 70),
            sidebar_ring: Color::from_rgb(212, 212, 216),
        }
    }
}

// ============================================================
// SPACING
// ============================================================

#[derive(Clone, Copy, Debug)]
pub struct Spacing {
    pub px: f32,

    pub xs: f32,
    pub sm: f32,
    pub md: f32,
    pub lg: f32,
    pub xl: f32,
    pub xxl: f32,
    pub xxxl: f32,

    pub s1: f32,
    pub s2: f32,
    pub s3: f32,
    pub s4: f32,
    pub s5: f32,
    pub s6: f32,
    pub s8: f32,
    pub s10: f32,
    pub s12: f32,
    pub s16: f32,
    pub s20: f32,
    pub s24: f32,
}

impl Default for Spacing {
    fn default() -> Self {
        Self {
            px: 1.0,

            xs: 4.0,
            sm: 8.0,
            md: 12.0,
            lg: 16.0,
            xl: 24.0,
            xxl: 32.0,
            xxxl: 48.0,

            s1: 4.0,
            s2: 8.0,
            s3: 12.0,
            s4: 16.0,
            s5: 20.0,
            s6: 24.0,
            s8: 32.0,
            s10: 40.0,
            s12: 48.0,
            s16: 64.0,
            s20: 80.0,
            s24: 96.0,
        }
    }
}

// ============================================================
// RADIUS
// ============================================================

#[derive(Clone, Copy, Debug)]
pub struct Radius {
    pub none: f32,
    pub xs: f32,
    pub sm: f32,
    pub md: f32,
    pub lg: f32,
    pub xl: f32,
    pub full: f32,
}

impl Default for Radius {
    fn default() -> Self {
        Self {
            none: 0.0,
            xs: 2.0,
            sm: 4.0,
            md: 6.0,
            lg: 8.0,
            xl: 12.0,
            full: 9999.0,
        }
    }
}

// ============================================================
// TYPOGRAPHY
// ============================================================

#[derive(Clone, Copy, Debug)]
pub struct Typography {
    pub font_size: FontSizes,
    pub font_weight: FontWeights,
    pub line_height: LineHeights,
    pub letter_spacing: LetterSpacing,
}

impl Default for Typography {
    fn default() -> Self {
        Self {
            font_size: FontSizes::default(),
            font_weight: FontWeights::default(),
            line_height: LineHeights::default(),
            letter_spacing: LetterSpacing::default(),
        }
    }
}

// ------------------------------------------------------------
// FONT SIZES
// ------------------------------------------------------------

#[derive(Clone, Copy, Debug)]
pub struct FontSizes {
    pub xs: f32,
    pub sm: f32,
    pub base: f32,
    pub lg: f32,
    pub xl: f32,
    pub xxl: f32,
    pub xxxl: f32,

    pub display_sm: f32,
    pub display_md: f32,
    pub display_lg: f32,
    pub display_xl: f32,
}

impl Default for FontSizes {
    fn default() -> Self {
        Self {
            xs: 12.0,
            sm: 14.0,
            base: 16.0,
            lg: 18.0,
            xl: 20.0,
            xxl: 24.0,
            xxxl: 30.0,

            display_sm: 36.0,
            display_md: 48.0,
            display_lg: 60.0,
            display_xl: 72.0,
        }
    }
}

// ------------------------------------------------------------
// FONT WEIGHTS
// ------------------------------------------------------------

#[derive(Clone, Copy, Debug)]
pub struct FontWeights {
    pub thin: u16,
    pub extralight: u16,
    pub light: u16,
    pub normal: u16,
    pub medium: u16,
    pub semibold: u16,
    pub bold: u16,
    pub extrabold: u16,
    pub black: u16,
}

impl Default for FontWeights {
    fn default() -> Self {
        Self {
            thin: 100,
            extralight: 200,
            light: 300,
            normal: 400,
            medium: 500,
            semibold: 600,
            bold: 700,
            extrabold: 800,
            black: 900,
        }
    }
}

// ------------------------------------------------------------
// LINE HEIGHT
// ------------------------------------------------------------

#[derive(Clone, Copy, Debug)]
pub struct LineHeights {
    pub none: f32,
    pub tight: f32,
    pub snug: f32,
    pub normal: f32,
    pub relaxed: f32,
    pub loose: f32,
}

impl Default for LineHeights {
    fn default() -> Self {
        Self {
            none: 1.0,
            tight: 1.1,
            snug: 1.25,
            normal: 1.5,
            relaxed: 1.625,
            loose: 2.0,
        }
    }
}

// ------------------------------------------------------------
// LETTER SPACING
// ------------------------------------------------------------

#[derive(Clone, Copy, Debug)]
pub struct LetterSpacing {
    pub tighter: f32,
    pub tight: f32,
    pub normal: f32,
    pub wide: f32,
    pub wider: f32,
}

impl Default for LetterSpacing {
    fn default() -> Self {
        Self {
            tighter: -0.05,
            tight: -0.025,
            normal: 0.0,
            wide: 0.025,
            wider: 0.05,
        }
    }
}

// ============================================================
// SIZES
// ============================================================

#[derive(Clone, Copy, Debug)]
pub struct Sizes {
    // General
    pub xs: f32,
    pub sm: f32,
    pub md: f32,
    pub lg: f32,
    pub xl: f32,

    // Buttons
    pub button_xs: f32,
    pub button_sm: f32,
    pub button_default: f32,
    pub button_lg: f32,

    // Inputs
    pub input_xs: f32,
    pub input_sm: f32,
    pub input_default: f32,
    pub input_lg: f32,

    // Icons
    pub icon_xs: f32,
    pub icon_sm: f32,
    pub icon_default: f32,
    pub icon_lg: f32,
    pub icon_xl: f32,

    // Avatars
    pub avatar_xs: f32,
    pub avatar_sm: f32,
    pub avatar_default: f32,
    pub avatar_lg: f32,
    pub avatar_xl: f32,
}

impl Default for Sizes {
    fn default() -> Self {
        Self {
            xs: 24.0,
            sm: 32.0,
            md: 40.0,
            lg: 44.0,
            xl: 48.0,

            button_xs: 28.0,
            button_sm: 32.0,
            button_default: 40.0,
            button_lg: 44.0,

            input_xs: 28.0,
            input_sm: 32.0,
            input_default: 40.0,
            input_lg: 44.0,

            icon_xs: 12.0,
            icon_sm: 16.0,
            icon_default: 20.0,
            icon_lg: 24.0,
            icon_xl: 32.0,

            avatar_xs: 24.0,
            avatar_sm: 32.0,
            avatar_default: 40.0,
            avatar_lg: 48.0,
            avatar_xl: 64.0,
        }
    }
}

// ============================================================
// BORDERS
// ============================================================

#[derive(Clone, Copy, Debug)]
pub struct Borders {
    pub none: f32,
    pub thin: f32,
    pub medium: f32,
    pub thick: f32,
}

impl Default for Borders {
    fn default() -> Self {
        Self {
            none: 0.0,
            thin: 1.0,
            medium: 2.0,
            thick: 4.0,
        }
    }
}

// ============================================================
// SHADOWS
// ============================================================

#[derive(Clone, Copy, Debug)]
pub struct Shadows {
    pub none: Shadow,
    pub xs: Shadow,
    pub sm: Shadow,
    pub md: Shadow,
    pub lg: Shadow,
    pub xl: Shadow,
    pub xxl: Shadow,
}

#[derive(Clone, Copy, Debug)]
pub struct Shadow {
    pub offset_x: f32,
    pub offset_y: f32,
    pub blur: f32,
    pub spread: f32,
    pub opacity: f32,
}

impl Default for Shadows {
    fn default() -> Self {
        Self {
            none: Shadow {
                offset_x: 0.0,
                offset_y: 0.0,
                blur: 0.0,
                spread: 0.0,
                opacity: 0.0,
            },

            xs: Shadow {
                offset_x: 0.0,
                offset_y: 1.0,
                blur: 2.0,
                spread: 0.0,
                opacity: 0.05,
            },

            sm: Shadow {
                offset_x: 0.0,
                offset_y: 1.0,
                blur: 3.0,
                spread: 0.0,
                opacity: 0.08,
            },

            md: Shadow {
                offset_x: 0.0,
                offset_y: 4.0,
                blur: 6.0,
                spread: -1.0,
                opacity: 0.10,
            },

            lg: Shadow {
                offset_x: 0.0,
                offset_y: 10.0,
                blur: 15.0,
                spread: -3.0,
                opacity: 0.10,
            },

            xl: Shadow {
                offset_x: 0.0,
                offset_y: 20.0,
                blur: 25.0,
                spread: -5.0,
                opacity: 0.10,
            },

            xxl: Shadow {
                offset_x: 0.0,
                offset_y: 25.0,
                blur: 50.0,
                spread: -12.0,
                opacity: 0.20,
            },
        }
    }
}

// ============================================================
// OPACITY
// ============================================================

#[derive(Clone, Copy, Debug)]
pub struct Opacity {
    pub transparent: f32,
    pub subtle: f32,
    pub muted: f32,
    pub disabled: f32,
    pub overlay: f32,
    pub opaque: f32,
}

impl Default for Opacity {
    fn default() -> Self {
        Self {
            transparent: 0.0,
            subtle: 0.25,
            muted: 0.50,
            disabled: 0.50,
            overlay: 0.80,
            opaque: 1.0,
        }
    }
}

// ============================================================
// ANIMATION
// ============================================================

#[derive(Clone, Copy, Debug)]
pub struct Animation {
    pub instant: u32,
    pub fast: u32,
    pub normal: u32,
    pub slow: u32,
    pub slower: u32,

    pub easing: Easings,
}

#[derive(Clone, Copy, Debug)]
pub struct Easings {
    pub linear: Easing,
    pub ease_in: Easing,
    pub ease_out: Easing,
    pub ease_in_out: Easing,
}

#[derive(Clone, Copy, Debug)]
pub enum Easing {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
}

impl Default for Animation {
    fn default() -> Self {
        Self {
            instant: 0,
            fast: 100,
            normal: 200,
            slow: 300,
            slower: 500,

            easing: Easings {
                linear: Easing::Linear,
                ease_in: Easing::EaseIn,
                ease_out: Easing::EaseOut,
                ease_in_out: Easing::EaseInOut,
            },
        }
    }
}

// ============================================================
// BREAKPOINTS
// ============================================================

#[derive(Clone, Copy, Debug)]
pub struct Breakpoints {
    pub sm: f32,
    pub md: f32,
    pub lg: f32,
    pub xl: f32,
    pub xxl: f32,
}

impl Default for Breakpoints {
    fn default() -> Self {
        Self {
            sm: 640.0,
            md: 768.0,
            lg: 1024.0,
            xl: 1280.0,
            xxl: 1536.0,
        }
    }
}

// ============================================================
// CONTAINERS
// ============================================================

#[derive(Clone, Copy, Debug)]
pub struct Containers {
    pub xs: f32,
    pub sm: f32,
    pub md: f32,
    pub lg: f32,
    pub xl: f32,
    pub xxl: f32,
    pub full: f32,
}

impl Default for Containers {
    fn default() -> Self {
        Self {
            xs: 320.0,
            sm: 640.0,
            md: 768.0,
            lg: 1024.0,
            xl: 1280.0,
            xxl: 1536.0,
            full: f32::INFINITY,
        }
    }
}

// ============================================================
// LAYERS
// ============================================================

#[derive(Clone, Copy, Debug)]
pub struct Layers {
    pub base: i32,
    pub dropdown: i32,
    pub sticky: i32,
    pub fixed: i32,
    pub modal_backdrop: i32,
    pub modal: i32,
    pub popover: i32,
    pub tooltip: i32,
    pub toast: i32,
}

impl Default for Layers {
    fn default() -> Self {
        Self {
            base: 0,
            dropdown: 1000,
            sticky: 1100,
            fixed: 1200,
            modal_backdrop: 1300,
            modal: 1400,
            popover: 1500,
            tooltip: 1600,
            toast: 1700,
        }
    }
}