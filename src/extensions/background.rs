//! # Background Extension Trait
//!
//! This module provides the [`BackgroundExt`] trait, which adds Tailwind‑inspired
//! background colour helpers to any component. It automatically uses the current
//! FreyaCN theme, so you can write expressive, theme‑aware UI code.
//!
//! ## Overview
//!
//! The trait offers three categories of background helpers:
//!
//! - **Palette colours** – `bg_slate_50()`, `bg_red_500()`, `bg_blue_200()`, etc.
//!   All colours from the Tailwind palette are available.
//! - **Semantic colours** – `bg_primary()`, `bg_destructive()`, `bg_muted()`, etc.
//!   These map to the theme’s semantic tokens.
//! - **Literal colours** – `bg_white()`, `bg_black()` for quick overrides.
//!
//! All methods fetch the current theme via [`use_cn_theme()`] and apply the colour
//! to the component’s background using the [`background`](BackgroundExt::background) method.
//!
//! ## Examples
//!
//! ```no_run
//! use freyacn::extensions::BackgroundExt;
//! use freyacn::components::Button;
//!
//! // Inside a component's render method:
//! let my_button = Button()
//!     .bg_primary()       // sets background to theme.primary
//!     .bg_white()         // override with white
//!     .bg_blue_500();     // use a specific palette colour
//! ```
//!
//! ## Implementing `BackgroundExt` for your own components
//!
//! To use these helpers on your custom component type, you only need to implement
//! the [`background`](BackgroundExt::background) method. The macro will generate
//! all other methods automatically.
//!
//! ```no_run
//! # use freyacn::extensions::BackgroundExt;
//! # use freya::prelude::Color;
//! struct MyComponent;
//!
//! impl BackgroundExt for MyComponent {
//!     fn background(self, color: Color) -> Self {
//!         // Apply the colour to your component's internal state
//!         self.background = color;
//!         self
//!     }
//! }
//! ```
//!
//! ## Macro Details
//!
//! The [`bg_color!`] macro generates all palette methods. It takes a list of
//! method names and corresponding field names from the [`Colors`](crate::core::theme::Colors) struct.
//! This keeps the code DRY and maintainable.
//!
//! ## Notes
//!
//! - The trait is designed to be used inside a Freya component’s `render` method,
//!   where the theme context is available via [`use_cn_theme()`].
//! - The background colour is applied as a **fill**; for text or icon colour,
//!   see [`ForegroundExt`](super::foreground::ForegroundExt).

use crate::theme::use_cn_theme;
use freya::prelude::Color;

/// Generates background colour methods for every palette colour.
///
/// This macro is used internally by [`BackgroundExt`] to produce methods like
/// `bg_slate_50()`, `bg_red_500()`, etc. Each method fetches the current theme
/// via [`use_cn_theme()`] and calls [`background`](BackgroundExt::background)
/// with the corresponding colour from `theme.colors.$field`.
///
/// # Arguments
///
/// * `$($method:ident => $field:ident),*` – A list of method names and the
///   corresponding field name in the [`Colors`](crate::core::theme::Colors) struct.
///
/// # Example expansion
///
/// ```ignore
/// bg_color! {
///     bg_slate_50 => slate_50,
///     bg_red_500 => red_500,
/// }
/// ```
///
/// Expands to:
///
/// ```ignore
/// fn bg_slate_50(self) -> Self {
///     let theme = use_cn_theme().read();
///     self.background(theme.colors.slate_50)
/// }
/// fn bg_red_500(self) -> Self {
///     let theme = use_cn_theme().read();
///     self.background(theme.colors.red_500)
/// }
/// ```
///
/// # Note
///
/// The macro is defined outside the trait so it can be reused; it is then
/// invoked inside the trait definition to generate all palette methods.
macro_rules! bg_color {
    ($($method:ident => $field:ident),* $(,)?) => {
        $(
            fn $method(self) -> Self {
                let theme = use_cn_theme().read();
                self.background(theme.colors.$field)
            }
        )*
    };
}

