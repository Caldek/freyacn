use crate::core::theme::Theme;
use freya::prelude::Color;

/// Macro to generate background color methods.
macro_rules! bg_color {
    ($($method:ident => $field:ident),* $(,)?) => {
        $(
            fn $method(self, theme: &Theme) -> Self {
                self.background(theme.colors.$field)
            }
        )*
    };
}

/// Macro to generate foreground (text) color methods.
macro_rules! fg_color {
    ($($method:ident => $field:ident),* $(,)?) => {
        $(
            fn $method(self, theme: &Theme) -> Self {
                self.color(theme.colors.$field)
            }
        )*
    };
}

pub trait CNExt: Sized {
    /// Set the background color explicitly.
    fn background(self, color: Color) -> Self;
    /// Set the foreground (text) color explicitly.
    fn color(self, color: Color) -> Self;
    /// Alias for `color`.
    fn foreground(self, color: Color) -> Self {
        self.color(color)
    }

    // --------------------------------------------------------------
    // Background palette helpers – covers ALL colors in `Colors`.
    // --------------------------------------------------------------
    bg_color! {
        // Neutrals
        bg_slate_50 => slate_50,
        bg_slate_100 => slate_100,
        bg_slate_200 => slate_200,
        bg_slate_300 => slate_300,
        bg_slate_400 => slate_400,
        bg_slate_500 => slate_500,
        bg_slate_600 => slate_600,
        bg_slate_700 => slate_700,
        bg_slate_800 => slate_800,
        bg_slate_900 => slate_900,
        bg_slate_950 => slate_950,

        bg_gray_50 => gray_50,
        bg_gray_100 => gray_100,
        bg_gray_200 => gray_200,
        bg_gray_300 => gray_300,
        bg_gray_400 => gray_400,
        bg_gray_500 => gray_500,
        bg_gray_600 => gray_600,
        bg_gray_700 => gray_700,
        bg_gray_800 => gray_800,
        bg_gray_900 => gray_900,
        bg_gray_950 => gray_950,

        bg_zinc_50 => zinc_50,
        bg_zinc_100 => zinc_100,
        bg_zinc_200 => zinc_200,
        bg_zinc_300 => zinc_300,
        bg_zinc_400 => zinc_400,
        bg_zinc_500 => zinc_500,
        bg_zinc_600 => zinc_600,
        bg_zinc_700 => zinc_700,
        bg_zinc_800 => zinc_800,
        bg_zinc_900 => zinc_900,
        bg_zinc_950 => zinc_950,

        bg_neutral_50 => neutral_50,
        bg_neutral_100 => neutral_100,
        bg_neutral_200 => neutral_200,
        bg_neutral_300 => neutral_300,
        bg_neutral_400 => neutral_400,
        bg_neutral_500 => neutral_500,
        bg_neutral_600 => neutral_600,
        bg_neutral_700 => neutral_700,
        bg_neutral_800 => neutral_800,
        bg_neutral_900 => neutral_900,
        bg_neutral_950 => neutral_950,

        // Stone, Mauve, Olive, Mist, Taupe
        bg_stone_50 => stone_50,
        bg_stone_100 => stone_100,
        bg_stone_200 => stone_200,
        bg_stone_300 => stone_300,
        bg_stone_400 => stone_400,
        bg_stone_500 => stone_500,
        bg_stone_600 => stone_600,
        bg_stone_700 => stone_700,
        bg_stone_800 => stone_800,
        bg_stone_900 => stone_900,
        bg_stone_950 => stone_950,

        bg_mauve_50 => mauve_50,
        bg_mauve_100 => mauve_100,
        bg_mauve_200 => mauve_200,
        bg_mauve_300 => mauve_300,
        bg_mauve_400 => mauve_400,
        bg_mauve_500 => mauve_500,
        bg_mauve_600 => mauve_600,
        bg_mauve_700 => mauve_700,
        bg_mauve_800 => mauve_800,
        bg_mauve_900 => mauve_900,
        bg_mauve_950 => mauve_950,

        bg_olive_50 => olive_50,
        bg_olive_100 => olive_100,
        bg_olive_200 => olive_200,
        bg_olive_300 => olive_300,
        bg_olive_400 => olive_400,
        bg_olive_500 => olive_500,
        bg_olive_600 => olive_600,
        bg_olive_700 => olive_700,
        bg_olive_800 => olive_800,
        bg_olive_900 => olive_900,
        bg_olive_950 => olive_950,

        bg_mist_50 => mist_50,
        bg_mist_100 => mist_100,
        bg_mist_200 => mist_200,
        bg_mist_300 => mist_300,
        bg_mist_400 => mist_400,
        bg_mist_500 => mist_500,
        bg_mist_600 => mist_600,
        bg_mist_700 => mist_700,
        bg_mist_800 => mist_800,
        bg_mist_900 => mist_900,
        bg_mist_950 => mist_950,

        bg_taupe_50 => taupe_50,
        bg_taupe_100 => taupe_100,
        bg_taupe_200 => taupe_200,
        bg_taupe_300 => taupe_300,
        bg_taupe_400 => taupe_400,
        bg_taupe_500 => taupe_500,
        bg_taupe_600 => taupe_600,
        bg_taupe_700 => taupe_700,
        bg_taupe_800 => taupe_800,
        bg_taupe_900 => taupe_900,
        bg_taupe_950 => taupe_950,

        // Accent colors
        bg_red_50 => red_50,
        bg_red_100 => red_100,
        bg_red_200 => red_200,
        bg_red_300 => red_300,
        bg_red_400 => red_400,
        bg_red_500 => red_500,
        bg_red_600 => red_600,
        bg_red_700 => red_700,
        bg_red_800 => red_800,
        bg_red_900 => red_900,
        bg_red_950 => red_950,

        bg_orange_50 => orange_50,
        bg_orange_100 => orange_100,
        bg_orange_200 => orange_200,
        bg_orange_300 => orange_300,
        bg_orange_400 => orange_400,
        bg_orange_500 => orange_500,
        bg_orange_600 => orange_600,
        bg_orange_700 => orange_700,
        bg_orange_800 => orange_800,
        bg_orange_900 => orange_900,
        bg_orange_950 => orange_950,

        bg_amber_50 => amber_50,
        bg_amber_100 => amber_100,
        bg_amber_200 => amber_200,
        bg_amber_300 => amber_300,
        bg_amber_400 => amber_400,
        bg_amber_500 => amber_500,
        bg_amber_600 => amber_600,
        bg_amber_700 => amber_700,
        bg_amber_800 => amber_800,
        bg_amber_900 => amber_900,
        bg_amber_950 => amber_950,

        bg_yellow_50 => yellow_50,
        bg_yellow_100 => yellow_100,
        bg_yellow_200 => yellow_200,
        bg_yellow_300 => yellow_300,
        bg_yellow_400 => yellow_400,
        bg_yellow_500 => yellow_500,
        bg_yellow_600 => yellow_600,
        bg_yellow_700 => yellow_700,
        bg_yellow_800 => yellow_800,
        bg_yellow_900 => yellow_900,
        bg_yellow_950 => yellow_950,

        bg_lime_50 => lime_50,
        bg_lime_100 => lime_100,
        bg_lime_200 => lime_200,
        bg_lime_300 => lime_300,
        bg_lime_400 => lime_400,
        bg_lime_500 => lime_500,
        bg_lime_600 => lime_600,
        bg_lime_700 => lime_700,
        bg_lime_800 => lime_800,
        bg_lime_900 => lime_900,
        bg_lime_950 => lime_950,

        bg_green_50 => green_50,
        bg_green_100 => green_100,
        bg_green_200 => green_200,
        bg_green_300 => green_300,
        bg_green_400 => green_400,
        bg_green_500 => green_500,
        bg_green_600 => green_600,
        bg_green_700 => green_700,
        bg_green_800 => green_800,
        bg_green_900 => green_900,
        bg_green_950 => green_950,

        bg_emerald_50 => emerald_50,
        bg_emerald_100 => emerald_100,
        bg_emerald_200 => emerald_200,
        bg_emerald_300 => emerald_300,
        bg_emerald_400 => emerald_400,
        bg_emerald_500 => emerald_500,
        bg_emerald_600 => emerald_600,
        bg_emerald_700 => emerald_700,
        bg_emerald_800 => emerald_800,
        bg_emerald_900 => emerald_900,
        bg_emerald_950 => emerald_950,

        bg_teal_50 => teal_50,
        bg_teal_100 => teal_100,
        bg_teal_200 => teal_200,
        bg_teal_300 => teal_300,
        bg_teal_400 => teal_400,
        bg_teal_500 => teal_500,
        bg_teal_600 => teal_600,
        bg_teal_700 => teal_700,
        bg_teal_800 => teal_800,
        bg_teal_900 => teal_900,
        bg_teal_950 => teal_950,

        bg_cyan_50 => cyan_50,
        bg_cyan_100 => cyan_100,
        bg_cyan_200 => cyan_200,
        bg_cyan_300 => cyan_300,
        bg_cyan_400 => cyan_400,
        bg_cyan_500 => cyan_500,
        bg_cyan_600 => cyan_600,
        bg_cyan_700 => cyan_700,
        bg_cyan_800 => cyan_800,
        bg_cyan_900 => cyan_900,
        bg_cyan_950 => cyan_950,

        bg_sky_50 => sky_50,
        bg_sky_100 => sky_100,
        bg_sky_200 => sky_200,
        bg_sky_300 => sky_300,
        bg_sky_400 => sky_400,
        bg_sky_500 => sky_500,
        bg_sky_600 => sky_600,
        bg_sky_700 => sky_700,
        bg_sky_800 => sky_800,
        bg_sky_900 => sky_900,
        bg_sky_950 => sky_950,

        bg_blue_50 => blue_50,
        bg_blue_100 => blue_100,
        bg_blue_200 => blue_200,
        bg_blue_300 => blue_300,
        bg_blue_400 => blue_400,
        bg_blue_500 => blue_500,
        bg_blue_600 => blue_600,
        bg_blue_700 => blue_700,
        bg_blue_800 => blue_800,
        bg_blue_900 => blue_900,
        bg_blue_950 => blue_950,

        bg_indigo_50 => indigo_50,
        bg_indigo_100 => indigo_100,
        bg_indigo_200 => indigo_200,
        bg_indigo_300 => indigo_300,
        bg_indigo_400 => indigo_400,
        bg_indigo_500 => indigo_500,
        bg_indigo_600 => indigo_600,
        bg_indigo_700 => indigo_700,
        bg_indigo_800 => indigo_800,
        bg_indigo_900 => indigo_900,
        bg_indigo_950 => indigo_950,

        bg_violet_50 => violet_50,
        bg_violet_100 => violet_100,
        bg_violet_200 => violet_200,
        bg_violet_300 => violet_300,
        bg_violet_400 => violet_400,
        bg_violet_500 => violet_500,
        bg_violet_600 => violet_600,
        bg_violet_700 => violet_700,
        bg_violet_800 => violet_800,
        bg_violet_900 => violet_900,
        bg_violet_950 => violet_950,

        bg_purple_50 => purple_50,
        bg_purple_100 => purple_100,
        bg_purple_200 => purple_200,
        bg_purple_300 => purple_300,
        bg_purple_400 => purple_400,
        bg_purple_500 => purple_500,
        bg_purple_600 => purple_600,
        bg_purple_700 => purple_700,
        bg_purple_800 => purple_800,
        bg_purple_900 => purple_900,
        bg_purple_950 => purple_950,

        bg_fuchsia_50 => fuchsia_50,
        bg_fuchsia_100 => fuchsia_100,
        bg_fuchsia_200 => fuchsia_200,
        bg_fuchsia_300 => fuchsia_300,
        bg_fuchsia_400 => fuchsia_400,
        bg_fuchsia_500 => fuchsia_500,
        bg_fuchsia_600 => fuchsia_600,
        bg_fuchsia_700 => fuchsia_700,
        bg_fuchsia_800 => fuchsia_800,
        bg_fuchsia_900 => fuchsia_900,
        bg_fuchsia_950 => fuchsia_950,

        bg_pink_50 => pink_50,
        bg_pink_100 => pink_100,
        bg_pink_200 => pink_200,
        bg_pink_300 => pink_300,
        bg_pink_400 => pink_400,
        bg_pink_500 => pink_500,
        bg_pink_600 => pink_600,
        bg_pink_700 => pink_700,
        bg_pink_800 => pink_800,
        bg_pink_900 => pink_900,
        bg_pink_950 => pink_950,

        bg_rose_50 => rose_50,
        bg_rose_100 => rose_100,
        bg_rose_200 => rose_200,
        bg_rose_300 => rose_300,
        bg_rose_400 => rose_400,
        bg_rose_500 => rose_500,
        bg_rose_600 => rose_600,
        bg_rose_700 => rose_700,
        bg_rose_800 => rose_800,
        bg_rose_900 => rose_900,
        bg_rose_950 => rose_950,
    }

