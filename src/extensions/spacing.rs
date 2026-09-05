//! # Spacing Extension Trait
//!
//! This module provides the [`SpacingExt`] trait, which adds Tailwind‑inspired
//! padding and margin helpers to any component. It enables you to use familiar
//! naming like `p_4()`, `mx_2()`, and `py_3()` to apply spacing directly in your
//! UI code.
//!
//! ## Overview
//!
//! The trait offers two categories of spacing helpers:
//!
//! - **Generic padding and margin** – `p()`, `px()`, `py()`, `pt()`, `pr()`,
//!   `pb()`, `pl()` for padding, and `m()`, `mx()`, `my()`, `mt()`, `mr()`,
//!   `mb()`, `ml()` for margin. These accept raw pixel values.
//! - **Scale methods** – `p_0()`, `p_1()`, `p_2()`, …, up to `p_96()`, and
//!   equivalents for `px_*`, `py_*`, etc., and `m_*`, `mx_*`, etc. These follow
//!   the Tailwind spacing scale where 1 unit = 4px.
//!
//! All methods delegate to the component’s [`padding`](SpacingExt::padding) and
//! [`margin`](SpacingExt::margin) methods, which you implement on your component.
//!
//! ## Examples
//!
//! ```no_run
//! use freyacn::extensions::SpacingExt;
//! use freyacn::components::Button;
//!
//! // Inside a component's render method:
//! let my_button = Button()
//!     .p_4()           // padding: 16px all sides
//!     .mx_2()          // margin: 8px left & right
//!     .py_3()          // padding: 12px top & bottom
//!     .mt_6()          // margin: 24px top
//!     .p(10.0)         // padding: 10px all sides (raw pixel value)
//!     .mx(5.0);        // margin: 5px left & right
//! ```
//!
//! ## Implementing `SpacingExt` for your own components
//!
//! To use these helpers on your custom component type, you need to implement
//! the [`padding`](SpacingExt::padding) and [`margin`](SpacingExt::margin)
//! methods. These should forward the spacing to the underlying container.
//!
//! ```no_run
//! # use freyacn::extensions::SpacingExt;
//! # use freya::prelude::Gaps;
//! struct MyComponent;
//!
//! impl SpacingExt for MyComponent {
//!     fn padding(self, gaps: impl Into<Gaps>) -> Self {
//!         // Apply padding to your component's container
//!         self.with_padding(gaps.into())
//!     }
//!
//!     fn margin(self, gaps: impl Into<Gaps>) -> Self {
//!         // Apply margin to your component's container
//!         self.with_margin(gaps.into())
//!     }
//! }
//! ```
//!
//! ## Macro Details
//!
//! The [`spacing_scale!`] macro generates all scale methods. It takes the
//! method prefix (e.g., `p`), the base method name (e.g., `p`), and a list
//! of integer literals from the Tailwind spacing scale. It then produces
//! methods like `p_0()`, `p_1()`, …, `p_96()`.
//!
//! ## Notes
//!
//! - The spacing unit is fixed at 4px, matching Tailwind’s default.
//! - All methods return `Self` for method chaining.
//! - The `padding` and `margin` methods accept any type that implements
//!   `Into<Gaps>`, which includes tuples and `Gaps` itself.

use freya::prelude::Gaps;
use paste::paste;

/// The base spacing unit in pixels (1 unit = 4px), matching Tailwind's default.
const SPACING_UNIT: f32 = 4.0;

