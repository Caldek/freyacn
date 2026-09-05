//! # Typography Extension Trait
//!
//! This module provides the [`TypographyExt`] trait, which adds Tailwind‑inspired
//! typography helpers to any component. It enables you to set font size, weight,
//! alignment, and decoration using familiar naming conventions like `text_xs()`,
//! `font_bold()`, `text_center()`, and `underline()`.
//!
//! ## Overview
//!
//! The trait offers four categories of typography helpers:
//!
//! - **Font sizes** – `text_xs()`, `text_sm()`, `text_base()`, `text_lg()`, `text_xl()`,
//!   `text_2xl()`, …, `text_9xl()` for Tailwind’s font size scale.
//! - **Font weights** – `font_thin()`, `font_light()`, `font_normal()`, `font_medium()`,
//!   `font_bold()`, `font_extrabold()` for common font weights.
//! - **Text alignment** – `text_left()`, `text_center()`, `text_right()` for horizontal
//!   alignment.
//! - **Text decoration** – `underline()`, `no_underline()` for underline styling.
//!
//! All methods delegate to the component’s underlying typography methods, which you
//! implement on your component.
//!
//! ## Examples
//!
//! ```no_run
//! use freyacn::extensions::TypographyExt;
//! use freyacn::label::Label;
//!
//! // Inside a component's render method:
//! let my_label = Label("Hello, World!")
//!     .text_xl()          // font size: 20px
//!     .font_bold()        // font weight: bold
//!     .text_center()      // center alignment
//!     .underline();       // underlined text
//! ```
//!
//! ## Implementing `TypographyExt` for your own components
//!
//! To use these helpers on your custom component type, you need to implement
//! the core typography methods: [`font_size`](TypographyExt::font_size),
//! [`font_weight`](TypographyExt::font_weight), [`text_align`](TypographyExt::text_align),
//! and [`text_decoration`](TypographyExt::text_decoration). These should forward the
//! styling to the underlying text element.
//!
//! ```no_run
//! # use freyacn::extensions::TypographyExt;
//! # use freya::prelude::{FontWeight, TextAlign, TextDecoration};
//! struct MyLabel;
//!
//! impl TypographyExt for MyLabel {
//!     fn font_size(self, size: f32) -> Self {
//!         self.with_font_size(size)
//!     }
//!     fn font_weight(self, weight: FontWeight) -> Self {
//!         self.with_font_weight(weight)
//!     }
//!     fn text_align(self, align: TextAlign) -> Self {
//!         self.with_text_align(align)
//!     }
//!     fn text_decoration(self, decoration: TextDecoration) -> Self {
//!         self.with_text_decoration(decoration)
//!     }
//! }
//! ```
//!
//! ## Notes
//!
//! - Font sizes follow Tailwind’s default scale, measured in pixels.
//! - Font weights map to Freya’s `FontWeight` enum, which supports thin, light,
//!   normal, medium, bold, and extra‑bold.
//! - Alignment and decoration are applied to the text element directly.
//! - All methods return `Self` for method chaining.
//! - Methods are designed to be used inside a component’s `render` method.

use freya::prelude::{FontWeight, TextAlign, TextDecoration};

/// An extension trait that adds Tailwind‑inspired typography helpers.
///
/// This trait is implemented for any type that can display text (e.g., labels,
/// buttons, headers, text components). It provides a comprehensive set of
/// methods to control typography using the Tailwind naming convention.
///
/// # Required Methods
///
/// You must implement [`font_size`](TypographyExt::font_size),
/// [`font_weight`](TypographyExt::font_weight), [`text_align`](TypographyExt::text_align),
/// and [`text_decoration`](TypographyExt::text_decoration) to apply typography
/// styling to your component. All other methods are provided by the trait.
///
/// # Examples
///
/// Basic usage:
///
/// ```no_run
/// # use freyacn::extensions::TypographyExt;
/// # use freyacn::label::Label;
/// let label = Label("Hello")
///     .text_xl()          // large text
///     .font_bold()        // bold weight
///     .text_center()      // center alignment
///     .underline();       // underline
/// ```
///
/// Custom font sizes:
///
/// ```no_run
/// # use freyacn::extensions::TypographyExt;
/// # use freyacn::label::Label;
/// let label = Label("Hello")
///     .font_size(24.0)    // 24px custom font size
///     .font_medium()      // medium weight
///     .text_left();       // left alignment
/// ```
///
/// # Implementing on your own type
///
/// ```no_run
/// # use freyacn::extensions::TypographyExt;
/// # use freya::prelude::{FontWeight, TextAlign, TextDecoration};
/// struct MyLabel;
///
/// impl TypographyExt for MyLabel {
///     fn font_size(self, size: f32) -> Self {
///         self.with_font_size(size)
///     }
///     fn font_weight(self, weight: FontWeight) -> Self {
///         self.with_font_weight(weight)
///     }
///     fn text_align(self, align: TextAlign) -> Self {
///         self.with_text_align(align)
///     }
///     fn text_decoration(self, decoration: TextDecoration) -> Self {
///         self.with_text_decoration(decoration)
///     }
/// }
/// ```
///
/// # Notes
///
/// - The font size helpers follow Tailwind’s scale (12px to 128px).
/// - For custom font sizes, use [`font_size`](TypographyExt::font_size) directly.
/// - For custom font weights beyond those provided, use [`font_weight`](TypographyExt::font_weight)
///   with a `FontWeight` value.
pub trait TypographyExt: Sized {
    // ---- Core methods that must be implemented by the component ----

