//! # FreyaCN Icon Component
//!
//! A flexible SVG icon component that integrates with the FreyaCN theming system.
//!
//! ## Overview
//!
//! This component renders an SVG icon from raw `Bytes` (typically obtained from
//! `freya::icons::lucide::*`). It automatically inherits the current theme’s foreground
//! colour, so icons blend seamlessly with your application’s design. You can override
//! the colour at any time with the `.color()` method.
//!
//! ## Features
//!
//! - **Theme‑aware**: Uses `theme.foreground` by default.
//! - **Rich sizing**: Chain `size()` or use semantic shortcuts (`size_xs`, `size_lg`, `size_3xl`)
//!   or direct pixel values (`size_16`, `size_24`, `size_48`).
//! - **Colour control**: Set a custom colour with `.color(Color)` or reset to theme default
//!   with `.color_theme()`.
//! - **Lightweight**: Built on Freya’s `SvgViewer`, which rasterises SVGs efficiently.
//! - **FreyaCN integration**: Implements `CNExt`, so you can also use `.color()` from that trait.
//!
//! ## Examples
//!
//! ### Basic usage with a Lucide icon
//! ```no_run
//! # use freyacn::icon::Icon;
//! # use freya::icons;
//! let heart = Icon(icons::lucide::heart())
//!     .size_24(); // uses theme.foreground automatically
//! ```
//!
//! ### Setting a custom colour
//! ```no_run
//! # use freyacn::icon::Icon;
//! # use freya::prelude::Color;
//! # use freya::icons;
//! let icon = Icon(icons::lucide::settings())
//!     .size_20()
//!     .color(Color::from_rgb(255, 100, 100));
//! ```
//!
//! ### Using the theme’s foreground explicitly
//! ```no_run
//! # use freyacn::icon::Icon;
//! # use freya::icons;
//! # let theme = use_cn_theme().read();
//! let icon = Icon(icons::lucide::user())
//!     .size_16()
//!     .color(theme.foreground);
//! ```
//!
//! ### Resetting to theme default after a custom colour
//! ```no_run
//! # use freyacn::icon::Icon;
//! # use freya::icons;
//! # let theme = use_cn_theme().read();
//! let icon = Icon(icons::lucide::home())
//!     .size_24()
//!     .color_theme(); // back to theme.foreground
//! ```

use freya::components::SvgViewer;
use freya::prelude::*;

use crate::core::CNExt;
use crate::core::theme::use_cn_theme;

/// A flexible SVG icon component.
///
/// This struct represents an SVG icon that can be rendered anywhere in your Freya
/// application. It is designed to be used as a building block in larger UI components
/// (e.g., buttons, navigation items) or as a standalone element.
///
/// By default, the icon uses the current theme’s `foreground` colour. You can override
/// this with `.color()` or reset to theme default with `.color_theme()`.
///
/// # Example
/// ```
/// # use freyacn::icon::Icon;
/// # use freya::icons;
/// # let theme = use_cn_theme().read();
/// let star = Icon(icons::lucide::star())
///     .size_32()
///     .color(theme.primary);
/// ```
#[derive(PartialEq, Clone)]
pub struct CNIcon {
    /// Raw SVG data (e.g., from `icons::lucide::heart()`).
    svg_data: Bytes,
    /// Width of the rendered SVG in pixels.
    width: f32,
    /// Height of the rendered SVG in pixels.
    height: f32,
    /// Optional custom colour. If `None`, the theme’s foreground is used.
    color: Option<Color>,
    /// Diff key for reconciliation (internal).
    key: DiffKey,
}

impl CNIcon {
    /// Creates a new icon from the given SVG data.
    ///
    /// The default size is 20×20 pixels (matching the typical `size-5` in shadcn).
    /// Use the various `size_*` methods to adjust.
    ///
    /// # Example
    /// ```
    /// # use freyacn::icon::Icon;
    /// # use freya::icons;
    /// let icon = Icon(icons::lucide::star());
    /// ```
    pub fn new(icon: Bytes) -> Self {
        Self {
            svg_data: icon,
            width: 20.0,
            height: 20.0,
            color: None, // will default to theme.foreground in render
            key: DiffKey::None,
        }
    }

    // ============================================================
    // Sizing
    // ============================================================

    /// Sets both width and height to the same value.
    ///
    /// # Example
    /// ```
    /// # use freyacn::icon::Icon;
    /// # use freya::icons;
    /// let icon = Icon(icons::lucide::check()).size(32.0);
    /// ```
    pub fn size(mut self, size: f32) -> Self {
        self.width = size;
        self.height = size;
        self
    }

    /// Sets the width only (height remains unchanged).
    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    /// Sets the height only (width remains unchanged).
    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    // ============================================================
    // Colour
    // ============================================================

    /// Sets the icon colour (applied to both fill and stroke).
    ///
    /// This overrides the default theme foreground colour. To revert to the
    /// theme default, call `.color_theme()`.
    ///
    /// # Example
    /// ```
    /// # use freyacn::icon::Icon;
    /// # use freya::prelude::Color;
    /// # use freya::icons;
    /// let icon = Icon(icons::lucide::heart())
    ///     .color(Color::from_rgb(255, 0, 0));
    /// ```
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Reverts the icon to use the theme’s foreground colour.
    ///
    /// This is a convenience method that clears any custom colour set by `.color()`.
    pub fn color_theme(mut self) -> Self {
        self.color = None;
        self
    }

