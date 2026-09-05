//! # Foreground Extension Trait
//!
//! This module provides the [`ForegroundExt`] trait, which adds Tailwind‑inspired
//! text and icon colour helpers to any component. It automatically uses the current
//! FreyaCN theme, enabling you to write expressive, theme‑aware UI code.
//!
//! ## Overview
//!
//! The trait offers three categories of foreground (text/icon colour) helpers:
//!
//! - **Palette colours** – `text_slate_50()`, `text_red_500()`, `text_blue_200()`, etc.
//!   All colours from the Tailwind palette are available.
//! - **Semantic colours** – `text_primary()`, `text_destructive()`, `text_muted()`, etc.
//!   These map to the theme’s semantic tokens.
//! - **Literal colours** – `text_white()`, `text_black()` for quick overrides.
//!
//! All methods fetch the current theme via [`use_cn_theme()`] and apply the colour
//! to the component’s foreground using the [`color`](ForegroundExt::color) method.
//!
//! ## Examples
//!
//! ```no_run
//! use freyacn::extensions::ForegroundExt;
//! use freyacn::components::Button;
//!
//! // Inside a component's render method:
//! let my_button = Button()
//!     .text_primary()       // sets text colour to theme.primary
//!     .text_white()         // override with white
//!     .text_blue_500();     // use a specific palette colour
//! ```
//!
//! ## Implementing `ForegroundExt` for your own components
//!
//! To use these helpers on your custom component type, you only need to implement
//! the [`color`](ForegroundExt::color) method. The macro will generate all other
//! methods automatically.
//!
//! ```no_run
//! # use freyacn::extensions::ForegroundExt;
//! # use freya::prelude::Color;
//! struct MyComponent;
//!
//! impl ForegroundExt for MyComponent {
//!     fn color(self, color: Color) -> Self {
//!         // Apply the colour to your component's internal state
//!         self.text_color = Some(color);
//!         self
//!     }
//! }
//! ```
//!
//! ## Macro Details
//!
//! The [`text_color!`] macro generates all palette methods. It takes a list of
//! method names and corresponding field names from the [`Colors`](crate::core::theme::Colors) struct.
//! This keeps the code DRY and maintainable.
//!
//! ## Notes
//!
//! - The trait is designed to be used inside a Freya component’s `render` method,
//!   where the theme context is available via [`use_cn_theme()`].
//! - The foreground colour is applied as **text or icon colour**; for background
//!   colour, see [`BackgroundExt`](super::background::BackgroundExt).

use crate::theme::use_cn_theme;
use freya::prelude::Color;

/// Generates foreground colour methods for every palette colour.
///
/// This macro is used internally by [`ForegroundExt`] to produce methods like
/// `text_slate_50()`, `text_red_500()`, etc. Each method fetches the current theme
/// via [`use_cn_theme()`] and calls [`color`](ForegroundExt::color) with the
/// corresponding colour from `theme.colors.$field`.
///
/// # Arguments
///
/// * `$($method:ident => $field:ident),*` – A list of method names and the
///   corresponding field name in the [`Colors`](crate::core::theme::Colors) struct.
///
/// # Example expansion
///
/// ```ignore
/// text_color! {
///     text_slate_50 => slate_50,
///     text_red_500 => red_500,
/// }
/// ```
///
/// Expands to:
///
/// ```ignore
/// fn text_slate_50(self) -> Self {
///     let theme = use_cn_theme().read();
///     self.color(theme.colors.slate_50)
/// }
/// fn text_red_500(self) -> Self {
///     let theme = use_cn_theme().read();
///     self.color(theme.colors.red_500)
/// }
/// ```
///
/// # Note
///
/// The macro is defined outside the trait so it can be reused; it is then
/// invoked inside the trait definition to generate all palette methods.
macro_rules! text_color {
    ($($method:ident => $field:ident),* $(,)?) => {
        $(
            fn $method(self) -> Self {
                let theme = use_cn_theme().read();
                self.color(theme.colors.$field)
            }
        )*
    };
}