    /// Set the font size in pixels.
    ///
    /// This method must be implemented by your component. It should apply the
    /// font size to the underlying text element and return `self`.
    ///
    /// # Arguments
    ///
    /// * `size` – The font size in pixels.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use freyacn::extensions::TypographyExt;
    /// # struct MyLabel;
    /// impl TypographyExt for MyLabel {
    ///     fn font_size(self, size: f32) -> Self {
    ///         self.with_font_size(size)
    ///     }
    ///     // ...
    /// }
    /// ```
    fn font_size(self, size: f32) -> Self;

    /// Set the font weight.
    ///
    /// This method must be implemented by your component. It should apply the
    /// font weight to the underlying text element and return `self`.
    ///
    /// # Arguments
    ///
    /// * `weight` – The [`FontWeight`] to use.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use freyacn::extensions::TypographyExt;
    /// # use freya::prelude::FontWeight;
    /// # struct MyLabel;
    /// impl TypographyExt for MyLabel {
    ///     fn font_weight(self, weight: FontWeight) -> Self {
    ///         self.with_font_weight(weight)
    ///     }
    ///     // ...
    /// }
    /// ```
    fn font_weight(self, weight: FontWeight) -> Self;

    /// Set the text alignment.
    ///
    /// This method must be implemented by your component. It should apply the
    /// text alignment to the underlying text element and return `self`.
    ///
    /// # Arguments
    ///
    /// * `align` – The [`TextAlign`] to use (Left, Center, or Right).
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use freyacn::extensions::TypographyExt;
    /// # use freya::prelude::TextAlign;
    /// # struct MyLabel;
    /// impl TypographyExt for MyLabel {
    ///     fn text_align(self, align: TextAlign) -> Self {
    ///         self.with_text_align(align)
    ///     }
    ///     // ...
    /// }
    /// ```
    fn text_align(self, align: TextAlign) -> Self;

    /// Set the text decoration.
    ///
    /// This method must be implemented by your component. It should apply the
    /// text decoration to the underlying text element and return `self`.
    ///
    /// # Arguments
    ///
    /// * `decoration` – The [`TextDecoration`] to use (Underline or None).
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use freyacn::extensions::TypographyExt;
    /// # use freya::prelude::TextDecoration;
    /// # struct MyLabel;
    /// impl TypographyExt for MyLabel {
    ///     fn text_decoration(self, decoration: TextDecoration) -> Self {
    ///         self.with_text_decoration(decoration)
    ///     }
    ///     // ...
    /// }
    /// ```
    fn text_decoration(self, decoration: TextDecoration) -> Self;

    // ---- Font sizes (Tailwind text-*) ----
    // The following methods correspond to Tailwind's font size scale.
    // Values are in pixels.

    /// Extra‑small font size: 12px.
    ///
    /// Equivalent to `text-xs` in Tailwind.
    ///
    /// # Example
    ///
    /// ```
    /// # use freyacn::extensions::TypographyExt;
    /// # use freyacn::label::Label;
    /// let label = Label("Small text").text_xs();
    /// ```
    fn text_xs(self) -> Self {
        self.font_size(12.0)
    }

    /// Small font size: 14px.
    ///
    /// Equivalent to `text-sm` in Tailwind.
    ///
    /// # Example
    ///
    /// ```
    /// # use freyacn::extensions::TypographyExt;
    /// # use freyacn::label::Label;
    /// let label = Label("Small text").text_sm();
    /// ```
    fn text_sm(self) -> Self {
        self.font_size(14.0)
    }