    // --------------------------------------------------------------
    // Foreground (text) palette helpers – covers ALL colors in `Colors`.
    // --------------------------------------------------------------
    fg_color! {
        // Neutrals
        fg_slate_50 => slate_50,
        fg_slate_100 => slate_100,
        fg_slate_200 => slate_200,
        fg_slate_300 => slate_300,
        fg_slate_400 => slate_400,
        fg_slate_500 => slate_500,
        fg_slate_600 => slate_600,
        fg_slate_700 => slate_700,
        fg_slate_800 => slate_800,
        fg_slate_900 => slate_900,
        fg_slate_950 => slate_950,

        fg_gray_50 => gray_50,
        fg_gray_100 => gray_100,
        fg_gray_200 => gray_200,
        fg_gray_300 => gray_300,
        fg_gray_400 => gray_400,
        fg_gray_500 => gray_500,
        fg_gray_600 => gray_600,
        fg_gray_700 => gray_700,
        fg_gray_800 => gray_800,
        fg_gray_900 => gray_900,
        fg_gray_950 => gray_950,

        fg_zinc_50 => zinc_50,
        fg_zinc_100 => zinc_100,
        fg_zinc_200 => zinc_200,
        fg_zinc_300 => zinc_300,
        fg_zinc_400 => zinc_400,
        fg_zinc_500 => zinc_500,
        fg_zinc_600 => zinc_600,
        fg_zinc_700 => zinc_700,
        fg_zinc_800 => zinc_800,
        fg_zinc_900 => zinc_900,
        fg_zinc_950 => zinc_950,

        fg_neutral_50 => neutral_50,
        fg_neutral_100 => neutral_100,
        fg_neutral_200 => neutral_200,
        fg_neutral_300 => neutral_300,
        fg_neutral_400 => neutral_400,
        fg_neutral_500 => neutral_500,
        fg_neutral_600 => neutral_600,
        fg_neutral_700 => neutral_700,
        fg_neutral_800 => neutral_800,
        fg_neutral_900 => neutral_900,
        fg_neutral_950 => neutral_950,

        // Stone, Mauve, Olive, Mist, Taupe
        fg_stone_50 => stone_50,
        fg_stone_100 => stone_100,
        fg_stone_200 => stone_200,
        fg_stone_300 => stone_300,
        fg_stone_400 => stone_400,
        fg_stone_500 => stone_500,
        fg_stone_600 => stone_600,
        fg_stone_700 => stone_700,
        fg_stone_800 => stone_800,
        fg_stone_900 => stone_900,
        fg_stone_950 => stone_950,

        fg_mauve_50 => mauve_50,
        fg_mauve_100 => mauve_100,
        fg_mauve_200 => mauve_200,
        fg_mauve_300 => mauve_300,
        fg_mauve_400 => mauve_400,
        fg_mauve_500 => mauve_500,
        fg_mauve_600 => mauve_600,
        fg_mauve_700 => mauve_700,
        fg_mauve_800 => mauve_800,
        fg_mauve_900 => mauve_900,
        fg_mauve_950 => mauve_950,

        fg_olive_50 => olive_50,
        fg_olive_100 => olive_100,
        fg_olive_200 => olive_200,
        fg_olive_300 => olive_300,
        fg_olive_400 => olive_400,
        fg_olive_500 => olive_500,
        fg_olive_600 => olive_600,
        fg_olive_700 => olive_700,
        fg_olive_800 => olive_800,
        fg_olive_900 => olive_900,
        fg_olive_950 => olive_950,

        fg_mist_50 => mist_50,
        fg_mist_100 => mist_100,
        fg_mist_200 => mist_200,
        fg_mist_300 => mist_300,
        fg_mist_400 => mist_400,
        fg_mist_500 => mist_500,
        fg_mist_600 => mist_600,
        fg_mist_700 => mist_700,
        fg_mist_800 => mist_800,
        fg_mist_900 => mist_900,
        fg_mist_950 => mist_950,

        fg_taupe_50 => taupe_50,
        fg_taupe_100 => taupe_100,
        fg_taupe_200 => taupe_200,
        fg_taupe_300 => taupe_300,
        fg_taupe_400 => taupe_400,
        fg_taupe_500 => taupe_500,
        fg_taupe_600 => taupe_600,
        fg_taupe_700 => taupe_700,
        fg_taupe_800 => taupe_800,
        fg_taupe_900 => taupe_900,
        fg_taupe_950 => taupe_950,

        // Accent colors
        fg_red_50 => red_50,
        fg_red_100 => red_100,
        fg_red_200 => red_200,
        fg_red_300 => red_300,
        fg_red_400 => red_400,
        fg_red_500 => red_500,
        fg_red_600 => red_600,
        fg_red_700 => red_700,
        fg_red_800 => red_800,
        fg_red_900 => red_900,
        fg_red_950 => red_950,

        fg_orange_50 => orange_50,
        fg_orange_100 => orange_100,
        fg_orange_200 => orange_200,
        fg_orange_300 => orange_300,
        fg_orange_400 => orange_400,
        fg_orange_500 => orange_500,
        fg_orange_600 => orange_600,
        fg_orange_700 => orange_700,
        fg_orange_800 => orange_800,
        fg_orange_900 => orange_900,
        fg_orange_950 => orange_950,

        fg_amber_50 => amber_50,
        fg_amber_100 => amber_100,
        fg_amber_200 => amber_200,
        fg_amber_300 => amber_300,
        fg_amber_400 => amber_400,
        fg_amber_500 => amber_500,
        fg_amber_600 => amber_600,
        fg_amber_700 => amber_700,
        fg_amber_800 => amber_800,
        fg_amber_900 => amber_900,
        fg_amber_950 => amber_950,

        fg_yellow_50 => yellow_50,
        fg_yellow_100 => yellow_100,
        fg_yellow_200 => yellow_200,
        fg_yellow_300 => yellow_300,
        fg_yellow_400 => yellow_400,
        fg_yellow_500 => yellow_500,
        fg_yellow_600 => yellow_600,
        fg_yellow_700 => yellow_700,
        fg_yellow_800 => yellow_800,
        fg_yellow_900 => yellow_900,
        fg_yellow_950 => yellow_950,

        fg_lime_50 => lime_50,
        fg_lime_100 => lime_100,
        fg_lime_200 => lime_200,
        fg_lime_300 => lime_300,
        fg_lime_400 => lime_400,
        fg_lime_500 => lime_500,
        fg_lime_600 => lime_600,
        fg_lime_700 => lime_700,
        fg_lime_800 => lime_800,
        fg_lime_900 => lime_900,
        fg_lime_950 => lime_950,

        fg_green_50 => green_50,
        fg_green_100 => green_100,
        fg_green_200 => green_200,
        fg_green_300 => green_300,
        fg_green_400 => green_400,
        fg_green_500 => green_500,
        fg_green_600 => green_600,
        fg_green_700 => green_700,
        fg_green_800 => green_800,
        fg_green_900 => green_900,
        fg_green_950 => green_950,

        fg_emerald_50 => emerald_50,
        fg_emerald_100 => emerald_100,
        fg_emerald_200 => emerald_200,
        fg_emerald_300 => emerald_300,
        fg_emerald_400 => emerald_400,
        fg_emerald_500 => emerald_500,
        fg_emerald_600 => emerald_600,
        fg_emerald_700 => emerald_700,
        fg_emerald_800 => emerald_800,
        fg_emerald_900 => emerald_900,
        fg_emerald_950 => emerald_950,

        fg_teal_50 => teal_50,
        fg_teal_100 => teal_100,
        fg_teal_200 => teal_200,
        fg_teal_300 => teal_300,
        fg_teal_400 => teal_400,
        fg_teal_500 => teal_500,
        fg_teal_600 => teal_600,
        fg_teal_700 => teal_700,
        fg_teal_800 => teal_800,
        fg_teal_900 => teal_900,
        fg_teal_950 => teal_950,

        fg_cyan_50 => cyan_50,
        fg_cyan_100 => cyan_100,
        fg_cyan_200 => cyan_200,
        fg_cyan_300 => cyan_300,
        fg_cyan_400 => cyan_400,
        fg_cyan_500 => cyan_500,
        fg_cyan_600 => cyan_600,
        fg_cyan_700 => cyan_700,
        fg_cyan_800 => cyan_800,
        fg_cyan_900 => cyan_900,
        fg_cyan_950 => cyan_950,

        fg_sky_50 => sky_50,
        fg_sky_100 => sky_100,
        fg_sky_200 => sky_200,
        fg_sky_300 => sky_300,
        fg_sky_400 => sky_400,
        fg_sky_500 => sky_500,
        fg_sky_600 => sky_600,
        fg_sky_700 => sky_700,
        fg_sky_800 => sky_800,
        fg_sky_900 => sky_900,
        fg_sky_950 => sky_950,

        fg_blue_50 => blue_50,
        fg_blue_100 => blue_100,
        fg_blue_200 => blue_200,
        fg_blue_300 => blue_300,
        fg_blue_400 => blue_400,
        fg_blue_500 => blue_500,
        fg_blue_600 => blue_600,
        fg_blue_700 => blue_700,
        fg_blue_800 => blue_800,
        fg_blue_900 => blue_900,
        fg_blue_950 => blue_950,

        fg_indigo_50 => indigo_50,
        fg_indigo_100 => indigo_100,
        fg_indigo_200 => indigo_200,
        fg_indigo_300 => indigo_300,
        fg_indigo_400 => indigo_400,
        fg_indigo_500 => indigo_500,
        fg_indigo_600 => indigo_600,
        fg_indigo_700 => indigo_700,
        fg_indigo_800 => indigo_800,
        fg_indigo_900 => indigo_900,
        fg_indigo_950 => indigo_950,

        fg_violet_50 => violet_50,
        fg_violet_100 => violet_100,
        fg_violet_200 => violet_200,
        fg_violet_300 => violet_300,
        fg_violet_400 => violet_400,
        fg_violet_500 => violet_500,
        fg_violet_600 => violet_600,
        fg_violet_700 => violet_700,
        fg_violet_800 => violet_800,
        fg_violet_900 => violet_900,
        fg_violet_950 => violet_950,

        fg_purple_50 => purple_50,
        fg_purple_100 => purple_100,
        fg_purple_200 => purple_200,
        fg_purple_300 => purple_300,
        fg_purple_400 => purple_400,
        fg_purple_500 => purple_500,
        fg_purple_600 => purple_600,
        fg_purple_700 => purple_700,
        fg_purple_800 => purple_800,
        fg_purple_900 => purple_900,
        fg_purple_950 => purple_950,

        fg_fuchsia_50 => fuchsia_50,
        fg_fuchsia_100 => fuchsia_100,
        fg_fuchsia_200 => fuchsia_200,
        fg_fuchsia_300 => fuchsia_300,
        fg_fuchsia_400 => fuchsia_400,
        fg_fuchsia_500 => fuchsia_500,
        fg_fuchsia_600 => fuchsia_600,
        fg_fuchsia_700 => fuchsia_700,
        fg_fuchsia_800 => fuchsia_800,
        fg_fuchsia_900 => fuchsia_900,
        fg_fuchsia_950 => fuchsia_950,

        fg_pink_50 => pink_50,
        fg_pink_100 => pink_100,
        fg_pink_200 => pink_200,
        fg_pink_300 => pink_300,
        fg_pink_400 => pink_400,
        fg_pink_500 => pink_500,
        fg_pink_600 => pink_600,
        fg_pink_700 => pink_700,
        fg_pink_800 => pink_800,
        fg_pink_900 => pink_900,
        fg_pink_950 => pink_950,

        fg_rose_50 => rose_50,
        fg_rose_100 => rose_100,
        fg_rose_200 => rose_200,
        fg_rose_300 => rose_300,
        fg_rose_400 => rose_400,
        fg_rose_500 => rose_500,
        fg_rose_600 => rose_600,
        fg_rose_700 => rose_700,
        fg_rose_800 => rose_800,
        fg_rose_900 => rose_900,
        fg_rose_950 => rose_950,
    }