/// An extension trait that adds theme‑aware background colour helpers.
///
/// This trait is implemented for any type that can have a background colour
/// (e.g., buttons, cards, containers, icons). It provides a comprehensive set
/// of methods to set the background using the current FreyaCN theme.
///
/// # Required Method
///
/// You must implement the [`background`](BackgroundExt::background) method to
/// apply the colour to your component. All other methods are provided by the
/// macro and semantic helpers.
///
/// # Examples
///
/// Basic usage:
///
/// ```no_run
/// # use freyacn::extensions::BackgroundExt;
/// # use freyacn::components::Button;
/// let button = Button()
///     .bg_primary()          // semantic colour
///     .bg_red_500()          // palette colour
///     .bg_white()            // literal colour
///     .background(Color::from_rgb(255, 0, 0)); // explicit override
/// ```
///
/// Chaining:
///
/// ```no_run
/// # use freyacn::extensions::BackgroundExt;
/// # use freyacn::components
/// use freyacn::components::Button;
/// let button = Button::new()
///     .bg_slate_100()
///     .bg_accent()
///     .bg_chart_1();
/// ```
///
/// The final call wins – background colour is not cumulative, but the last
/// set colour is applied.
///
/// # Implementing on your own type
///
/// ```no_run
/// # use freyacn::extensions::BackgroundExt;
/// # use freya::prelude::Color;
/// struct MyWidget;
///
/// impl BackgroundExt for MyWidget {
///     fn background(self, color: Color) -> Self {
///         // Store the colour in your widget's builder state
///          self.background = color;
///         self
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
pub trait BackgroundExt: Sized {
    /// Set the background colour explicitly.
    ///
    /// This is the core method that must be implemented by your component.
    /// It should store the colour in the component’s internal builder state
    /// and return `self` for method chaining.
    ///
    /// # Arguments
    ///
    /// * `color` – The [`Color`] to apply as the background.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use freyacn::core::ext::BackgroundExt;
    /// # use freya::prelude::Color;
    /// # struct MyWidget;
    /// impl BackgroundExt for MyWidget {
    ///     fn background(self, color: Color) -> Self {
    ///         // store the colour
    ///         self
    ///     }
    /// }
    /// ```
    fn background(self, color: Color) -> Self;

    // --------------------------------------------------------------
    // Palette colours
    // --------------------------------------------------------------
    // The following methods are generated by the bg_color! macro.
    // They cover all Tailwind‑style colour swatches from the theme's
    // `Colors` struct, including neutrals (slate, gray, zinc, neutral,
    // stone, mauve, olive, mist, taupe) and all accent colours
    // (red, orange, amber, yellow, lime, green, emerald, teal, cyan,
    // sky, blue, indigo, violet, purple, fuchsia, pink, rose).

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

        // Accents
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
    // Semantic colours
    // --------------------------------------------------------------
    // The following methods provide theme‑aware semantic colours.
    // They are not generated by the macro because they refer to
    // fields on the `Theme` struct, not `Colors`.

    /// Sets the background to the theme's `background` colour.
    ///
    /// This is the main background colour of the application, typically a light
    /// colour in light mode and a dark colour in dark mode.
    fn bg_background(self) -> Self {
        let theme = use_cn_theme().read();
        self.background(theme.background)
    }

    /// Sets the background to the theme's `foreground` colour.
    ///
    /// This is the main text/icon colour; using it as a background can create
    /// high‑contrast elements.
    fn bg_foreground(self) -> Self {
        let theme = use_cn_theme().read();
        self.background(theme.foreground)
    }

    /// Sets the background to the theme's `primary` colour.
    ///
    /// Used for primary actions and call‑to‑action elements.
    fn bg_primary(self) -> Self {
        let theme = use_cn_theme().read();
        self.background(theme.primary)
    }

    /// Sets the background to the theme's `primary_foreground` colour.
    ///
    /// This is the colour that should be used for text/icon on top of
    /// the `primary` background.
    fn bg_primary_foreground(self) -> Self {
        let theme = use_cn_theme().read();
        self.background(theme.primary_foreground)
    }

    /// Sets the background to the theme's `secondary` colour.
    ///
    /// Used for secondary buttons and less prominent elements.
    fn bg_secondary(self) -> Self {
        let theme = use_cn_theme().read();
        self.background(theme.secondary)
    }

    /// Sets the background to the theme's `secondary_foreground` colour.
    ///
    /// The foreground colour for secondary backgrounds.
    fn bg_secondary_foreground(self) -> Self {
        let theme = use_cn_theme().read();
        self.background(theme.secondary_foreground)
    }

    /// Sets the background to the theme's `muted` colour.
    ///
    /// Used for subtle, low‑emphasis backgrounds.
    fn bg_muted(self) -> Self {
        let theme = use_cn_theme().read();
        self.background(theme.muted)
    }