/// An extension trait that adds theme‑aware foreground colour helpers.
///
/// This trait is implemented for any type that can have a foreground colour
/// (e.g., text labels, icons, buttons, cards). It provides a comprehensive set
/// of methods to set the foreground using the current FreyaCN theme.
///
/// # Required Method
///
/// You must implement the [`color`](ForegroundExt::color) method to apply the
/// colour to your component. All other methods are provided by the macro and
/// semantic helpers.
///
/// # Examples
///
/// Basic usage:
///
/// ```no_run
/// # use freyacn::extensions::ForegroundExt;
/// # use freyacn::components::Button;
/// let button = Button()
///     .text_primary()          // semantic colour
///     .text_red_500()          // palette colour
///     .text_white()            // literal colour
///     .color(Color::from_rgb(255, 0, 0)); // explicit override
/// ```
///
/// Chaining:
///
/// ```no_run
/// # use freyacn::extensions::ForegroundExt;
/// # use freyacn::components::Button;
/// let button = Button::new()
///     .text_slate_100()
///     .text_accent()
///     .text_chart_1();
/// ```
///
/// The final call wins – foreground colour is not cumulative, but the last
/// set colour is applied.
///
/// # Implementing on your own type
///
/// ```no_run
/// # use freyacn::extensions::ForegroundExt;
/// # use freya::prelude::Color;
/// struct MyWidget;
///
/// impl ForegroundExt for MyWidget {
///     fn color(self, color: Color) -> Self {
///         // Store the colour in your widget's builder state
///         self.with_color(color)
///     }
/// }
/// ```
///
/// # Notes
///
/// - All methods automatically use the theme from the context; they are
///   intended to be called inside a component’s `render` method.
/// - The macro generates **all** palette colours, so you get a consistent
///   set of helpers without repeating code.
pub trait ForegroundExt: Sized {
    /// Set the foreground (text/icon) colour explicitly.
    ///
    /// This is the core method that must be implemented by your component.
    /// It should store the colour in the component’s internal builder state
    /// and return `self` for method chaining.
    ///
    /// # Arguments
    ///
    /// * `color` – The [`Color`] to apply as the foreground.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use freyacn::extensions::ForegroundExt;
    /// # use freya::prelude::Color;
    /// # struct MyWidget;
    /// impl ForegroundExt for MyWidget {
    ///     fn color(self, color: Color) -> Self {
    ///         // store the colour
    ///         self
    ///     }
    /// }
    /// ```
    fn color(self, color: Color) -> Self;

    // --------------------------------------------------------------
    // Palette colours
    // --------------------------------------------------------------
    // The following methods are generated by the text_color! macro.
    // They cover all Tailwind‑style colour swatches from the theme's
    // `Colors` struct, including neutrals (slate, gray, zinc, neutral,
    // stone, mauve, olive, mist, taupe) and all accent colours
    // (red, orange, amber, yellow, lime, green, emerald, teal, cyan,
    // sky, blue, indigo, violet, purple, fuchsia, pink, rose).