    // ============================================================
    // Size presets (numeric scale)
    // ============================================================

    /// 12px – extra small
    pub fn size_1(self) -> Self {
        self.size(12.0)
    }
    /// 14px – slightly larger than xs
    pub fn size_2(self) -> Self {
        self.size(14.0)
    }
    /// 16px – small (typical for inline icons)
    pub fn size_3(self) -> Self {
        self.size(16.0)
    }
    /// 18px – between small and medium
    pub fn size_4(self) -> Self {
        self.size(18.0)
    }
    /// 20px – medium (default)
    pub fn size_5(self) -> Self {
        self.size(20.0)
    }
    /// 24px – large (common for buttons)
    pub fn size_6(self) -> Self {
        self.size(24.0)
    }
    /// 28px – extra large
    pub fn size_7(self) -> Self {
        self.size(28.0)
    }
    /// 32px – 2xl (often used for feature icons)
    pub fn size_8(self) -> Self {
        self.size(32.0)
    }
    /// 36px – 3xl
    pub fn size_9(self) -> Self {
        self.size(36.0)
    }

    // ============================================================
    // Semantic size presets (Tailwind‑inspired)
    // ============================================================

    /// Alias for `size_1()` – 12px.
    pub fn size_xs(self) -> Self {
        self.size_1()
    }
    /// Alias for `size_3()` – 16px.
    pub fn size_sm(self) -> Self {
        self.size_3()
    }
    /// Alias for `size_5()` – 20px.
    pub fn size_md(self) -> Self {
        self.size_5()
    }
    /// Alias for `size_6()` – 24px.
    pub fn size_lg(self) -> Self {
        self.size_6()
    }
    /// Alias for `size_8()` – 32px.
    pub fn size_xl(self) -> Self {
        self.size_8()
    }
    /// 40px – 2xl
    pub fn size_2xl(self) -> Self {
        self.size(40.0)
    }
    /// 48px – 3xl
    pub fn size_3xl(self) -> Self {
        self.size(48.0)
    }

    // ============================================================
    // Direct pixel presets (common values)
    // ============================================================

    /// 12px
    pub fn size_12(self) -> Self {
        self.size(12.0)
    }
    /// 14px
    pub fn size_14(self) -> Self {
        self.size(14.0)
    }
    /// 16px
    pub fn size_16(self) -> Self {
        self.size(16.0)
    }
    /// 18px
    pub fn size_18(self) -> Self {
        self.size(18.0)
    }
    /// 20px
    pub fn size_20(self) -> Self {
        self.size(20.0)
    }
    /// 24px
    pub fn size_24(self) -> Self {
        self.size(24.0)
    }
    /// 28px
    pub fn size_28(self) -> Self {
        self.size(28.0)
    }
    /// 32px
    pub fn size_32(self) -> Self {
        self.size(32.0)
    }
    /// 36px
    pub fn size_36(self) -> Self {
        self.size(36.0)
    }
    /// 40px
    pub fn size_40(self) -> Self {
        self.size(40.0)
    }
    /// 48px
    pub fn size_48(self) -> Self {
        self.size(48.0)
    }
    /// 64px
    pub fn size_64(self) -> Self {
        self.size(64.0)
    }
    /// 96px
    pub fn size_96(self) -> Self {
        self.size(96.0)
    }
}

// ------------------------------------------------------------
// Internal key handling
// ------------------------------------------------------------

impl KeyExt for CNIcon {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

// ------------------------------------------------------------
// Component rendering
// ------------------------------------------------------------

impl Component for CNIcon {
    fn render(&self) -> impl IntoElement {
        // Access the theme via the FreyaCN hook.
        let theme = use_cn_theme().read();

        // Determine the colour: if explicitly set, use it; otherwise use theme.foreground.
        let color = self.color.unwrap_or(theme.foreground);

        // Build the SvgViewer with the icon data, dimensions, and colour.
        SvgViewer::new(self.svg_data.clone())
            .width(Size::px(self.width))
            .height(Size::px(self.height))
            .color(color)
    }
}

// ------------------------------------------------------------
// Constructor
// ------------------------------------------------------------

/// Constructor for the Icon component.
///
/// This is the primary way to create an icon.
///
/// # Example
/// ```
/// use freya::icons;
/// use freyacn::icon::Icon;
///
/// let heart = Icon(icons::lucide::heart())
///     .size_24()
///     .color((255, 100, 100));
/// ```
#[allow(non_snake_case)]
pub fn Icon(icon: Bytes) -> CNIcon {
    CNIcon::new(icon)
}

// ------------------------------------------------------------
// CNExt trait implementation
// ------------------------------------------------------------

/// The `CNExt` trait provides `.background()` and `.color()` methods.
/// For icons, `.background()` is a no‑op (icons don’t have backgrounds),
/// while `.color()` forwards to `CNIcon::color()`.
impl CNExt for CNIcon {
    /// Background is ignored for icons (no effect).
    fn background(self, _color: Color) -> Self {
        self
    }

    /// Sets the icon colour (same as `.color()`).
    fn color(self, color: Color) -> Self {
        self.color(color)
    }
}
