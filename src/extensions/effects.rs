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
//! # use freya::prelude::Shadow;
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
//! ## Shadow Values
//!
//! The predefined shadow helpers use the following values (x, y, blur, spread):
//!
//! - `shadow_sm`: `(0, 1, 2, 0)`  – small shadow
//! - `shadow_md`: `(0, 4, 6, -1)` – medium shadow
//! - `shadow_lg`: `(0, 10, 15, -3)` – large shadow
//! - `shadow_xl`: `(0, 20, 25, -5)` – extra‑large shadow
//! - `shadow_none`: no shadow
//!
//! These values are designed to match Tailwind's default shadows.
//!
//! ## Notes
//!
//! - Opacity values range from `0.0` (completely transparent) to `1.0` (fully opaque).
//! - Shadow helpers use Freya's built‑in [`Shadow`] type from `freya::prelude`.
//! - All methods return `Self` for method chaining.
//! - Methods are designed to be used inside a component’s `render` method.

use freya::prelude::Shadow;

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
/// # use freya::prelude::Shadow;
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
/// - Shadows are defined using Freya's built‑in `Shadow` type.
/// - All methods return `Self` for method chaining.
/// - Methods are designed to be used inside a component’s `render` method.
pub trait EffectsExt: Sized {
    // ---- Core methods that must be implemented by the component ----

    /// Set the opacity of the component.
    ///
    /// # Arguments
    ///
    /// * `opacity` – The opacity value, ranging from `0.0` (transparent) to
    ///   `1.0` (fully opaque).
    fn opacity(self, opacity: f32) -> Self;

    /// Set the shadow of the component.
    ///
    /// # Arguments
    ///
    /// * `shadow` – The shadow effect. Can be anything that implements
    ///   `Into<Shadow>`, such as `Shadow` itself or a tuple `(x, y, blur, spread, color)`.
    fn shadow(self, shadow: impl Into<Shadow>) -> Self;

    // ---- Opacity helpers ----
    // The following methods provide common opacity values.

    /// Set opacity to `0%` (completely transparent).
    ///
    /// Equivalent to `opacity-0` in Tailwind.
    fn opacity_0(self) -> Self {
        self.opacity(0.0)
    }

    /// Set opacity to `25%`.
    ///
    /// Equivalent to `opacity-25` in Tailwind.
    fn opacity_25(self) -> Self {
        self.opacity(0.25)
    }

    /// Set opacity to `50%`.
    ///
    /// Equivalent to `opacity-50` in Tailwind.
    fn opacity_50(self) -> Self {
        self.opacity(0.5)
    }

    /// Set opacity to `75%`.
    ///
    /// Equivalent to `opacity-75` in Tailwind.
    fn opacity_75(self) -> Self {
        self.opacity(0.75)
    }

    /// Set opacity to `100%` (fully opaque).
    ///
    /// Equivalent to `opacity-100` in Tailwind.
    fn opacity_100(self) -> Self {
        self.opacity(1.0)
    }

    // ---- Shadow helpers ----
    // The following methods provide predefined shadow sizes.

    /// Apply a small shadow (0, 1, 2, 0).
    ///
    /// Equivalent to `shadow-sm` in Tailwind.
    ///
    /// The shadow uses a black color with 30% opacity.
    fn shadow_sm(self) -> Self {
        self.shadow(
            Shadow::new()
                .x(0.0)
                .y(1.0)
                .blur(2.0)
                .spread(0.0)
                .color((0, 0, 0, 0.3)),
        )
    }

    /// Apply a medium shadow (0, 4, 6, -1).
    ///
    /// Equivalent to `shadow-md` in Tailwind.
    ///
    /// The shadow uses a black color with 15% opacity.
    fn shadow_md(self) -> Self {
        self.shadow(
            Shadow::new()
                .x(0.0)
                .y(4.0)
                .blur(6.0)
                .spread(-1.0)
                .color((0, 0, 0, 0.15)),
        )
    }

    /// Apply a large shadow (0, 10, 15, -3).
    ///
    /// Equivalent to `shadow-lg` in Tailwind.
    ///
    /// The shadow uses a black color with 15% opacity.
    fn shadow_lg(self) -> Self {
        self.shadow(
            Shadow::new()
                .x(0.0)
                .y(10.0)
                .blur(15.0)
                .spread(-3.0)
                .color((0, 0, 0, 0.15)),
        )
    }

    /// Apply an extra‑large shadow (0, 20, 25, -5).
    ///
    /// Equivalent to `shadow-xl` in Tailwind.
    ///
    /// The shadow uses a black color with 15% opacity.
    fn shadow_xl(self) -> Self {
        self.shadow(
            Shadow::new()
                .x(0.0)
                .y(20.0)
                .blur(25.0)
                .spread(-5.0)
                .color((0, 0, 0, 0.15)),
        )
    }

    /// Remove the shadow (no shadow effect).
    ///
    /// Equivalent to `shadow-none` in Tailwind.
    fn shadow_none(self) -> Self {
        // Create a shadow with zero offsets, blur, and spread, and fully transparent color.
        self.shadow(
            Shadow::new()
                .x(0.0)
                .y(0.0)
                .blur(0.0)
                .spread(0.0)
                .color((0, 0, 0, 0.0)),
        )
    }
}