    // --------------------------------------------------------------
    // Semantic foreground (text) colors – map directly to Theme fields.
    // --------------------------------------------------------------

    fn fg_background(self, theme: &Theme) -> Self {
        self.color(theme.background)
    }
    fn fg_foreground(self, theme: &Theme) -> Self {
        self.color(theme.foreground)
    }

    fn fg_primary(self, theme: &Theme) -> Self {
        self.color(theme.primary)
    }
    fn fg_primary_foreground(self, theme: &Theme) -> Self {
        self.color(theme.primary_foreground)
    }

    fn fg_secondary(self, theme: &Theme) -> Self {
        self.color(theme.secondary)
    }
    fn fg_secondary_foreground(self, theme: &Theme) -> Self {
        self.color(theme.secondary_foreground)
    }

    fn fg_muted(self, theme: &Theme) -> Self {
        self.color(theme.muted)
    }
    fn fg_muted_foreground(self, theme: &Theme) -> Self {
        self.color(theme.muted_foreground)
    }

    fn fg_accent(self, theme: &Theme) -> Self {
        self.color(theme.accent)
    }
    fn fg_accent_foreground(self, theme: &Theme) -> Self {
        self.color(theme.accent_foreground)
    }

    fn fg_destructive(self, theme: &Theme) -> Self {
        self.color(theme.destructive)
    }
    fn fg_destructive_foreground(self, theme: &Theme) -> Self {
        self.color(theme.destructive_foreground)
    }

