//! # Flex Layout Extension Trait
//!
//! This module provides the [`FlexExt`] trait, which adds Tailwind‑inspired
//! flexbox layout helpers to any component. It enables you to use familiar
//! naming like `flex_col()`, `items_center()`, `justify_between()`, and `gap_4()`
//! to control flex layout directly in your UI code.
//!
//! ## Overview
//!
//! The trait offers four categories of flex helpers:
//!
//! - **Direction** – `flex_col()`, `flex_row()` to set the flex direction.
//! - **Main axis alignment** – `justify_start()`, `justify_center()`, `justify_end()`,
//!   `justify_between()`, `justify_around()`, `justify_evenly()` to align items on
//!   the main axis.
//! - **Cross axis alignment** – `items_start()`, `items_center()`, `items_end()` to
//!   align items on the cross axis.
//! - **Gap (spacing)** – `gap()` for raw pixel values, and `gap_0()`, `gap_1()`, …,
//!   `gap_96()` for the Tailwind spacing scale (1 unit = 4px).
//!
//! All methods delegate to the component’s underlying layout methods, which you
//! implement on your component.
//!
//! ## Examples
//!
//! ```no_run
//! use freyacn::extensions::FlexExt;
//! use freyacn::container::Container;
//!
//! // Inside a component's render method:
//! let my_container = Container::new()
//!     .flex_col()         // vertical layout
//!     .items_center()     // align items to center (cross axis)
//!     .justify_between()  // distribute items evenly (main axis)
//!     .gap_4()            // spacing: 16px between items
//!     .gap(10.0);         // spacing: 10px between items (raw pixel)
//! ```
//!
//! ## Implementing `FlexExt` for your own components
//!
//! To use these helpers on your custom component type, you need to implement
//! the core flex methods: [`direction`](FlexExt::direction),
//! [`main_align`](FlexExt::main_align), [`cross_align`](FlexExt::cross_align), and
//! [`spacing`](FlexExt::spacing). These should forward the layout properties to
//! the underlying container.
//!
//! ```no_run
//! # use freyacn::extensions::FlexExt;
//! # use freya::prelude::{Alignment, Direction};
//! struct MyContainer;
//!
//! impl FlexExt for MyContainer {
//!     fn direction(self, dir: Direction) -> Self {
//!         self.with_direction(dir)
//!     }
//!     fn main_align(self, align: Alignment) -> Self {
//!         self.with_main_align(align)
//!     }
//!     fn cross_align(self, align: Alignment) -> Self {
//!         self.with_cross_align(align)
//!     }
//!     fn spacing(self, spacing: f32) -> Self {
//!         self.with_spacing(spacing)
//!     }
//! }
//! ```
//!
//! ## Macro Details
//!
//! The [`gap_scale!`] macro generates all gap scale methods. It takes a list
//! of integer literals from the Tailwind spacing scale and produces methods like
//! `gap_0()`, `gap_1()`, …, `gap_96()`.
//!
//! ## Notes
//!
//! - The spacing unit is fixed at 4px, matching Tailwind’s default.
//! - All methods return `Self` for method chaining.
//! - Reversed directions (`flex_col_reverse`, `flex_row_reverse`) are not available
//!   in the current version of `torin` (Freya’s layout engine).
//! - Methods are designed to be used inside a component’s `render` method.

use freya::prelude::{Alignment, Direction};
use paste::paste;

/// The base spacing unit in pixels (1 unit = 4px), matching Tailwind's default.
const SPACING_UNIT: f32 = 4.0;

/// Macro to generate gap scale methods.
///
/// This macro creates methods like `gap_0()`, `gap_1()`, …, `gap_96()` for the
/// given list of scale values. Each generated method multiplies the scale value
/// by [`SPACING_UNIT`] and calls [`gap`](FlexExt::gap) with the result.
///
/// # Arguments
///
/// * `$($value:literal),*` – A list of integer literals from the Tailwind spacing scale.
///
/// # Example expansion
///
/// ```ignore
/// gap_scale!(0, 1, 2);
/// ```
///
/// Expands to:
///
/// ```ignore
/// fn gap_0(self) -> Self {
///     self.gap(SPACING_UNIT * (0 as f32))
/// }
/// fn gap_1(self) -> Self {
///     self.gap(SPACING_UNIT * (1 as f32))
/// }
/// fn gap_2(self) -> Self {
///     self.gap(SPACING_UNIT * (2 as f32))
/// }
/// ```
macro_rules! gap_scale {
    ($($value:literal),*) => {
        $(
            paste! {
                fn [<gap_ $value>](self) -> Self {
                    self.gap(SPACING_UNIT * ($value as f32))
                }
            }
        )*
    };
}

