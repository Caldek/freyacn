use crate::core::theme::Theme;
use freya::prelude::Color;

macro_rules! bg_color {
    ($(
        $method:ident => $field:ident
    ),* $(,)?) => {
        $(
            fn $method(self) -> Self {
                let color = self.theme().colors.$field.clone();
                self.background(color)
            }
        )*
    };
}

pub trait CNExt: Sized {
    fn theme(&self) -> &Theme;

    fn background(self, color: Color) -> Self;

    bg_color! {
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

    // ---------------------------------------------------------------------
    // Semantic theme colors
    // ---------------------------------------------------------------------

    fn bg_background(self) -> Self {
        let color = self.theme().background.clone();
        self.background(color)
    }

    fn bg_foreground(self) -> Self {
        let color = self.theme().foreground.clone();
        self.background(color)
    }

    fn bg_primary(self) -> Self {
        let color = self.theme().primary.clone();
        self.background(color)
    }

    fn bg_primary_foreground(self) -> Self {
        let color = self.theme().primary_foreground.clone();
        self.background(color)
    }

    fn bg_secondary(self) -> Self {
        let color = self.theme().secondary.clone();
        self.background(color)
    }

    fn bg_secondary_foreground(self) -> Self {
        let color = self.theme().secondary_foreground.clone();
        self.background(color)
    }

    fn bg_muted(self) -> Self {
        let color = self.theme().muted.clone();
        self.background(color)
    }

    fn bg_muted_foreground(self) -> Self {
        let color = self.theme().muted_foreground.clone();
        self.background(color)
    }

    fn bg_accent(self) -> Self {
        let color = self.theme().accent.clone();
        self.background(color)
    }

    fn bg_accent_foreground(self) -> Self {
        let color = self.theme().accent_foreground.clone();
        self.background(color)
    }

    fn bg_destructive(self) -> Self {
        let color = self.theme().destructive.clone();
        self.background(color)
    }

    fn bg_destructive_foreground(self) -> Self {
        let color = self.theme().destructive_foreground.clone();
        self.background(color)
    }

    fn bg_card(self) -> Self {
        let color = self.theme().card.clone();
        self.background(color)
    }

    fn bg_card_foreground(self) -> Self {
        let color = self.theme().card_foreground.clone();
        self.background(color)
    }

    fn bg_popover(self) -> Self {
        let color = self.theme().popover.clone();
        self.background(color)
    }

    fn bg_popover_foreground(self) -> Self {
        let color = self.theme().popover_foreground.clone();
        self.background(color)
    }

    fn bg_border(self) -> Self {
        let color = self.theme().border.clone();
        self.background(color)
    }

    fn bg_input(self) -> Self {
        let color = self.theme().input.clone();
        self.background(color)
    }

    fn bg_ring(self) -> Self {
        let color = self.theme().ring.clone();
        self.background(color)
    }

    fn bg_chart_1(self) -> Self {
        let color = self.theme().chart_1.clone();
        self.background(color)
    }

    fn bg_chart_2(self) -> Self {
        let color = self.theme().chart_2.clone();
        self.background(color)
    }

    fn bg_chart_3(self) -> Self {
        let color = self.theme().chart_3.clone();
        self.background(color)
    }

    fn bg_chart_4(self) -> Self {
        let color = self.theme().chart_4.clone();
        self.background(color)
    }

    fn bg_chart_5(self) -> Self {
        let color = self.theme().chart_5.clone();
        self.background(color)
    }

    // ---------------------------------------------------------------------
    // Literal colors
    // ---------------------------------------------------------------------

    fn bg_white(self) -> Self {
        let color = self.theme().colors.white.clone();
        self.background(color)
    }

    fn bg_black(self) -> Self {
        let color = self.theme().colors.black.clone();
        self.background(color)
    }
}
