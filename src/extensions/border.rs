//! # Border Extension Trait
//!
//! This module provides the [`BorderExt`] trait, which adds Tailwind‑inspired
//! border styling helpers to any component. It enables you to set border width,
//! colour, and corner radius using familiar naming conventions.
//!
//! ## Overview
//!
//! The trait offers three categories of border helpers:
//!
//! - **Border width** – `border_0()`, `border_2()`, `border_4()`, `border_8()` for
//!   common border widths.
//! - **Border colour** – `border_primary()`, `border_destructive()` to set border
//!   colours using theme tokens.
//! - **Corner radius** – Direct access to [`corner_radius`](BorderExt::corner_radius)
//!   which accepts any `impl Into<CornerRadius>`, allowing full control over
//!   individual corners.
//!
//! All colour methods fetch the current theme via [`use_cn_theme()`] and apply
//! the colour using the component’s [`border_color`](BorderExt::border_color) method.
//!
//! ## Examples
//!
//! ```no_run
//! use freyacn::extensions::BorderExt;
//! use freyacn::components::Button;
//! use freya::prelude::CornerRadius;
//!
//! // Inside a component's render method:
//! let my_button = Button()
//!     .border_2()                    // border width: 2px
//!     .border_primary()              // border colour: theme.primary
//!     .corner_radius(8.0)            // all corners: 8px
//!     .corner_radius(CornerRadius::new(4.0, 8.0, 4.0, 8.0)); // individual corners
//! ```
//!
//! ## Implementing `BorderExt` for your own components
//!
//! To use these helpers on your custom component type, you need to implement
//! the core methods: [`border_width`](BorderExt::border_width),
//! [`border_color`](BorderExt::border_color), and
//! [`corner_radius`](BorderExt::corner_radius). These should forward the styling
//! to the underlying container.
//!
//! ```no_run
//! # use freyacn::extensions::BorderExt;
//! # use freya::prelude::{Color, CornerRadius};
//! struct MyComponent;
//!
//! impl BorderExt for MyComponent {
//!     fn border_width(self, width: f32) -> Self {
//!         self.with_border_width(width)
//!     }
//!     fn border_color(self, color: Color) -> Self {
//!         self.with_border_color(color)
//!     }
//!     fn corner_radius(self, radius: impl Into<CornerRadius>) -> Self {
//!         self.with_corner_radius(radius.into())
//!     }
//! }
//! ```
//!
//! ## Notes
//!
//! - The corner radius method accepts any type that implements `Into<CornerRadius>`,
//!   such as `f32`, `[f32; 4]`, or `CornerRadius` itself.
//! - Border colour helpers use the theme from the current context.
//! - All methods return `Self` for method chaining.
//! - For more advanced corner radius control, use the [`corner_radius`] method
//!   directly with a `CornerRadius` value.

use crate::theme::use_cn_theme;
use freya::prelude::{Color, CornerRadius};

/// An extension trait that adds Tailwind‑inspired border styling helpers.
///
/// This trait is implemented for any type that can have border styling
/// (e.g., buttons, cards, containers, images). It provides a comprehensive set
/// of methods to set border width, colour, and corner radius using the
/// FreyaCN theme and Tailwind naming conventions.
///
/// # Required Methods
///
/// You must implement [`border_width`](BorderExt::border_width),
/// [`border_color`](BorderExt::border_color), and
/// [`corner_radius`](BorderExt::corner_radius) to apply border styling to your
/// component. All other methods are provided by the trait.
///
/// # Examples
///
/// Basic usage:
///
/// ```no_run
/// # use freyacn::extensions::BorderExt;
/// # use freyacn::components::Button;
/// let button = Button()
///     .border_2()          // 2px border width
///     .border_primary()    // theme primary colour
///     .corner_radius(8.0); // 8px rounded corners
/// ```
///
/// Custom corner radii:
///
/// ```no_run
/// # use freyacn::extensions::BorderExt;
/// # use freyacn::components::Button;
/// # use freya::prelude::CornerRadius;
/// let button = Button()
///     .border_4()
///     .border_destructive()
///     .corner_radius(CornerRadius::new(0.0, 10.0, 10.0, 0.0)); // rounded right side
/// ```
///
/// # Implementing on your own type
///
/// ```no_run
/// # use freyacn::extensions::BorderExt;
/// # use freya::prelude::{Color, CornerRadius};
/// struct MyWidget;
///
/// impl BorderExt for MyWidget {
///     fn border_width(self, width: f32) -> Self {
///         self.with_border_width(width)
///     }
///     fn border_color(self, color: Color) -> Self {
///         self.with_border_color(color)
///     }
///     fn corner_radius(self, radius: impl Into<CornerRadius>) -> Self {
///         self.with_corner_radius(radius.into())
///     }
/// }
/// ```
///
/// # Notes
///
/// - Border width helpers use fixed pixel values common in Tailwind (`0`, `2`, `4`, `8`).
/// - For custom border widths, use [`border_width`](BorderExt::border_width) directly.
/// - For custom colours, use [`border_color`](BorderExt::border_color) with any `Color`.
/// - The [`corner_radius`] method is flexible; use it for any radius value.
pub trait BorderExt: Sized {
    // ---- Core methods that must be implemented by the component ----

