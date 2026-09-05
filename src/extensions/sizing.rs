//! # Sizing Extension Trait
//!
//! This module provides the [`SizingExt`] trait, which adds Tailwind‑inspired
//! width and height helpers to any component. It enables you to use familiar
//! naming like `w_4()`, `h_8()`, and `w_full()` to control element sizing
//! directly in your UI code.
//!
//! ## Overview
//!
//! The trait offers three categories of sizing helpers:
//!
//! - **Generic pixel sizing** – `w()`, `h()`, `min_w()`, `max_w()`, `min_h()`,
//!   `max_h()` for raw pixel values.
//! - **Percentage sizing** – `w_percent()`, `h_percent()` for percentage‑based sizes.
//! - **Tailwind specials** – `w_full()`, `w_screen()`, `w_auto()`, `h_full()`,
//!   `h_screen()`, `h_auto()` for common layout needs.
//! - **Scale methods** – `w_0()`, `w_1()`, `w_2()`, …, up to `w_96()`, and
//!   equivalents for `h_*`, `min_w_*`, etc. These follow the Tailwind spacing scale
//!   where 1 unit = 4px.
//!
//! All methods delegate to the component’s underlying sizing methods, which you
//! implement on your component.
//!
//! ## Examples
//!
//! ```no_run
//! use freyacn::extensions::SizingExt;
//! use freyacn::button::Button;
//!
//! // Inside a component's render method:
//! let my_button = Button::new()
//!     .w_32()          // width: 128px
//!     .h_16()          // height: 64px
//!     .w_full()        // width: 100%
//!     .h_auto()        // height: auto
//!     .min_w_8()       // min‑width: 32px
//!     .max_w_64()      // max‑width: 256px
//!     .w(100.0)        // width: 100px (raw pixel)
//!     .w_percent(50.0); // width: 50%
//! ```
//!
//! ## Implementing `SizingExt` for your own components
//!
//! To use these helpers on your custom component type, you need to implement
//! the core sizing methods: [`width`](SizingExt::width),
//! [`height`](SizingExt::height), [`min_width`](SizingExt::min_width),
//! [`min_height`](SizingExt::min_height), [`max_width`](SizingExt::max_width), and
//! [`max_height`](SizingExt::max_height). These should forward the sizing to the
//! underlying container.
//!
//! ```no_run
//! # use freyacn::extensions::SizingExt;
//! # use freya::prelude::Size;
//! struct MyComponent;
//!
//! impl SizingExt for MyComponent {
//!     fn width(self, size: impl Into<Size>) -> Self {
//!         self.with_width(size.into())
//!     }
//!     fn height(self, size: impl Into<Size>) -> Self {
//!         self.with_height(size.into())
//!     }
//!     fn min_width(self, size: impl Into<Size>) -> Self {
//!         self.with_min_width(size.into())
//!     }
//!     fn min_height(self, size: impl Into<Size>) -> Self {
//!         self.with_min_height(size.into())
//!     }
//!     fn max_width(self, size: impl Into<Size>) -> Self {
//!         self.with_max_width(size.into())
//!     }
//!     fn max_height(self, size: impl Into<Size>) -> Self {
//!         self.with_max_height(size.into())
//!     }
//! }
//! ```
//!
//! ## Macro Details
//!
//! The [`sizing_scale!`] macro generates all scale methods. It takes the
//! method prefix (e.g., `w`), the base method name (e.g., `w`), and a list
//! of integer literals from the Tailwind spacing scale. It then produces
//! methods like `w_0()`, `w_1()`, …, `w_96()`.
//!
//! ## Notes
//!
//! - The sizing unit is fixed at 4px, matching Tailwind’s default.
//! - All methods return `Self` for method chaining.
//! - The core methods accept any type that implements `Into<Size>`, which
//!   includes `f32`, `Size`, and other compatible types.

use freya::prelude::Size;
use paste::paste;

/// The base spacing unit in pixels (1 unit = 4px), matching Tailwind's default.
const SPACING_UNIT: f32 = 4.0;