    text_color! {
        // Neutrals
        text_slate_50 => slate_50,
        text_slate_100 => slate_100,
        text_slate_200 => slate_200,
        text_slate_300 => slate_300,
        text_slate_400 => slate_400,
        text_slate_500 => slate_500,
        text_slate_600 => slate_600,
        text_slate_700 => slate_700,
        text_slate_800 => slate_800,
        text_slate_900 => slate_900,
        text_slate_950 => slate_950,

        text_gray_50 => gray_50,
        text_gray_100 => gray_100,
        text_gray_200 => gray_200,
        text_gray_300 => gray_300,
        text_gray_400 => gray_400,
        text_gray_500 => gray_500,
        text_gray_600 => gray_600,
        text_gray_700 => gray_700,
        text_gray_800 => gray_800,
        text_gray_900 => gray_900,
        text_gray_950 => gray_950,

        text_zinc_50 => zinc_50,
        text_zinc_100 => zinc_100,
        text_zinc_200 => zinc_200,
        text_zinc_300 => zinc_300,
        text_zinc_400 => zinc_400,
        text_zinc_500 => zinc_500,
        text_zinc_600 => zinc_600,
        text_zinc_700 => zinc_700,
        text_zinc_800 => zinc_800,
        text_zinc_900 => zinc_900,
        text_zinc_950 => zinc_950,

        text_neutral_50 => neutral_50,
        text_neutral_100 => neutral_100,
        text_neutral_200 => neutral_200,
        text_neutral_300 => neutral_300,
        text_neutral_400 => neutral_400,
        text_neutral_500 => neutral_500,
        text_neutral_600 => neutral_600,
        text_neutral_700 => neutral_700,
        text_neutral_800 => neutral_800,
        text_neutral_900 => neutral_900,
        text_neutral_950 => neutral_950,

        text_stone_50 => stone_50,
        text_stone_100 => stone_100,
        text_stone_200 => stone_200,
        text_stone_300 => stone_300,
        text_stone_400 => stone_400,
        text_stone_500 => stone_500,
        text_stone_600 => stone_600,
        text_stone_700 => stone_700,
        text_stone_800 => stone_800,
        text_stone_900 => stone_900,
        text_stone_950 => stone_950,

        text_mauve_50 => mauve_50,
        text_mauve_100 => mauve_100,
        text_mauve_200 => mauve_200,
        text_mauve_300 => mauve_300,
        text_mauve_400 => mauve_400,
        text_mauve_500 => mauve_500,
        text_mauve_600 => mauve_600,
        text_mauve_700 => mauve_700,
        text_mauve_800 => mauve_800,
        text_mauve_900 => mauve_900,
        text_mauve_950 => mauve_950,

        text_olive_50 => olive_50,
        text_olive_100 => olive_100,
        text_olive_200 => olive_200,
        text_olive_300 => olive_300,
        text_olive_400 => olive_400,
        text_olive_500 => olive_500,
        text_olive_600 => olive_600,
        text_olive_700 => olive_700,
        text_olive_800 => olive_800,
        text_olive_900 => olive_900,
        text_olive_950 => olive_950,

        text_mist_50 => mist_50,
        text_mist_100 => mist_100,
        text_mist_200 => mist_200,
        text_mist_300 => mist_300,
        text_mist_400 => mist_400,
        text_mist_500 => mist_500,
        text_mist_600 => mist_600,
        text_mist_700 => mist_700,
        text_mist_800 => mist_800,
        text_mist_900 => mist_900,
        text_mist_950 => mist_950,

        text_taupe_50 => taupe_50,
        text_taupe_100 => taupe_100,
        text_taupe_200 => taupe_200,
        text_taupe_300 => taupe_300,
        text_taupe_400 => taupe_400,
        text_taupe_500 => taupe_500,
        text_taupe_600 => taupe_600,
        text_taupe_700 => taupe_700,
        text_taupe_800 => taupe_800,
        text_taupe_900 => taupe_900,
        text_taupe_950 => taupe_950,

        // Accents
        text_red_50 => red_50,
        text_red_100 => red_100,
        text_red_200 => red_200,
        text_red_300 => red_300,
        text_red_400 => red_400,
        text_red_500 => red_500,
        text_red_600 => red_600,
        text_red_700 => red_700,
        text_red_800 => red_800,
        text_red_900 => red_900,
        text_red_950 => red_950,

        text_orange_50 => orange_50,
        text_orange_100 => orange_100,
        text_orange_200 => orange_200,
        text_orange_300 => orange_300,
        text_orange_400 => orange_400,
        text_orange_500 => orange_500,
        text_orange_600 => orange_600,
        text_orange_700 => orange_700,
        text_orange_800 => orange_800,
        text_orange_900 => orange_900,
        text_orange_950 => orange_950,

        text_amber_50 => amber_50,
        text_amber_100 => amber_100,
        text_amber_200 => amber_200,
        text_amber_300 => amber_300,
        text_amber_400 => amber_400,
        text_amber_500 => amber_500,
        text_amber_600 => amber_600,
        text_amber_700 => amber_700,
        text_amber_800 => amber_800,
        text_amber_900 => amber_900,
        text_amber_950 => amber_950,

        text_yellow_50 => yellow_50,
        text_yellow_100 => yellow_100,
        text_yellow_200 => yellow_200,
        text_yellow_300 => yellow_300,
        text_yellow_400 => yellow_400,
        text_yellow_500 => yellow_500,
        text_yellow_600 => yellow_600,
        text_yellow_700 => yellow_700,
        text_yellow_800 => yellow_800,
        text_yellow_900 => yellow_900,
        text_yellow_950 => yellow_950,

        text_lime_50 => lime_50,
        text_lime_100 => lime_100,
        text_lime_200 => lime_200,
        text_lime_300 => lime_300,
        text_lime_400 => lime_400,
        text_lime_500 => lime_500,
        text_lime_600 => lime_600,
        text_lime_700 => lime_700,
        text_lime_800 => lime_800,
        text_lime_900 => lime_900,
        text_lime_950 => lime_950,

        text_green_50 => green_50,
        text_green_100 => green_100,
        text_green_200 => green_200,
        text_green_300 => green_300,
        text_green_400 => green_400,
        text_green_500 => green_500,
        text_green_600 => green_600,
        text_green_700 => green_700,
        text_green_800 => green_800,
        text_green_900 => green_900,
        text_green_950 => green_950,

        text_emerald_50 => emerald_50,
        text_emerald_100 => emerald_100,
        text_emerald_200 => emerald_200,
        text_emerald_300 => emerald_300,
        text_emerald_400 => emerald_400,
        text_emerald_500 => emerald_500,
        text_emerald_600 => emerald_600,
        text_emerald_700 => emerald_700,
        text_emerald_800 => emerald_800,
        text_emerald_900 => emerald_900,
        text_emerald_950 => emerald_950,

        text_teal_50 => teal_50,
        text_teal_100 => teal_100,
        text_teal_200 => teal_200,
        text_teal_300 => teal_300,
        text_teal_400 => teal_400,
        text_teal_500 => teal_500,
        text_teal_600 => teal_600,
        text_teal_700 => teal_700,
        text_teal_800 => teal_800,
        text_teal_900 => teal_900,
        text_teal_950 => teal_950,

        text_cyan_50 => cyan_50,
        text_cyan_100 => cyan_100,
        text_cyan_200 => cyan_200,
        text_cyan_300 => cyan_300,
        text_cyan_400 => cyan_400,
        text_cyan_500 => cyan_500,
        text_cyan_600 => cyan_600,
        text_cyan_700 => cyan_700,
        text_cyan_800 => cyan_800,
        text_cyan_900 => cyan_900,
        text_cyan_950 => cyan_950,

        text_sky_50 => sky_50,
        text_sky_100 => sky_100,
        text_sky_200 => sky_200,
        text_sky_300 => sky_300,
        text_sky_400 => sky_400,
        text_sky_500 => sky_500,
        text_sky_600 => sky_600,
        text_sky_700 => sky_700,
        text_sky_800 => sky_800,
        text_sky_900 => sky_900,
        text_sky_950 => sky_950,

        text_blue_50 => blue_50,
        text_blue_100 => blue_100,
        text_blue_200 => blue_200,
        text_blue_300 => blue_300,
        text_blue_400 => blue_400,
        text_blue_500 => blue_500,
        text_blue_600 => blue_600,
        text_blue_700 => blue_700,
        text_blue_800 => blue_800,
        text_blue_900 => blue_900,
        text_blue_950 => blue_950,

        text_indigo_50 => indigo_50,
        text_indigo_100 => indigo_100,
        text_indigo_200 => indigo_200,
        text_indigo_300 => indigo_300,
        text_indigo_400 => indigo_400,
        text_indigo_500 => indigo_500,
        text_indigo_600 => indigo_600,
        text_indigo_700 => indigo_700,
        text_indigo_800 => indigo_800,
        text_indigo_900 => indigo_900,
        text_indigo_950 => indigo_950,

        text_violet_50 => violet_50,
        text_violet_100 => violet_100,
        text_violet_200 => violet_200,
        text_violet_300 => violet_300,
        text_violet_400 => violet_400,
        text_violet_500 => violet_500,
        text_violet_600 => violet_600,
        text_violet_700 => violet_700,
        text_violet_800 => violet_800,
        text_violet_900 => violet_900,
        text_violet_950 => violet_950,

        text_purple_50 => purple_50,
        text_purple_100 => purple_100,
        text_purple_200 => purple_200,
        text_purple_300 => purple_300,
        text_purple_400 => purple_400,
        text_purple_500 => purple_500,
        text_purple_600 => purple_600,
        text_purple_700 => purple_700,
        text_purple_800 => purple_800,
        text_purple_900 => purple_900,
        text_purple_950 => purple_950,

        text_fuchsia_50 => fuchsia_50,
        text_fuchsia_100 => fuchsia_100,
        text_fuchsia_200 => fuchsia_200,
        text_fuchsia_300 => fuchsia_300,
        text_fuchsia_400 => fuchsia_400,
        text_fuchsia_500 => fuchsia_500,
        text_fuchsia_600 => fuchsia_600,
        text_fuchsia_700 => fuchsia_700,
        text_fuchsia_800 => fuchsia_800,
        text_fuchsia_900 => fuchsia_900,
        text_fuchsia_950 => fuchsia_950,

        text_pink_50 => pink_50,
        text_pink_100 => pink_100,
        text_pink_200 => pink_200,
        text_pink_300 => pink_300,
        text_pink_400 => pink_400,
        text_pink_500 => pink_500,
        text_pink_600 => pink_600,
        text_pink_700 => pink_700,
        text_pink_800 => pink_800,
        text_pink_900 => pink_900,
        text_pink_950 => pink_950,

        text_rose_50 => rose_50,
        text_rose_100 => rose_100,
        text_rose_200 => rose_200,
        text_rose_300 => rose_300,
        text_rose_400 => rose_400,
        text_rose_500 => rose_500,
        text_rose_600 => rose_600,
        text_rose_700 => rose_700,
        text_rose_800 => rose_800,
        text_rose_900 => rose_900,
        text_rose_950 => rose_950,
    }