/// Macro to generate scale methods for spacing helpers.
///
/// This macro creates methods like `p_0()`, `p_1()`, …, `p_96()` for the given
/// prefix and base method. Each generated method multiplies the scale value by
/// [`SPACING_UNIT`] and calls `self.$base()` with the result.
///
/// # Arguments
///
/// * `$prefix` – The method prefix, e.g., `p` for `p_0()`, `px` for `px_0()`.
/// * `$base` – The base method name to call, e.g., `p` for `p()`.
/// * `$($value:literal),*` – A list of integer literals from the Tailwind spacing scale.
///
/// # Example expansion
///
/// ```ignore
/// spacing_scale!(p, p, 0, 1, 2);
/// ```
///
/// Expands to:
///
/// ```ignore
/// fn p_0(self) -> Self {
///     self.p(SPACING_UNIT * (0 as f32))
/// }
/// fn p_1(self) -> Self {
///     self.p(SPACING_UNIT * (1 as f32))
/// }
/// fn p_2(self) -> Self {
///     self.p(SPACING_UNIT * (2 as f32))
/// }
/// ```
///
/// # Note
///
/// The macro is invoked multiple times inside the trait, once for each prefix
/// (`p`, `px`, `py`, `pt`, `pr`, `pb`, `pl`, `m`, `mx`, `my`, `mt`, `mr`, `mb`, `ml`).
macro_rules! spacing_scale {
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

/// An extension trait that adds Tailwind‑inspired padding and margin helpers.
///
/// This trait is implemented for any type that can have padding and margin
/// (e.g., buttons, cards, containers). It provides a comprehensive set of
/// methods to set spacing using the Tailwind naming convention.
///
/// # Required Methods
///
/// You must implement [`padding`](SpacingExt::padding) and
/// [`margin`](SpacingExt::margin) to apply the spacing to your component.
/// All other methods are provided by the trait.
///
/// # Examples
///
/// Basic usage:
///
/// ```no_run
/// # use freyacn::extensions::SpacingExt;
/// # use freyacn::components::Button;
/// let button = Button()
///     .p_4()          // 16px padding all sides
///     .mx_2()         // 8px margin left & right
///     .pt_3()         // 12px padding top
///     .m(10.0);       // 10px margin all sides (raw pixel)
/// ```
///
/// Chaining:
///
/// ```no_run
/// # use freyacn::extensions::SpacingExt;
/// # use freyacn::components::Button;
/// let button = Button()
///     .p_4()
///     .px_6()
///     .py_2()
///     .mt_8();
/// ```
///
/// # Implementing on your own type
///
/// ```no_run
/// # use freyacn::extensions::SpacingExt;
/// # use freya::prelude::Gaps;
/// struct MyWidget;
///
/// impl SpacingExt for MyWidget {
///     fn padding(self, gaps: impl Into<Gaps>) -> Self {
///         self.with_padding(gaps.into())
///     }
///
///     fn margin(self, gaps: impl Into<Gaps>) -> Self {
///         self.with_margin(gaps.into())
///     }
/// }
/// ```
///
/// # Notes
///
/// - The spacing scale follows Tailwind: `p_1` = 4px, `p_2` = 8px, etc.
/// - The `Into<Gaps>` bound allows passing tuples like `(10.0, 0.0, 10.0, 0.0)`.
/// - Methods are designed to be used inside a component’s `render` method.
pub trait SpacingExt: Sized {
    // ---- Core methods that must be implemented by the component ----

    /// Set padding on the component.
    ///
    /// This method must be implemented by your component. It should apply the
    /// padding to the underlying container and return `self`.
    ///
    /// # Arguments
    ///
    /// * `gaps` – The padding values. Can be anything that implements `Into<Gaps>`,
    ///   such as `Gaps::new_all(10.0)`, or a tuple `(top, right, bottom, left)`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use freyacn::extensions::SpacingExt;
    /// # use freya::prelude::Gaps;
    /// # struct MyComponent;
    /// impl SpacingExt for MyComponent {
    ///     fn padding(self, gaps: impl Into<Gaps>) -> Self {
    ///         self.with_padding(gaps.into())
    ///     }
    ///     // ...
    /// }
    /// ```
    fn padding(self, gaps: impl Into<Gaps>) -> Self;

    /// Set margin on the component.
    ///
    /// This method must be implemented by your component. It should apply the
    /// margin to the underlying container and return `self`.
    ///
    /// # Arguments
    ///
    /// * `gaps` – The margin values. Can be anything that implements `Into<Gaps>`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use freyacn::extensions::SpacingExt;
    /// # use freya::prelude::Gaps;
    /// # struct MyComponent;
    /// impl SpacingExt for MyComponent {
    ///     fn margin(self, gaps: impl Into<Gaps>) -> Self {
    ///         self.with_margin(gaps.into())
    ///     }
    ///     // ...
    /// }
    /// ```
    fn margin(self, gaps: impl Into<Gaps>) -> Self;

    // ---- Generic padding methods (raw pixel values) ----

    /// Set padding on all sides.
    ///
    /// # Arguments
    ///
    /// * `size` – The padding size in pixels.
    ///
    /// # Example
    ///
    /// ```
    /// # use freyacn::extensions::SpacingExt;
    /// # use freyacn::components::Button;
    /// let button = Button().p(10.0); // 10px padding all sides
    /// ```
    fn p(self, size: f32) -> Self {
        self.padding(Gaps::new_all(size))
    }

    /// Set padding on the x‑axis (left and right).
    ///
    /// # Arguments
    ///
    /// * `size` – The padding size in pixels.
    ///
    /// # Example
    ///
    /// ```
    /// # use freyacn::extensions::SpacingExt;
    /// # use freyacn::components::Button;
    /// let button = Button().px(8.0); // 8px padding left & right
    /// ```
    fn px(self, size: f32) -> Self {
        self.padding(Gaps::new(size, 0.0, size, 0.0))
    }

    /// Set padding on the y‑axis (top and bottom).
    ///
    /// # Arguments
    ///
    /// * `size` – The padding size in pixels.
    ///
    /// # Example
    ///
    /// ```
    /// # use freyacn::extensions::SpacingExt;
    /// # use freyacn::components::Button;
    /// let button = Button().py(6.0); // 6px padding top & bottom
    /// ```
    fn py(self, size: f32) -> Self {
        self.padding(Gaps::new(0.0, size, 0.0, size))
    }

    /// Set padding on the top side.
    ///
    /// # Arguments
    ///
    /// * `size` – The padding size in pixels.
    fn pt(self, size: f32) -> Self {
        self.padding(Gaps::new(size, 0.0, 0.0, 0.0))
    }

    /// Set padding on the right side.
    ///
    /// # Arguments
    ///
    /// * `size` – The padding size in pixels.
    fn pr(self, size: f32) -> Self {
        self.padding(Gaps::new(0.0, size, 0.0, 0.0))
    }

    /// Set padding on the bottom side.
    ///
    /// # Arguments
    ///
    /// * `size` – The padding size in pixels.
    fn pb(self, size: f32) -> Self {
        self.padding(Gaps::new(0.0, 0.0, size, 0.0))
    }

    /// Set padding on the left side.
    ///
    /// # Arguments
    ///
    /// * `size` – The padding size in pixels.
    fn pl(self, size: f32) -> Self {
        self.padding(Gaps::new(0.0, 0.0, 0.0, size))
    }

    // ---- Generic margin methods (raw pixel values) ----

    /// Set margin on all sides.
    ///
    /// # Arguments
    ///
    /// * `size` – The margin size in pixels.
    fn m(self, size: f32) -> Self {
        self.margin(Gaps::new_all(size))
    }

    /// Set margin on the x‑axis (left and right).
    ///
    /// # Arguments
    ///
    /// * `size` – The margin size in pixels.
    fn mx(self, size: f32) -> Self {
        self.margin(Gaps::new(size, 0.0, size, 0.0))
    }

    /// Set margin on the y‑axis (top and bottom).
    ///
    /// # Arguments
    ///
    /// * `size` – The margin size in pixels.
    fn my(self, size: f32) -> Self {
        self.margin(Gaps::new(0.0, size, 0.0, size))
    }

    /// Set margin on the top side.
    ///
    /// # Arguments
    ///
    /// * `size` – The margin size in pixels.
    fn mt(self, size: f32) -> Self {
        self.margin(Gaps::new(size, 0.0, 0.0, 0.0))
    }

    /// Set margin on the right side.
    ///
    /// # Arguments
    ///
    /// * `size` – The margin size in pixels.
    fn mr(self, size: f32) -> Self {
        self.margin(Gaps::new(0.0, size, 0.0, 0.0))
    }

    /// Set margin on the bottom side.
    ///
    /// # Arguments
    ///
    /// * `size` – The margin size in pixels.
    fn mb(self, size: f32) -> Self {
        self.margin(Gaps::new(0.0, 0.0, size, 0.0))
    }

    /// Set margin on the left side.
    ///
    /// # Arguments
    ///
    /// * `size` – The margin size in pixels.
    fn ml(self, size: f32) -> Self {
        self.margin(Gaps::new(0.0, 0.0, 0.0, size))
    }

    // ---- Scale methods (Tailwind spacing scale) ----
    // The following methods are generated by the spacing_scale! macro.
    // They correspond to the Tailwind spacing scale: 0, 1, 2, 3, 4, 5, 6, 8, 10, 12,
    // 16, 20, 24, 32, 48, 64, 96, where each unit equals 4px.
    //
    // For example:
    // - p_0() → padding: 0px all sides
    // - p_1() → padding: 4px all sides
    // - p_2() → padding: 8px all sides
    // - p_4() → padding: 16px all sides
    // - p_96() → padding: 384px all sides
    //
    // The same applies to px_*, py_*, pt_*, pr_*, pb_*, pl_*, and margin variants.

    spacing_scale!(
        p, p, 0, 1, 2, 3, 4, 5, 6, 8, 10, 12, 16, 20, 24, 32, 48, 64, 96
    );

    spacing_scale!(
        px, px, 0, 1, 2, 3, 4, 5, 6, 8, 10, 12, 16, 20, 24, 32, 48, 64, 96
    );

    spacing_scale!(
        py, py, 0, 1, 2, 3, 4, 5, 6, 8, 10, 12, 16, 20, 24, 32, 48, 64, 96
    );

    spacing_scale!(
        pt, pt, 0, 1, 2, 3, 4, 5, 6, 8, 10, 12, 16, 20, 24, 32, 48, 64, 96
    );

    spacing_scale!(
        pr, pr, 0, 1, 2, 3, 4, 5, 6, 8, 10, 12, 16, 20, 24, 32, 48, 64, 96
    );

    spacing_scale!(
        pb, pb, 0, 1, 2, 3, 4, 5, 6, 8, 10, 12, 16, 20, 24, 32, 48, 64, 96
    );

    spacing_scale!(
        pl, pl, 0, 1, 2, 3, 4, 5, 6, 8, 10, 12, 16, 20, 24, 32, 48, 64, 96
    );

    // ---- Margin scale methods ----
    spacing_scale!(
        m, m, 0, 1, 2, 3, 4, 5, 6, 8, 10, 12, 16, 20, 24, 32, 48, 64, 96
    );

    spacing_scale!(
        mx, mx, 0, 1, 2, 3, 4, 5, 6, 8, 10, 12, 16, 20, 24, 32, 48, 64, 96
    );

    spacing_scale!(
        my, my, 0, 1, 2, 3, 4, 5, 6, 8, 10, 12, 16, 20, 24, 32, 48, 64, 96
    );

    spacing_scale!(
        mt, mt, 0, 1, 2, 3, 4, 5, 6, 8, 10, 12, 16, 20, 24, 32, 48, 64, 96
    );

    spacing_scale!(
        mr, mr, 0, 1, 2, 3, 4, 5, 6, 8, 10, 12, 16, 20, 24, 32, 48, 64, 96
    );

    spacing_scale!(
        mb, mb, 0, 1, 2, 3, 4, 5, 6, 8, 10, 12, 16, 20, 24, 32, 48, 64, 96
    );

    spacing_scale!(
        ml, ml, 0, 1, 2, 3, 4, 5, 6, 8, 10, 12, 16, 20, 24, 32, 48, 64, 96
    );
}