/// Macro to generate scale methods for sizing helpers.
///
/// This macro creates methods like `w_0()`, `w_1()`, …, `w_96()` for the given
/// prefix and base method. Each generated method multiplies the scale value by
/// [`SPACING_UNIT`] and calls `self.$base()` with the result.
///
/// # Arguments
///
/// * `$prefix` – The method prefix, e.g., `w` for `w_0()`, `h` for `h_0()`.
/// * `$base` – The base method name to call, e.g., `w` for `w()`.
/// * `$($value:literal),*` – A list of integer literals from the Tailwind spacing scale.
///
/// # Example expansion
///
/// ```ignore
/// sizing_scale!(w, w, 0, 1, 2);
/// ```
///
/// Expands to:
///
/// ```ignore
/// fn w_0(self) -> Self {
///     self.w(SPACING_UNIT * (0 as f32))
/// }
/// fn w_1(self) -> Self {
///     self.w(SPACING_UNIT * (1 as f32))
/// }
/// fn w_2(self) -> Self {
///     self.w(SPACING_UNIT * (2 as f32))
/// }
/// ```
///
/// # Note
///
/// The macro is invoked multiple times inside the trait, once for each prefix
/// (`w`, `h`, `min_w`, `max_w`, `min_h`, `max_h`).
macro_rules! sizing_scale {
    ($prefix:ident, $base:ident, $($value:literal),*) => {
        $(
            paste! {
                fn [<$prefix _ $value>](self) -> Self {
                    self.$base(SPACING_UNIT * ($value as f32))
                }
            }
        )*
    };
}

/// An extension trait that adds Tailwind‑inspired width and height helpers.
///
/// This trait is implemented for any type that can have size constraints
/// (e.g., containers, images, buttons). It provides a comprehensive set of
/// methods to set dimensions using the Tailwind naming convention.
///
/// # Required Methods
///
/// You must implement [`width`](SizingExt::width), [`height`](SizingExt::height),
/// [`min_width`](SizingExt::min_width), [`min_height`](SizingExt::min_height),
/// [`max_width`](SizingExt::max_width), and [`max_height`](SizingExt::max_height)
/// to apply sizing to your component. All other methods are provided by the trait.
///
/// # Examples
///
/// Basic usage:
///
/// ```no_run
/// # use freyacn::extensions::SizingExt;
/// # use freyacn::components::Button;
/// let button = Button()
///     .w_32()          // width: 128px
///     .h_16()          // height: 64px
///     .w_full()        // width: 100%
///     .h_auto()        // height: auto
///     .min_w_8()       // min‑width: 32px
///     .max_w_64()      // max‑width: 256px
///     .w(100.0)        // width: 100px (raw pixel)
///     .w_percent(50.0); // width: 50%
/// ```
///
/// Chaining:
///
/// ```no_run
/// # use freyacn::extensions::SizingExt;
/// # use freyacn::components::Button;
/// let button = Button()
///     .w_4()
///     .h_8()
///     .min_w_2()
///     .max_w_16()
///     .w_full();
/// ```
///
/// # Implementing on your own type
///
/// ```no_run
/// # use freyacn::extensions::SizingExt;
/// # use freya::prelude::Size;
/// struct MyWidget;
///
/// impl SizingExt for MyWidget {
///     fn width(self, size: impl Into<Size>) -> Self {
///         self.with_width(size.into())
///     }
///     fn height(self, size: impl Into<Size>) -> Self {
///         self.with_height(size.into())
///     }
///     fn min_width(self, size: impl Into<Size>) -> Self {
///         self.with_min_width(size.into())
///     }
///     fn min_height(self, size: impl Into<Size>) -> Self {
///         self.with_min_height(size.into())
///     }
///     fn max_width(self, size: impl Into<Size>) -> Self {
///         self.with_max_width(size.into())
///     }
///     fn max_height(self, size: impl Into<Size>) -> Self {
///         self.with_max_height(size.into())
///     }
/// }
/// ```
///
/// # Notes
///
/// - The sizing scale follows Tailwind: `w_1` = 4px, `w_2` = 8px, etc.
/// - The `Into<Size>` bound allows passing `f32`, `Size`, and other types.
/// - Methods are designed to be used inside a component’s `render` method.
pub trait SizingExt: Sized {
    // ---- Core methods that must be implemented by the component ----