    // --------------------------------------------------------------
    // Semantic colours
    // --------------------------------------------------------------
    // The following methods provide theme‑aware semantic colours.
    // They are not generated by the macro because they refer to
    // fields on the `Theme` struct, not `Colors`.

    /// Sets the foreground to the theme's `background` colour.
    ///
    /// This is the main background colour of the application, typically a light
    /// colour in light mode and a dark colour in dark mode.
    fn text_background(self) -> Self {
        let theme = use_cn_theme().read();
        self.color(theme.background)
    }

    /// Sets the foreground to the theme's `foreground` colour.
    ///
    /// This is the default text/icon colour, typically dark in light mode and
    /// light in dark mode.
    fn text_foreground(self) -> Self {
        let theme = use_cn_theme().read();
        self.color(theme.foreground)
    }

    /// Sets the foreground to the theme's `primary` colour.
    ///
    /// Used for primary actions and call‑to‑action elements.
    fn text_primary(self) -> Self {
        let theme = use_cn_theme().read();
        self.color(theme.primary)
    }

    /// Sets the foreground to the theme's `primary_foreground` colour.
    ///
    /// This is the colour that should be used for text/icon on top of
    /// the `primary` background.
    fn text_primary_foreground(self) -> Self {
        let theme = use_cn_theme().read();
        self.color(theme.primary_foreground)
    }

