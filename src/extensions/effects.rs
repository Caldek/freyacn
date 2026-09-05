//! # Visual Effects Extension Trait
//!
//! This module provides the [`EffectsExt`] trait, which adds Tailwind‑inspired
//! visual effect helpers to any component. It enables you to set opacity and
//! shadows using familiar naming conventions like `opacity_50()`, `shadow_sm()`,
//! and `shadow_none()`.
//!
//! ## Overview
//!
//! The trait offers two categories of effect helpers:
//!
//! - **Opacity** – `opacity_0()`, `opacity_25()`, `opacity_50()`, `opacity_75()`,
//!   `opacity_100()` for common opacity values.
//! - **Shadows** – `shadow_sm()`, `shadow_md()`, `shadow_lg()`, `shadow_xl()`,
//!   `shadow_none()` for predefined shadow sizes.
//!
//! All methods delegate to the component’s underlying effect methods, which you
//! implement on your component.
//!
//! ## Examples
//!
//! ```no_run
//! use freyacn::extensions::EffectsExt;
//! use freyacn::components::Button;
//!
//! // Inside a component's render method:
//! let my_button = Button()
//!     .opacity_50()      // 50% opacity
//!     .shadow_md()       // medium shadow
//!     .opacity(0.75)     // 75% opacity (raw value)
//!     .shadow_none();    // remove shadow
//! ```
//!
//! ## Implementing `EffectsExt` for your own components
//!
//! To use these helpers on your custom component type, you need to implement
//! the core methods: [`opacity`](EffectsExt::opacity) and
//! [`shadow`](EffectsExt::shadow). These should forward the effect styling
//! to the underlying container.
//!
//! ```no_run
//! # use freyacn::extensions::EffectsExt;
//! # use freyacn::extensions::Shadow;
//! struct MyComponent;
//!
//! impl EffectsExt for MyComponent {
//!     fn opacity(self, opacity: f32) -> Self {
//!         self.with_opacity(opacity)
//!     }
//!     fn shadow(self, shadow: impl Into<Shadow>) -> Self {
//!         self.with_shadow(shadow.into())
//!     }
//! }
//! ```
//!
//! ## Shadow Implementation
//!
//! The [`Shadow`] type in this module is a placeholder. You should replace it
//! with the actual shadow type used by Freya (e.g., `BoxShadow`, `Shadow`, or
//! similar). If Freya does not provide a built‑in shadow type, you can define
//! your own that matches your layout engine's expectations.
//!
//! ## Notes
//!
//! - Opacity values range from `0.0` (completely transparent) to `1.0` (fully opaque).
//! - Shadow sizes follow Tailwind’s naming: `sm`, `md`, `lg`, `xl`.
//! - All methods return `Self` for method chaining.
//! - Methods are designed to be used inside a component’s `render` method.

/// A placeholder shadow type.
///
/// This type represents a shadow effect. You should replace this with the actual
/// shadow type from Freya or define your own that matches your layout engine's
/// shadow API (e.g., `BoxShadow`, `Shadow`, etc.).
///
/// The default implementation provides empty constructors for the most common
/// shadow sizes. You can override these to fit your needs.
///
/// # Example
///
/// ```no_run
/// # use freyacn::extensions::Shadow;
/// let shadow = Shadow::small();
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Shadow {
    // Adjust fields to match your shadow API.
    // For example, you might have:
    // offset_x: f32,
    // offset_y: f32,
    // blur: f32,
    // spread: f32,
    // color: Color,
}

impl Shadow {
    /// Create a small shadow.
    ///
    /// Equivalent to `shadow-sm` in Tailwind.
    pub fn small() -> Self {
        Self { /* ... */ }
    }

    /// Create a medium shadow.
    ///
    /// Equivalent to `shadow-md` in Tailwind.
    pub fn medium() -> Self {
        Self { /* ... */ }
    }

    /// Create a large shadow.
    ///
    /// Equivalent to `shadow-lg` in Tailwind.
    pub fn large() -> Self {
        Self { /* ... */ }
    }