    /// Base font size: 16px.
    ///
    /// Equivalent to `text-base` in Tailwind.
    ///
    /// # Example
    ///
    /// ```
    /// # use freyacn::extensions::TypographyExt;
    /// # use freyacn::label::Label;
    /// let label = Label("Base text").text_base();
    /// ```
    fn text_base(self) -> Self {
        self.font_size(16.0)
    }

    /// Large font size: 18px.
    ///
    /// Equivalent to `text-lg` in Tailwind.
    ///
    /// # Example
    ///
    /// ```
    /// # use freyacn::extensions::TypographyExt;
    /// # use freyacn::label::Label;
    /// let label = Label("Large text").text_lg();
    /// ```
    fn text_lg(self) -> Self {
        self.font_size(18.0)
    }

    /// Extra‑large font size: 20px.
    ///
    /// Equivalent to `text-xl` in Tailwind.
    ///
    /// # Example
    ///
    /// ```
    /// # use freyacn::extensions::TypographyExt;
    /// # use freyacn::label::Label;
    /// let label = Label("XL text").text_xl();
    /// ```
    fn text_xl(self) -> Self {
        self.font_size(20.0)
    }

    /// 2XL font size: 24px.
    ///
    /// Equivalent to `text-2xl` in Tailwind.
    ///
    /// # Example
    ///
    /// ```
    /// # use freyacn::extensions::TypographyExt;
    /// # use freyacn::label::Label;
    /// let label = Label("2XL text").text_2xl();
    /// ```
    fn text_2xl(self) -> Self {
        self.font_size(24.0)
    }

    /// 3XL font size: 30px.
    ///
    /// Equivalent to `text-3xl` in Tailwind.
    ///
    /// # Example
    ///
    /// ```
    /// # use freyacn::extensions::TypographyExt;
    /// # use freyacn::label::Label;
    /// let label = Label("3XL text").text_3xl();
    /// ```
    fn text_3xl(self) -> Self {
        self.font_size(30.0)
    }

    /// 4XL font size: 36px.
    ///
    /// Equivalent to `text-4xl` in Tailwind.
    ///
    /// # Example
    ///
    /// ```
    /// # use freyacn::extensions::TypographyExt;
    /// # use freyacn::label::Label;
    /// let label = Label("4XL text").text_4xl();
    /// ```
    fn text_4xl(self) -> Self {
        self.font_size(36.0)
    }

    /// 5XL font size: 48px.
    ///
    /// Equivalent to `text-5xl` in Tailwind.
    ///
    /// # Example
    ///
    /// ```
    /// # use freyacn::extensions::TypographyExt;
    /// # use freyacn::label::Label;
    /// let label = Label("5XL text").text_5xl();
    /// ```
    fn text_5xl(self) -> Self {
        self.font_size(48.0)
    }

    /// 6XL font size: 60px.
    ///
    /// Equivalent to `text-6xl` in Tailwind.
    ///
    /// # Example
    ///
    /// ```
    /// # use freyacn::extensions::TypographyExt;
    /// # use freyacn::label::Label;
    /// let label = Label("6XL text").text_6xl();
    /// ```
    fn text_6xl(self) -> Self {
        self.font_size(60.0)
    }

    /// 7XL font size: 72px.
    ///
    /// Equivalent to `text-7xl` in Tailwind.
    ///
    /// # Example
    ///
    /// ```
    /// # use freyacn::extensions::TypographyExt;
    /// # use freyacn::label::Label;
    /// let label = Label("7XL text").text_7xl();
    /// ```
    fn text_7xl(self) -> Self {
        self.font_size(72.0)
    }

    /// 8XL font size: 96px.
    ///
    /// Equivalent to `text-8xl` in Tailwind.
    ///
    /// # Example
    ///
    /// ```
    /// # use freyacn::extensions::TypographyExt;
    /// # use freyacn::label::Label;
    /// let label = Label("8XL text").text_8xl();
    /// ```
    fn text_8xl(self) -> Self {
        self.font_size(96.0)
    }

    /// 9XL font size: 128px.
    ///
    /// Equivalent to `text-9xl` in Tailwind.
    ///
    /// # Example
    ///
    /// ```
    /// # use freyacn::extensions::TypographyExt;
    /// # use freyacn::label::Label;
    /// let label = Label("9XL text").text_9xl();
    /// ```
    fn text_9xl(self) -> Self {
        self.font_size(128.0)
    }

    // ---- Font weights ----
    // The following methods map to Freya's FontWeight enum.