/// An extension trait that adds Tailwind‑inspired flexbox layout helpers.
///
/// This trait is implemented for any type that can act as a flex container
/// (e.g., containers, cards, layout components). It provides a comprehensive set
/// of methods to control flex layout using the Tailwind naming convention.
///
/// # Required Methods
///
/// You must implement [`direction`](FlexExt::direction),
/// [`main_align`](FlexExt::main_align), [`cross_align`](FlexExt::cross_align), and
/// [`spacing`](FlexExt::spacing) to apply layout properties to your component.
/// All other methods are provided by the trait.
///
/// # Examples
///
/// Basic usage:
///
/// ```no_run
/// # use freyacn::extensions::FlexExt;
/// # use freyacn::container::Container;
/// let container = Container::new()
///     .flex_col()         // vertical layout
///     .items_center()     // align items to center
///     .justify_between()  // distribute items evenly
///     .gap_4()            // spacing: 16px between items
///     .gap(10.0);         // spacing: 10px between items (raw pixel)
/// ```
///
/// Chaining:
///
/// ```no_run
/// # use freyacn::extensions::FlexExt;
/// # use freyacn::container::Container;
/// let container = Container::new()
///     .flex_row()
///     .justify_center()
///     .items_end()
///     .gap_8();
/// ```
///
/// # Implementing on your own type
///
/// ```no_run
/// # use freyacn::extensions::FlexExt;
/// # use freya::prelude::{Alignment, Direction};
/// struct MyContainer;
///
/// impl FlexExt for MyContainer {
///     fn direction(self, dir: Direction) -> Self {
///         self.with_direction(dir)
///     }
///     fn main_align(self, align: Alignment) -> Self {
///         self.with_main_align(align)
///     }
///     fn cross_align(self, align: Alignment) -> Self {
///         self.with_cross_align(align)
///     }
///     fn spacing(self, spacing: f32) -> Self {
///         self.with_spacing(spacing)
///     }
/// }
/// ```
///
/// # Notes
///
/// - The gap scale follows Tailwind: `gap_1` = 4px, `gap_2` = 8px, etc.
/// - All methods return `Self` for method chaining.
/// - Reversed directions are not supported in the current layout engine.
/// - Methods are designed to be used inside a component’s `render` method.
pub trait FlexExt: Sized {
    // ---- Core methods that must be implemented by the component ----

    /// Set the flex direction of the container.
    ///
    /// This method must be implemented by your component. It should apply the
    /// direction to the underlying container and return `self`.
    ///
    /// # Arguments
    ///
    /// * `dir` – The direction to use: [`Direction::Vertical`] or [`Direction::Horizontal`].
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use freyacn::extensions::FlexExt;
    /// # use freya::prelude::Direction;
    /// # struct MyContainer;
    /// impl FlexExt for MyContainer {
    ///     fn direction(self, dir: Direction) -> Self {
    ///         self.with_direction(dir)
    ///     }
    ///     // ...
    /// }
    /// ```
    fn direction(self, dir: Direction) -> Self;

    /// Set the main axis alignment of the flex container.
    ///
    /// This method must be implemented by your component. It should apply the
    /// main axis alignment to the underlying container and return `self`.
    ///
    /// # Arguments
    ///
    /// * `align` – The alignment to use: [`Alignment::Start`], [`Alignment::Center`],
    ///   [`Alignment::End`], [`Alignment::SpaceBetween`], [`Alignment::SpaceAround`],
    ///   or [`Alignment::SpaceEvenly`].
    fn main_align(self, align: Alignment) -> Self;

    /// Set the cross axis alignment of the flex container.
    ///
    /// This method must be implemented by your component. It should apply the
    /// cross axis alignment to the underlying container and return `self`.
    ///
    /// # Arguments
    ///
    /// * `align` – The alignment to use: [`Alignment::Start`], [`Alignment::Center`],
    ///   or [`Alignment::End`].
    fn cross_align(self, align: Alignment) -> Self;

    /// Set the spacing (gap) between flex items.
    ///
    /// This method must be implemented by your component. It should apply the
    /// gap between items to the underlying container and return `self`.
    ///
    /// # Arguments
    ///
    /// * `spacing` – The gap size in pixels.
    fn spacing(self, spacing: f32) -> Self;

    // ---- Direction ----

    /// Set the flex direction to vertical (column).
    ///
    /// Equivalent to `flex-direction: column` in CSS.
    ///
    /// # Example
    ///
    /// ```
    /// # use freyacn::extensions::FlexExt;
    /// # use freyacn::container::Container;
    /// let container = Container::new().flex_col();
    /// ```
    fn flex_col(self) -> Self {
        self.direction(Direction::Vertical)
    }

    /// Set the flex direction to horizontal (row).
    ///
    /// Equivalent to `flex-direction: row` in CSS.
    ///
    /// # Example
    ///
    /// ```
    /// # use freyacn::extensions::FlexExt;
    /// # use freyacn::container::Container;
    /// let container = Container::new().flex_row();
    /// ```
    fn flex_row(self) -> Self {
        self.direction(Direction::Horizontal)
    }

    // Reversed directions are not available in torin's Direction enum.

    // ---- Main axis alignment ----