    /// Create an extra‑large shadow.
    ///
    /// Equivalent to `shadow-xl` in Tailwind.
    pub fn extra_large() -> Self {
        Self { /* ... */ }
    }

    /// Create no shadow (no effect).
    ///
    /// Equivalent to `shadow-none` in Tailwind.
    pub fn none() -> Self {
        Self { /* ... */ }
    }
}

/// An extension trait that adds Tailwind‑inspired visual effects helpers.
///
/// This trait is implemented for any type that can have visual effects like
/// opacity and shadows (e.g., buttons, cards, containers, images). It provides
/// a comprehensive set of methods to control these effects using the Tailwind
/// naming convention.
///
/// # Required Methods
///
/// You must implement [`opacity`](EffectsExt::opacity) and
/// [`shadow`](EffectsExt::shadow) to apply effects to your component.
/// All other methods are provided by the trait.
///
/// # Examples
///
/// Basic usage:
///
/// ```no_run
/// # use freyacn::extensions::EffectsExt;
/// # use freyacn::components::Button;
/// let button = Button()
///     .opacity_50()      // 50% opacity
///     .shadow_md()       // medium shadow
///     .shadow_none();    // remove shadow
/// ```
///
/// Raw opacity values:
///
/// ```no_run
/// # use freyacn::extensions::EffectsExt;
/// # use freyacn::components::Button;
/// let button = Button()
///     .opacity(0.8)      // 80% opacity
///     .shadow_sm();      // small shadow
/// ```
///
/// # Implementing on your own type
///
/// ```no_run
/// # use freyacn::extensions::EffectsExt;
/// # use freyacn::extensions::Shadow;
/// struct MyWidget;
///
/// impl EffectsExt for MyWidget {
///     fn opacity(self, opacity: f32) -> Self {
///         self.with_opacity(opacity)
///     }
///     fn shadow(self, shadow: impl Into<Shadow>) -> Self {
///         self.with_shadow(shadow.into())
///     }
/// }
/// ```
///
/// # Notes
///
/// - Opacity values must be in the range `[0.0, 1.0]`.
/// - Shadows are defined by the `Shadow` type; you may need to adjust its
///   definition to match Freya’s shadow API.
/// - All methods return `Self` for method chaining.
/// - Methods are designed to be used inside a component’s `render` method.
pub trait EffectsExt: Sized {
    // ---- Core methods that must be implemented by the component ----

    /// Set the opacity of the component.
    ///
    /// This method must be implemented by your component. It should apply the
    /// opacity to the underlying container and return `self`.
    ///
    /// # Arguments
    ///
    /// * `opacity` – The opacity value, ranging from `0.0` (transparent) to
    ///   `1.0` (fully opaque).
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use freyacn::extensions::EffectsExt;
    /// # struct MyComponent;
    /// impl EffectsExt for MyComponent {
    ///     fn opacity(self, opacity: f32) -> Self {
    ///         self.with_opacity(opacity)
    ///     }
    ///     // ...
    /// }
    /// ```
    fn opacity(self, opacity: f32) -> Self;

    /// Set the shadow of the component.
    ///
    /// This method must be implemented by your component. It should apply the
    /// shadow to the underlying container and return `self`.
    ///
    /// # Arguments
    ///
    /// * `shadow` – The shadow effect. Can be anything that implements
    ///   `Into<Shadow>`, such as `Shadow` itself.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use freyacn::extensions::EffectsExt;
    /// # use freyacn::extensions::Shadow;
    /// # struct MyComponent;
    /// impl EffectsExt for MyComponent {
    ///     fn shadow(self, shadow: impl Into<Shadow>) -> Self {
    ///         self.with_shadow(shadow.into())
    ///     }
    ///     // ...
    /// }
    /// ```
    fn shadow(self, shadow: impl Into<Shadow>) -> Self;

    // ---- Opacity helpers ----
    // The following methods provide common opacity values.