    /// Sets the foreground to the theme's `secondary` colour.
    ///
    /// Used for secondary buttons and less prominent elements.
    fn text_secondary(self) -> Self {
        let theme = use_cn_theme().read();
        self.color(theme.secondary)
    }

    /// Sets the foreground to the theme's `secondary_foreground` colour.
    ///
    /// The foreground colour for secondary backgrounds.
    fn text_secondary_foreground(self) -> Self {
        let theme = use_cn_theme().read();
        self.color(theme.secondary_foreground)
    }

    /// Sets the foreground to the theme's `muted` colour.
    ///
    /// Used for subtle, low‑emphasis text.
    fn text_muted(self) -> Self {
        let theme = use_cn_theme().read();
        self.color(theme.muted)
    }

    /// Sets the foreground to the theme's `muted_foreground` colour.
    ///
    /// Foreground colour for muted backgrounds.
    fn text_muted_foreground(self) -> Self {
        let theme = use_cn_theme().read();
        self.color(theme.muted_foreground)
    }

    /// Sets the foreground to the theme's `accent` colour.
    ///
    /// Used for accent elements, often the same as `primary` or a different
    /// colour depending on the theme variant.
    fn text_accent(self) -> Self {
        let theme = use_cn_theme().read();
        self.color(theme.accent)
    }