    /// Align flex items to the start of the main axis.
    ///
    /// Equivalent to `justify-content: flex-start` in CSS.
    ///
    /// # Example
    ///
    /// ```
    /// # use freyacn::extensions::FlexExt;
    /// # use freyacn::container::Container;
    /// let container = Container::new().justify_start();
    /// ```
    fn justify_start(self) -> Self {
        self.main_align(Alignment::Start)
    }

    /// Align flex items to the center of the main axis.
    ///
    /// Equivalent to `justify-content: center` in CSS.
    ///
    /// # Example
    ///
    /// ```
    /// # use freyacn::extensions::FlexExt;
    /// # use freyacn::container::Container;
    /// let container = Container::new().justify_center();
    /// ```
    fn justify_center(self) -> Self {
        self.main_align(Alignment::Center)
    }

    /// Align flex items to the end of the main axis.
    ///
    /// Equivalent to `justify-content: flex-end` in CSS.
    ///
    /// # Example
    ///
    /// ```
    /// # use freyacn::extensions::FlexExt;
    /// # use freyacn::container::Container;
    /// let container = Container::new().justify_end();
    /// ```
    fn justify_end(self) -> Self {
        self.main_align(Alignment::End)
    }

    /// Distribute flex items with equal space between them.
    ///
    /// Equivalent to `justify-content: space-between` in CSS.
    ///
    /// # Example
    ///
    /// ```
    /// # use freyacn::extensions::FlexExt;
    /// # use freyacn::container::Container;
    /// let container = Container::new().justify_between();
    /// ```
    fn justify_between(self) -> Self {
        self.main_align(Alignment::SpaceBetween)
    }

    /// Distribute flex items with equal space around them.
    ///
    /// Equivalent to `justify-content: space-around` in CSS.
    ///
    /// # Example
    ///
    /// ```
    /// # use freyacn::extensions::FlexExt;
    /// # use freyacn::container::Container;
    /// let container = Container::new().justify_around();
    /// ```
    fn justify_around(self) -> Self {
        self.main_align(Alignment::SpaceAround)
    }

    /// Distribute flex items with equal space evenly.
    ///
    /// Equivalent to `justify-content: space-evenly` in CSS.
    ///
    /// # Example
    ///
    /// ```
    /// # use freyacn::extensions::FlexExt;
    /// # use freyacn::container::Container;
    /// let container = Container::new().justify_evenly();
    /// ```
    fn justify_evenly(self) -> Self {
        self.main_align(Alignment::SpaceEvenly)
    }

    // ---- Cross axis alignment ----

    /// Align flex items to the start of the cross axis.
    ///
    /// Equivalent to `align-items: flex-start` in CSS.
    ///
    /// # Example
    ///
    /// ```
    /// # use freyacn::extensions::FlexExt;
    /// # use freyacn::container::Container;
    /// let container = Container::new().items_start();
    /// ```
    fn items_start(self) -> Self {
        self.cross_align(Alignment::Start)
    }

    /// Align flex items to the center of the cross axis.
    ///
    /// Equivalent to `align-items: center` in CSS.
    ///
    /// # Example
    ///
    /// ```
    /// # use freyacn::extensions::FlexExt;
    /// # use freyacn::container::Container;
    /// let container = Container::new().items_center();
    /// ```
    fn items_center(self) -> Self {
        self.cross_align(Alignment::Center)
    }

    /// Align flex items to the end of the cross axis.
    ///
    /// Equivalent to `align-items: flex-end` in CSS.
    ///
    /// # Example
    ///
    /// ```
    /// # use freyacn::extensions::FlexExt;
    /// # use freyacn::container::Container;
    /// let container = Container::new().items_end();
    /// ```
    fn items_end(self) -> Self {
        self.cross_align(Alignment::End)
    }

    // ---- Gap ----

    /// Set the gap (spacing) between flex items in pixels.
    ///
    /// # Arguments
    ///
    /// * `size` – The gap size in pixels.
    ///
    /// # Example
    ///
    /// ```
    /// # use freyacn::extensions::FlexExt;
    /// # use freyacn::container::Container;
    /// let container = Container::new().gap(8.0); // 8px gap between items
    /// ```
    fn gap(self, size: f32) -> Self {
        self.spacing(size)
    }

    // ---- Gap scale ----
    // The following methods are generated by the gap_scale! macro.
    // They correspond to the Tailwind spacing scale: 0, 1, 2, 3, 4, 5, 6, 8, 10, 12,
    // 16, 20, 24, 32, 48, 64, 96, where each unit equals 4px.
    //
    // For example:
    // - gap_0() → gap: 0px
    // - gap_1() → gap: 4px
    // - gap_2() → gap: 8px
    // - gap_4() → gap: 16px
    // - gap_96() → gap: 384px

    gap_scale!(0, 1, 2, 3, 4, 5, 6, 8, 10, 12, 16, 20, 24, 32, 48, 64, 96);
}