    /// Set opacity to `0%` (completely transparent).
    ///
    /// Equivalent to `opacity-0` in Tailwind.
    ///
    /// # Example
    ///
    /// ```
    /// # use freyacn::extensions::EffectsExt;
    /// # use freyacn::components::Button;
    /// let button = Button().opacity_0();
    /// ```
    fn opacity_0(self) -> Self {
        self.opacity(0.0)
    }

    /// Set opacity to `25%`.
    ///
    /// Equivalent to `opacity-25` in Tailwind.
    ///
    /// # Example
    ///
    /// ```
    /// # use freyacn::extensions::EffectsExt;
    /// # use freyacn::components::Button;
    /// let button = Button().opacity_25();
    /// ```
    fn opacity_25(self) -> Self {
        self.opacity(0.25)
    }

    /// Set opacity to `50%`.
    ///
    /// Equivalent to `opacity-50` in Tailwind.
    ///
    /// # Example
    ///
    /// ```
    /// # use freyacn::extensions::EffectsExt;
    /// # use freyacn::components::Button;
    /// let button = Button().opacity_50();
    /// ```
    fn opacity_50(self) -> Self {
        self.opacity(0.5)
    }

    /// Set opacity to `75%`.
    ///
    /// Equivalent to `opacity-75` in Tailwind.
    ///
    /// # Example
    ///
    /// ```
    /// # use freyacn::extensions::EffectsExt;
    /// # use freyacn::components::Button;
    /// let button = Button().opacity_75();
    /// ```
    fn opacity_75(self) -> Self {
        self.opacity(0.75)
    }

    /// Set opacity to `100%` (fully opaque).
    ///
    /// Equivalent to `opacity-100` in Tailwind.
    ///
    /// # Example
    ///
    /// ```
    /// # use freyacn::extensions::EffectsExt;
    /// # use freyacn::components::Button;
    /// let button = Button().opacity_100();
    /// ```
    fn opacity_100(self) -> Self {
        self.opacity(1.0)
    }

    // ---- Shadow helpers ----
    // The following methods provide predefined shadow sizes.

    /// Apply a small shadow.
    ///
    /// Equivalent to `shadow-sm` in Tailwind.
    ///
    /// # Example
    ///
    /// ```
    /// # use freyacn::extensions::EffectsExt;
    /// # use freyacn::components::Button;
    /// let button = Button().shadow_sm();
    /// ```
    fn shadow_sm(self) -> Self {
        self.shadow(Shadow::small())
    }

    /// Apply a medium shadow.
    ///
    /// Equivalent to `shadow-md` in Tailwind.
    ///
    /// # Example
    ///
    /// ```
    /// # use freyacn::extensions::EffectsExt;
    /// # use freyacn::components::Button;
    /// let button = Button().shadow_md();
    /// ```
    fn shadow_md(self) -> Self {
        self.shadow(Shadow::medium())
    }

    /// Apply a large shadow.
    ///
    /// Equivalent to `shadow-lg` in Tailwind.
    ///
    /// # Example
    ///
    /// ```
    /// # use freyacn::extensions::EffectsExt;
    /// # use freyacn::components::Button;
    /// let button = Button().shadow_lg();
    /// ```
    fn shadow_lg(self) -> Self {
        self.shadow(Shadow::large())
    }

    /// Apply an extra‑large shadow.
    ///
    /// Equivalent to `shadow-xl` in Tailwind.
    ///
    /// # Example
    ///
    /// ```
    /// # use freyacn::extensions::EffectsExt;
    /// # use freyacn::components::Button;
    /// let button = Button().shadow_xl();
    /// ```
    fn shadow_xl(self) -> Self {
        self.shadow(Shadow::extra_large())
    }

    /// Remove the shadow (no shadow effect).
    ///
    /// Equivalent to `shadow-none` in Tailwind.
    ///
    /// # Example
    ///
    /// ```
    /// # use freyacn::extensions::EffectsExt;
    /// # use freyacn::components::Button;
    /// let button = Button().shadow_none();
    /// ```
    fn shadow_none(self) -> Self {
        self.shadow(Shadow::none())
    }
}