    /// Set the width of the component.
    ///
    /// This method must be implemented by your component. It should apply the
    /// width to the underlying container and return `self`.
    ///
    /// # Arguments
    ///
    /// * `size` – The width value. Can be anything that implements `Into<Size>`,
    ///   such as `Size::px(100.0)`, or a raw `f32` (which will be converted to pixels).
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use freyacn::extensions::SizingExt;
    /// # use freya::prelude::Size;
    /// # struct MyComponent;
    /// impl SizingExt for MyComponent {
    ///     fn width(self, size: impl Into<Size>) -> Self {
    ///         self.with_width(size.into())
    ///     }
    ///     // ...
    /// }
    /// ```
    fn width(self, size: impl Into<Size>) -> Self;

    /// Set the height of the component.
    ///
    /// This method must be implemented by your component. It should apply the
    /// height to the underlying container and return `self`.
    ///
    /// # Arguments
    ///
    /// * `size` – The height value. Can be anything that implements `Into<Size>`.
    fn height(self, size: impl Into<Size>) -> Self;

    /// Set the minimum width of the component.
    ///
    /// This method must be implemented by your component. It should apply the
    /// minimum width constraint to the underlying container and return `self`.
    ///
    /// # Arguments
    ///
    /// * `size` – The minimum width value. Can be anything that implements `Into<Size>`.
    fn min_width(self, size: impl Into<Size>) -> Self;

    /// Set the minimum height of the component.
    ///
    /// This method must be implemented by your component. It should apply the
    /// minimum height constraint to the underlying container and return `self`.
    ///
    /// # Arguments
    ///
    /// * `size` – The minimum height value. Can be anything that implements `Into<Size>`.
    fn min_height(self, size: impl Into<Size>) -> Self;

    /// Set the maximum width of the component.
    ///
    /// This method must be implemented by your component. It should apply the
    /// maximum width constraint to the underlying container and return `self`.
    ///
    /// # Arguments
    ///
    /// * `size` – The maximum width value. Can be anything that implements `Into<Size>`.
    fn max_width(self, size: impl Into<Size>) -> Self;

    /// Set the maximum height of the component.
    ///
    /// This method must be implemented by your component. It should apply the
    /// maximum height constraint to the underlying container and return `self`.
    ///
    /// # Arguments
    ///
    /// * `size` – The maximum height value. Can be anything that implements `Into<Size>`.
    fn max_height(self, size: impl Into<Size>) -> Self;

    // ---- Generic pixel methods ----

    /// Set the width in pixels.
    ///
    /// # Arguments
    ///
    /// * `px` – The width in pixels.
    ///
    /// # Example
    ///
    /// ```
    /// # use freyacn::extensions::SizingExt;
    /// # use freyacn::components::Button;
    /// let button = Button().w(100.0); // width: 100px
    /// ```
    fn w(self, px: f32) -> Self {
        self.width(Size::px(px))
    }

    /// Set the height in pixels.
    ///
    /// # Arguments
    ///
    /// * `px` – The height in pixels.
    ///
    /// # Example
    ///
    /// ```
    /// # use freyacn::extensions::SizingExt;
    /// # use freyacn::components::Button;
    /// let button = Button().h(50.0); // height: 50px
    /// ```
    fn h(self, px: f32) -> Self {
        self.height(Size::px(px))
    }

    /// Set the minimum width in pixels.
    ///
    /// # Arguments
    ///
    /// * `px` – The minimum width in pixels.
    fn min_w(self, px: f32) -> Self {
        self.min_width(Size::px(px))
    }

    /// Set the minimum height in pixels.
    ///
    /// # Arguments
    ///
    /// * `px` – The minimum height in pixels.
    fn min_h(self, px: f32) -> Self {
        self.min_height(Size::px(px))
    }

    /// Set the maximum width in pixels.
    ///
    /// # Arguments
    ///
    /// * `px` – The maximum width in pixels.
    fn max_w(self, px: f32) -> Self {
        self.max_width(Size::px(px))
    }

    /// Set the maximum height in pixels.
    ///
    /// # Arguments
    ///
    /// * `px` – The maximum height in pixels.
    fn max_h(self, px: f32) -> Self {
        self.max_height(Size::px(px))
    }

    // ---- Percentage methods ----