    /// Sets the background to the theme's `muted_foreground` colour.
    ///
    /// Foreground colour for muted backgrounds.
    fn bg_muted_foreground(self) -> Self {
        let theme = use_cn_theme().read();
        self.background(theme.muted_foreground)
    }

    /// Sets the background to the theme's `accent` colour.
    ///
    /// Used for accent elements, often the same as `primary` or a different
    /// colour depending on the theme variant.
    fn bg_accent(self) -> Self {
        let theme = use_cn_theme().read();
        self.background(theme.accent)
    }

    /// Sets the background to the theme's `accent_foreground` colour.
    ///
    /// Foreground colour for accent backgrounds.
    fn bg_accent_foreground(self) -> Self {
        let theme = use_cn_theme().read();
        self.background(theme.accent_foreground)
    }

    /// Sets the background to the theme's `destructive` colour.
    ///
    /// Used for destructive actions like delete buttons; typically red.
    fn bg_destructive(self) -> Self {
        let theme = use_cn_theme().read();
        self.background(theme.destructive)
    }

    /// Sets the background to the theme's `destructive_foreground` colour.
    ///
    /// Foreground colour for destructive backgrounds.
    fn bg_destructive_foreground(self) -> Self {
        let theme = use_cn_theme().read();
        self.background(theme.destructive_foreground)
    }

    /// Sets the background to the theme's `card` colour.
    ///
    /// Used for card components.
    fn bg_card(self) -> Self {
        let theme = use_cn_theme().read();
        self.background(theme.card)
    }

    /// Sets the background to the theme's `card_foreground` colour.
    ///
    /// Foreground colour for card backgrounds.
    fn bg_card_foreground(self) -> Self {
        let theme = use_cn_theme().read();
        self.background(theme.card_foreground)
    }

    /// Sets the background to the theme's `popover` colour.
    ///
    /// Used for popover, dropdown, and tooltip backgrounds.
    fn bg_popover(self) -> Self {
        let theme = use_cn_theme().read();
        self.background(theme.popover)
    }

    /// Sets the background to the theme's `popover_foreground` colour.
    ///
    /// Foreground colour for popover backgrounds.
    fn bg_popover_foreground(self) -> Self {
        let theme = use_cn_theme().read();
        self.background(theme.popover_foreground)
    }

    /// Sets the background to the theme's `border` colour.
    ///
    /// Used for borders and dividers.
    fn bg_border(self) -> Self {
        let theme = use_cn_theme().read();
        self.background(theme.border)
    }

    /// Sets the background to the theme's `input` colour.
    ///
    /// Used for input field backgrounds.
    fn bg_input(self) -> Self {
        let theme = use_cn_theme().read();
        self.background(theme.input)
    }

    /// Sets the background to the theme's `ring` colour.
    ///
    /// Used for focus rings and outlines.
    fn bg_ring(self) -> Self {
        let theme = use_cn_theme().read();
        self.background(theme.ring)
    }

    /// Sets the background to the theme's first chart colour.
    fn bg_chart_1(self) -> Self {
        let theme = use_cn_theme().read();
        self.background(theme.chart_1)
    }

    /// Sets the background to the theme's second chart colour.
    fn bg_chart_2(self) -> Self {
        let theme = use_cn_theme().read();
        self.background(theme.chart_2)
    }

    /// Sets the background to the theme's third chart colour.
    fn bg_chart_3(self) -> Self {
        let theme = use_cn_theme().read();
        self.background(theme.chart_3)
    }

    /// Sets the background to the theme's fourth chart colour.
    fn bg_chart_4(self) -> Self {
        let theme = use_cn_theme().read();
        self.background(theme.chart_4)
    }

    /// Sets the background to the theme's fifth chart colour.
    fn bg_chart_5(self) -> Self {
        let theme = use_cn_theme().read();
        self.background(theme.chart_5)
    }

    // --------------------------------------------------------------
    // Literal colours
    // --------------------------------------------------------------
    // These methods provide quick access to pure white and black.

    /// Sets the background to pure white (`#ffffff`).
    ///
    /// This is a literal colour, not theme‑aware.
    fn bg_white(self) -> Self {
        let theme = use_cn_theme().read();
        self.background(theme.colors.white)
    }

    /// Sets the background to pure black (`#000000`).
    ///
    /// This is a literal colour, not theme‑aware.
    fn bg_black(self) -> Self {
        let theme = use_cn_theme().read();
        self.background(theme.colors.black)
    }
}