    /// Sets the foreground to the theme's `accent_foreground` colour.
    ///
    /// Foreground colour for accent backgrounds.
    fn text_accent_foreground(self) -> Self {
        let theme = use_cn_theme().read();
        self.color(theme.accent_foreground)
    }

    /// Sets the foreground to the theme's `destructive` colour.
    ///
    /// Used for destructive actions like delete buttons; typically red.
    fn text_destructive(self) -> Self {
        let theme = use_cn_theme().read();
        self.color(theme.destructive)
    }

    /// Sets the foreground to the theme's `destructive_foreground` colour.
    ///
    /// Foreground colour for destructive backgrounds.
    fn text_destructive_foreground(self) -> Self {
        let theme = use_cn_theme().read();
        self.color(theme.destructive_foreground)
    }

    /// Sets the foreground to the theme's `card` colour.
    ///
    /// Used for card components.
    fn text_card(self) -> Self {
        let theme = use_cn_theme().read();
        self.color(theme.card)
    }

    /// Sets the foreground to the theme's `card_foreground` colour.
    ///
    /// Foreground colour for card backgrounds.
    fn text_card_foreground(self) -> Self {
        let theme = use_cn_theme().read();
        self.color(theme.card_foreground)
    }

    /// Sets the foreground to the theme's `popover` colour.
    ///
    /// Used for popover, dropdown, and tooltip backgrounds.
    fn text_popover(self) -> Self {
        let theme = use_cn_theme().read();
        self.color(theme.popover)
    }

    /// Sets the foreground to the theme's `popover_foreground` colour.
    ///
    /// Foreground colour for popover backgrounds.
    fn text_popover_foreground(self) -> Self {
        let theme = use_cn_theme().read();
        self.color(theme.popover_foreground)
    }

    /// Sets the foreground to the theme's `border` colour.
    ///
    /// Used for borders and dividers.
    fn text_border(self) -> Self {
        let theme = use_cn_theme().read();
        self.color(theme.border)
    }

    /// Sets the foreground to the theme's `input` colour.
    ///
    /// Used for input field backgrounds.
    fn text_input(self) -> Self {
        let theme = use_cn_theme().read();
        self.color(theme.input)
    }

    /// Sets the foreground to the theme's `ring` colour.
    ///
    /// Used for focus rings and outlines.
    fn text_ring(self) -> Self {
        let theme = use_cn_theme().read();
        self.color(theme.ring)
    }

    /// Sets the foreground to the theme's first chart colour.
    fn text_chart_1(self) -> Self {
        let theme = use_cn_theme().read();
        self.color(theme.chart_1)
    }

    /// Sets the foreground to the theme's second chart colour.
    fn text_chart_2(self) -> Self {
        let theme = use_cn_theme().read();
        self.color(theme.chart_2)
    }

    /// Sets the foreground to the theme's third chart colour.
    fn text_chart_3(self) -> Self {
        let theme = use_cn_theme().read();
        self.color(theme.chart_3)
    }

    /// Sets the foreground to the theme's fourth chart colour.
    fn text_chart_4(self) -> Self {
        let theme = use_cn_theme().read();
        self.color(theme.chart_4)
    }

    /// Sets the foreground to the theme's fifth chart colour.
    fn text_chart_5(self) -> Self {
        let theme = use_cn_theme().read();
        self.color(theme.chart_5)
    }

    // --------------------------------------------------------------
    // Literal colours
    // --------------------------------------------------------------
    // These methods provide quick access to pure white and black.

    /// Sets the foreground to pure white (`#ffffff`).
    ///
    /// This is a literal colour, not theme‑aware.
    fn text_white(self) -> Self {
        let theme = use_cn_theme().read();
        self.color(theme.colors.white)
    }

    /// Sets the foreground to pure black (`#000000`).
    ///
    /// This is a literal colour, not theme‑aware.
    fn text_black(self) -> Self {
        let theme = use_cn_theme().read();
        self.color(theme.colors.black)
    }
}