    /// Set the width as a percentage of the parent container.
    ///
    /// # Arguments
    ///
    /// * `pct` – The percentage value (e.g., `50.0` for 50%).
    ///
    /// # Example
    ///
    /// ```
    /// # use freyacn::extensions::SizingExt;
    /// # use freyacn::components::Button;
    /// let button = Button().w_percent(75.0); // width: 75% of parent
    /// ```
    fn w_percent(self, pct: f32) -> Self {
        self.width(Size::percent(pct))
    }

    /// Set the height as a percentage of the parent container.
    ///
    /// # Arguments
    ///
    /// * `pct` – The percentage value (e.g., `50.0` for 50%).
    ///
    /// # Example
    ///
    /// ```
    /// # use freyacn::extensions::SizingExt;
    /// # use freyacn::components::Button;
    /// let button = Button().h_percent(50.0); // height: 50% of parent
    /// ```
    fn h_percent(self, pct: f32) -> Self {
        self.height(Size::percent(pct))
    }

    // ---- Tailwind specials ----

    /// Set the width to fill the parent container (100%).
    ///
    /// Equivalent to `width: 100%` in CSS.
    ///
    /// # Example
    ///
    /// ```
    /// # use freyacn::extensions::SizingExt;
    /// # use freyacn::components::Button;
    /// let button = Button().w_full();
    /// ```
    fn w_full(self) -> Self {
        self.width(Size::percent(100.0))
    }

    /// Alias for `w_full()`. Sets the width to fill the screen (100%).
    fn w_screen(self) -> Self {
        self.width(Size::percent(100.0))
    }

    /// Set the width to `auto`, allowing the element to size based on its content.
    ///
    /// # Example
    ///
    /// ```
    /// # use freyacn::extensions::SizingExt;
    /// # use freyacn::components::Button;
    /// let button = Button().w_auto();
    /// ```
    fn w_auto(self) -> Self {
        self.width(Size::auto())
    }

    /// Set the height to fill the parent container (100%).
    ///
    /// Equivalent to `height: 100%` in CSS.
    ///
    /// # Example
    ///
    /// ```
    /// # use freyacn::extensions::SizingExt;
    /// # use freyacn::components::Button;
    /// let button = Button().h_full();
    /// ```
    fn h_full(self) -> Self {
        self.height(Size::percent(100.0))
    }

    /// Alias for `h_full()`. Sets the height to fill the screen (100%).
    fn h_screen(self) -> Self {
        self.height(Size::percent(100.0))
    }

    /// Set the height to `auto`, allowing the element to size based on its content.
    ///
    /// # Example
    ///
    /// ```
    /// # use freyacn::extensions::SizingExt;
    /// # use freyacn::components::Button;
    /// let button = Button().h_auto();
    /// ```
    fn h_auto(self) -> Self {
        self.height(Size::auto())
    }

    // ---- Scale methods ----
    // The following methods are generated by the sizing_scale! macro.
    // They correspond to the Tailwind spacing scale: 0, 1, 2, 3, 4, 5, 6, 8, 10, 12,
    // 16, 20, 24, 32, 48, 64, 96, where each unit equals 4px.
    //
    // For example:
    // - w_0() → width: 0px
    // - w_1() → width: 4px
    // - w_2() → width: 8px
    // - w_4() → width: 16px
    // - w_96() → width: 384px
    //
    // The same applies to h_*, min_w_*, max_w_*, min_h_*, and max_h_*.

    sizing_scale!(
        w, w, 0, 1, 2, 3, 4, 5, 6, 8, 10, 12, 16, 20, 24, 32, 48, 64, 96
    );

    sizing_scale!(
        h, h, 0, 1, 2, 3, 4, 5, 6, 8, 10, 12, 16, 20, 24, 32, 48, 64, 96
    );

    sizing_scale!(
        min_w, min_w, 0, 1, 2, 3, 4, 5, 6, 8, 10, 12, 16, 20, 24, 32, 48, 64, 96
    );

    sizing_scale!(
        max_w, max_w, 0, 1, 2, 3, 4, 5, 6, 8, 10, 12, 16, 20, 24, 32, 48, 64, 96
    );

    sizing_scale!(
        min_h, min_h, 0, 1, 2, 3, 4, 5, 6, 8, 10, 12, 16, 20, 24, 32, 48, 64, 96
    );

    sizing_scale!(
        max_h, max_h, 0, 1, 2, 3, 4, 5, 6, 8, 10, 12, 16, 20, 24, 32, 48, 64, 96
    );
}