    /// Set the border width in pixels.
    ///
    /// This method must be implemented by your component. It should apply the
    /// border width to the underlying container and return `self`.
    ///
    /// # Arguments
    ///
    /// * `width` – The border width in pixels.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use freyacn::extensions::BorderExt;
    /// # struct MyComponent;
    /// impl BorderExt for MyComponent {
    ///     fn border_width(self, width: f32) -> Self {
    ///         self.with_border_width(width)
    ///     }
    ///     // ...
    /// }
    /// ```
    fn border_width(self, width: f32) -> Self;

    /// Set the border colour.
    ///
    /// This method must be implemented by your component. It should apply the
    /// border colour to the underlying container and return `self`.
    ///
    /// # Arguments
    ///
    /// * `color` – The [`Color`] to use for the border.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use freyacn::extensions::BorderExt;
    /// # use freya::prelude::Color;
    /// # struct MyComponent;
    /// impl BorderExt for MyComponent {
    ///     fn border_color(self, color: Color) -> Self {
    ///         self.with_border_color(color)
    ///     }
    ///     // ...
    /// }
    /// ```
    fn border_color(self, color: Color) -> Self;

    /// Set the corner radius.
    ///
    /// This method must be implemented by your component. It should apply the
    /// corner radius to the underlying container and return `self`.
    ///
    /// # Arguments
    ///
    /// * `radius` – The corner radius value. Can be anything that implements
    ///   `Into<CornerRadius>`, such as `f32`, `[f32; 4]`, or `CornerRadius`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use freyacn::extensions::BorderExt;
    /// # use freya::prelude::CornerRadius;
    /// # struct MyComponent;
    /// impl BorderExt for MyComponent {
    ///     fn corner_radius(self, radius: impl Into<CornerRadius>) -> Self {
    ///         self.with_corner_radius(radius.into())
    ///     }
    ///     // ...
    /// }
    /// ```
    fn corner_radius(self, radius: impl Into<CornerRadius>) -> Self;

    // ---- Border width helpers ----

    /// Set the border width to `0` pixels (no border).
    ///
    /// Equivalent to `border: none` in CSS.
    ///
    /// # Example
    ///
    /// ```
    /// # use freyacn::extensions::BorderExt;
    /// # use freyacn::components::Button;
    /// let button = Button().border_0();
    /// ```
    fn border_0(self) -> Self {
        self.border_width(0.0)
    }

    /// Set the border width to `2` pixels.
    ///
    /// Equivalent to `border-width: 2px` in CSS.
    ///
    /// # Example
    ///
    /// ```
    /// # use freyacn::extensions::BorderExt;
    /// # use freyacn::components::Button;
    /// let button = Button().border_2();
    /// ```
    fn border_2(self) -> Self {
        self.border_width(2.0)
    }

    /// Set the border width to `4` pixels.
    ///
    /// Equivalent to `border-width: 4px` in CSS.
    ///
    /// # Example
    ///
    /// ```
    /// # use freyacn::extensions::BorderExt;
    /// # use freyacn::components::Button;
    /// let button = Button().border_4();
    /// ```
    fn border_4(self) -> Self {
        self.border_width(4.0)
    }

    /// Set the border width to `8` pixels.
    ///
    /// Equivalent to `border-width: 8px` in CSS.
    ///
    /// # Example
    ///
    /// ```
    /// # use freyacn::extensions::BorderExt;
    /// # use freyacn::components::Button;
    /// let button = Button().border_8();
    /// ```
    fn border_8(self) -> Self {
        self.border_width(8.0)
    }

    // ---- Border colour helpers using theme tokens ----

    /// Set the border colour to the theme's `primary` colour.
    ///
    /// This method fetches the current theme and applies `theme.primary`
    /// as the border colour.
    ///
    /// # Example
    ///
    /// ```
    /// # use freyacn::extensions::BorderExt;
    /// # use freyacn::components::Button;
    /// let button = Button().border_primary();
    /// ```
    fn border_primary(self) -> Self {
        let theme = use_cn_theme().read();
        self.border_color(theme.primary)
    }

    /// Set the border colour to the theme's `destructive` colour.
    ///
    /// This method fetches the current theme and applies `theme.destructive`
    /// as the border colour, typically used for destructive actions.
    ///
    /// # Example
    ///
    /// ```
    /// # use freyacn::extensions::BorderExt;
    /// # use freyacn::components::Button;
    /// let button = Button().border_destructive();
    /// ```
    fn border_destructive(self) -> Self {
        let theme = use_cn_theme().read();
        self.border_color(theme.destructive)
    }
}