    fn fg_card(self, theme: &Theme) -> Self {
        self.color(theme.card)
    }
    fn fg_card_foreground(self, theme: &Theme) -> Self {
        self.color(theme.card_foreground)
    }

    fn fg_popover(self, theme: &Theme) -> Self {
        self.color(theme.popover)
    }
    fn fg_popover_foreground(self, theme: &Theme) -> Self {
        self.color(theme.popover_foreground)
    }

    fn fg_border(self, theme: &Theme) -> Self {
        self.color(theme.border)
    }
    fn fg_input(self, theme: &Theme) -> Self {
        self.color(theme.input)
    }
    fn fg_ring(self, theme: &Theme) -> Self {
        self.color(theme.ring)
    }

    fn fg_chart_1(self, theme: &Theme) -> Self {
        self.color(theme.chart_1)
    }
    fn fg_chart_2(self, theme: &Theme) -> Self {
        self.color(theme.chart_2)
    }
    fn fg_chart_3(self, theme: &Theme) -> Self {
        self.color(theme.chart_3)
    }
    fn fg_chart_4(self, theme: &Theme) -> Self {
        self.color(theme.chart_4)
    }
    fn fg_chart_5(self, theme: &Theme) -> Self {
        self.color(theme.chart_5)
    }

    // --------------------------------------------------------------
    // Literal colors
    // --------------------------------------------------------------

    fn fg_white(self, theme: &Theme) -> Self {
        self.color(theme.colors.white)
    }
    fn fg_black(self, theme: &Theme) -> Self {
        self.color(theme.colors.black)
    }
}