    /// Thin font weight.
    ///
    /// Equivalent to `font-thin` in Tailwind.
    ///
    /// # Example
    ///
    /// ```
    /// # use freyacn::extensions::TypographyExt;
    /// # use freyacn::label::Label;
    /// let label = Label("Thin text").font_thin();
    /// ```
    fn font_thin(self) -> Self {
        self.font_weight(FontWeight::THIN)
    }

    /// Light font weight.
    ///
    /// Equivalent to `font-light` in Tailwind.
    ///
    /// # Example
    ///
    /// ```
    /// # use freyacn::extensions::TypographyExt;
    /// # use freyacn::label::Label;
    /// let label = Label("Light text").font_light();
    /// ```
    fn font_light(self) -> Self {
        self.font_weight(FontWeight::LIGHT)
    }

    /// Normal font weight.
    ///
    /// Equivalent to `font-normal` in Tailwind.
    ///
    /// # Example
    ///
    /// ```
    /// # use freyacn::extensions::TypographyExt;
    /// # use freyacn::label::Label;
    /// let label = Label("Normal text").font_normal();
    /// ```
    fn font_normal(self) -> Self {
        self.font_weight(FontWeight::NORMAL)
    }

    /// Medium font weight.
    ///
    /// Equivalent to `font-medium` in Tailwind.
    ///
    /// # Example
    ///
    /// ```
    /// # use freyacn::extensions::TypographyExt;
    /// # use freyacn::label::Label;
    /// let label = Label("Medium text").font_medium();
    /// ```
    fn font_medium(self) -> Self {
        self.font_weight(FontWeight::MEDIUM)
    }

    /// Bold font weight.
    ///
    /// Equivalent to `font-bold` in Tailwind.
    ///
    /// # Example
    ///
    /// ```
    /// # use freyacn::extensions::TypographyExt;
    /// # use freyacn::label::Label;
    /// let label = Label("Bold text").font_bold();
    /// ```
    fn font_bold(self) -> Self {
        self.font_weight(FontWeight::BOLD)
    }

    /// Extra‑bold font weight.
    ///
    /// Equivalent to `font-extrabold` in Tailwind.
    ///
    /// # Example
    ///
    /// ```
    /// # use freyacn::extensions::TypographyExt;
    /// # use freyacn::label::Label;
    /// let label = Label("Extra bold text").font_extrabold();
    /// ```
    fn font_extrabold(self) -> Self {
        self.font_weight(FontWeight::EXTRA_BOLD)
    }

    // ---- Text alignment ----
    // The following methods set horizontal alignment.

    /// Align text to the left.
    ///
    /// Equivalent to `text-left` in Tailwind.
    ///
    /// # Example
    ///
    /// ```
    /// # use freyacn::extensions::TypographyExt;
    /// # use freyacn::label::Label;
    /// let label = Label("Left aligned").text_left();
    /// ```
    fn text_left(self) -> Self {
        self.text_align(TextAlign::Left)
    }

    /// Align text to the center.
    ///
    /// Equivalent to `text-center` in Tailwind.
    ///
    /// # Example
    ///
    /// ```
    /// # use freyacn::extensions::TypographyExt;
    /// # use freyacn::label::Label;
    /// let label = Label("Centered").text_center();
    /// ```
    fn text_center(self) -> Self {
        self.text_align(TextAlign::Center)
    }

    /// Align text to the right.
    ///
    /// Equivalent to `text-right` in Tailwind.
    ///
    /// # Example
    ///
    /// ```
    /// # use freyacn::extensions::TypographyExt;
    /// # use freyacn::label::Label;
    /// let label = Label("Right aligned").text_right();
    /// ```
    fn text_right(self) -> Self {
        self.text_align(TextAlign::Right)
    }

    // ---- Text decoration ----
    // The following methods set underline styling.

    /// Apply underline to the text.
    ///
    /// Equivalent to `underline` in Tailwind.
    ///
    /// # Example
    ///
    /// ```
    /// # use freyacn::extensions::TypographyExt;
    /// # use freyacn::label::Label;
    /// let label = Label("Underlined").underline();
    /// ```
    fn underline(self) -> Self {
        self.text_decoration(TextDecoration::Underline)
    }

    /// Remove underline from the text.
    ///
    /// Equivalent to `no-underline` in Tailwind.
    ///
    /// # Example
    ///
    /// ```
    /// # use freyacn::extensions::TypographyExt;
    /// # use freyacn::label::Label;
    /// let label = Label("No underline").no_underline();
    /// ```
    fn no_underline(self) -> Self {
        self.text_decoration(TextDecoration::None)
    }
}
