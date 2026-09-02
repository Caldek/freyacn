//! A fully-featured button component inspired by shadcn/ui.
//!
//! The button supports multiple variants (default, destructive, outline, secondary, ghost, link)
//! and sizes (default, xs, sm, lg, icon, icon-xs, icon-sm, icon-lg). It integrates with the
//! FreyaCN theming system and uses the `CNExt` trait for background and color overrides.
//!
//! # Example
//! ```
//! use freyacn::components::{Button, ButtonVariant, ButtonSize};
//! use freyacn::theme::use_cn_theme;
//!
//! fn MyButton() -> impl IntoElement {
//!     let theme = *use_cn_theme();  // Deref to get the Theme
//!     // ...
//! }
//! ```

use crate::core::CNExt;
use crate::core::icon_context::IconColorContext;
use crate::core::theme::use_cn_theme;
use freya::prelude::{Button as ButtonPrimitive, *};

/// Available visual variants for the button, matching shadcn/ui styles.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ButtonVariant {
    /// The primary action button (uses `theme.primary` background).
    Default,
    /// A destructive action button (uses `theme.destructive` with opacity).
    Destructive,
    /// An outlined button with a border and transparent background.
    Outline,
    /// A secondary action button (uses `theme.secondary` background).
    Secondary,
    /// A ghost button with no background except on hover.
    Ghost,
    /// A text-only link button with underline on hover.
    Link,
}

/// Available size presets for the button.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ButtonSize {
    /// Standard size (32px height, 10px horizontal padding).
    Default,
    /// Extra small (24px height, 8px horizontal padding).
    Xs,
    /// Small (28px height, 10px horizontal padding).
    Sm,
    /// Large (36px height, 10px horizontal padding).
    Lg,
    /// Icon‑only square button (32×32, 10px padding).
    Icon,
    /// Extra‑small icon‑only (24×24, 8px padding).
    IconXs,
    /// Small icon‑only (28×28, 10px padding).
    IconSm,
    /// Large icon‑only (36×36, 10px padding).
    IconLg,
}

/// A fully configurable button component.
///
/// This struct implements the builder pattern; all configuration methods consume
/// `self` and return a new instance, so you can chain them.
#[derive(PartialEq, Clone)]
pub struct CNButton {
    /// The visual variant of the button.
    variant: ButtonVariant,
    /// The size preset.
    size: ButtonSize,
    /// Corner radius in pixels (default 8.0).
    corner_radius: f32,
    /// Optional text label.
    label: Option<String>,
    /// Child elements (can be added via `.child()` or `.children()`).
    elements: Vec<Element>,

    // Event handlers
    on_press: Option<EventHandler<Event<PressEventData>>>,
    on_secondary_down: Option<EventHandler<Event<PressEventData>>>,
    on_pointer_down: Option<EventHandler<Event<PointerEventData>>>,

    /// Diff key for reconciliation (internal).
    key: DiffKey,
    /// Whether the button is interactive (default true).
    enabled: bool,
    /// Whether the button can receive focus (default true).
    focusable: bool,

    /// Cursor icon shown on hover (default default cursor).
    cursor_icon: CursorIcon,

    /// Override for the button background color.
    background: Option<Color>,
    /// Override for the button text color.
    text_color: Option<Color>,
}

impl Default for CNButton {
    fn default() -> Self {
        Self::new()
    }
}

impl ChildrenExt for CNButton {
    fn get_children(&mut self) -> &mut Vec<Element> {
        &mut self.elements
    }
}

impl KeyExt for CNButton {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl CNButton {
    /// Creates a new button with default settings.
    ///
    /// # Defaults
    /// - Variant: `ButtonVariant::Default`
    /// - Size: `ButtonSize::Default`
    /// - Corner radius: 8.0
    /// - Enabled: true
    /// - Focusable: true
    /// - No label, no children, no events.
    pub fn new() -> Self {
        Self {
            variant: ButtonVariant::Default,
            size: ButtonSize::Default,
            corner_radius: 8.0,
            label: None,
            elements: Vec::new(),
            on_press: None,
            on_secondary_down: None,
            on_pointer_down: None,
            key: DiffKey::None,
            enabled: true,
            focusable: true,
            cursor_icon: CursorIcon::default(),
            background: None,
            text_color: None,
        }
    }

    // ------------------------------------------------------------
    // Configuration methods
    // ------------------------------------------------------------

    /// Sets the visual variant of the button.
    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Sets the size preset.
    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }

    /// Enables or disables the button (interactivity).
    pub fn enabled(mut self, enabled: impl Into<bool>) -> Self {
        self.enabled = enabled.into();
        self
    }

    /// Makes the button focusable or not via keyboard navigation.
    pub fn focusable(mut self, focusable: impl Into<bool>) -> Self {
        self.focusable = focusable.into();
        self
    }

    /// Sets a handler for the primary press event.
    pub fn on_press(mut self, on_press: impl Into<EventHandler<Event<PressEventData>>>) -> Self {
        self.on_press = Some(on_press.into());
        self
    }

    /// Sets a handler for the secondary (right) press event.
    pub fn on_secondary_down(
        mut self,
        on_secondary_down: impl Into<EventHandler<Event<PressEventData>>>,
    ) -> Self {
        self.on_secondary_down = Some(on_secondary_down.into());
        self
    }

    /// Sets a handler for the pointer down event (advanced).
    pub fn on_pointer_down(
        mut self,
        on_pointer_down: impl Into<EventHandler<Event<PointerEventData>>>,
    ) -> Self {
        self.on_pointer_down = Some(on_pointer_down.into());
        self
    }

    // ------------------------------------------------------------
    // Variant shortcuts
    // ------------------------------------------------------------

    /// Shortcut for `variant(ButtonVariant::Default)`.
    pub fn default(self) -> Self {
        self.variant(ButtonVariant::Default)
    }
    /// Shortcut for `variant(ButtonVariant::Destructive)`.
    pub fn destructive(self) -> Self {
        self.variant(ButtonVariant::Destructive)
    }
    /// Shortcut for `variant(ButtonVariant::Outline)`.
    pub fn outline(self) -> Self {
        self.variant(ButtonVariant::Outline)
    }
    /// Shortcut for `variant(ButtonVariant::Secondary)`.
    pub fn secondary(self) -> Self {
        self.variant(ButtonVariant::Secondary)
    }
    /// Shortcut for `variant(ButtonVariant::Ghost)`.
    pub fn ghost(self) -> Self {
        self.variant(ButtonVariant::Ghost)
    }
    /// Shortcut for `variant(ButtonVariant::Link)`.
    pub fn link(self) -> Self {
        self.variant(ButtonVariant::Link)
    }

    // ------------------------------------------------------------
    // Size shortcuts
    // ------------------------------------------------------------

    /// Shortcut for `size(ButtonSize::Default)`.
    pub fn size_default(self) -> Self {
        self.size(ButtonSize::Default)
    }
    /// Shortcut for `size(ButtonSize::Xs)`.
    pub fn size_xs(self) -> Self {
        self.size(ButtonSize::Xs)
    }
    /// Shortcut for `size(ButtonSize::Sm)`.
    pub fn size_sm(self) -> Self {
        self.size(ButtonSize::Sm)
    }
    /// Shortcut for `size(ButtonSize::Lg)`.
    pub fn size_lg(self) -> Self {
        self.size(ButtonSize::Lg)
    }
    /// Shortcut for `size(ButtonSize::Icon)`.
    pub fn size_icon(self) -> Self {
        self.size(ButtonSize::Icon)
    }
    /// Shortcut for `size(ButtonSize::IconXs)`.
    pub fn size_icon_xs(self) -> Self {
        self.size(ButtonSize::IconXs)
    }
    /// Shortcut for `size(ButtonSize::IconSm)`.
    pub fn size_icon_sm(self) -> Self {
        self.size(ButtonSize::IconSm)
    }
    /// Shortcut for `size(ButtonSize::IconLg)`.
    pub fn size_icon_lg(self) -> Self {
        self.size(ButtonSize::IconLg)
    }

    // ------------------------------------------------------------
    // Appearance
    // ------------------------------------------------------------

    /// Sets the cursor icon shown when hovering over the button.
    pub fn cursor_icon(mut self, cursor_icon: impl Into<CursorIcon>) -> Self {
        self.cursor_icon = cursor_icon.into();
        self
    }

    /// Sets a custom corner radius (overrides the size‑default radius).
    pub fn corner_radius(mut self, corner_radius: f32) -> Self {
        self.corner_radius = corner_radius;
        self
    }

    /// Sets the button’s text label.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }
}

// ------------------------------------------------------------
// Trait implementations for Freya integration
// ------------------------------------------------------------

impl CornerRadiusExt for CNButton {
    fn with_corner_radius(self, corner_radius: f32) -> Self {
        self.corner_radius(corner_radius)
    }
}

// CNExt gives us .background() and .color() overrides.
impl CNExt for CNButton {
    fn background(mut self, color: Color) -> Self {
        self.background = Some(color);
        self
    }

    fn color(mut self, color: Color) -> Self {
        self.text_color = Some(color);
        self
    }
}

// ------------------------------------------------------------
// Color helpers (using Freya 0.4.1 APIs)
// ------------------------------------------------------------

/// Returns a new color with the given alpha channel (0.0 .. 1.0).
fn color_with_alpha(color: Color, alpha: f32) -> Color {
    let r = color.r();
    let g = color.g();
    let b = color.b();
    let a = (alpha * 255.0) as u8;
    Color::from_argb(a, r, g, b)
}

/// Blends two colors by the given ratio (0.0 = base, 1.0 = blend).
fn blend_colors(base: Color, blend: Color, ratio: f32) -> Color {
    let r1 = base.r() as f32;
    let g1 = base.g() as f32;
    let b1 = base.b() as f32;
    let a1 = base.a() as f32 / 255.0;
    let r2 = blend.r() as f32;
    let g2 = blend.g() as f32;
    let b2 = blend.b() as f32;
    let a2 = blend.a() as f32 / 255.0;
    let r = r1 + (r2 - r1) * ratio;
    let g = g1 + (g2 - g1) * ratio;
    let b = b1 + (b2 - b1) * ratio;
    let a = a1 + (a2 - a1) * ratio;
    Color::from_argb((a * 255.0) as u8, r as u8, g as u8, b as u8)
}

// ------------------------------------------------------------
// Component implementation
// ------------------------------------------------------------

impl Component for CNButton {
    fn render(&self) -> impl IntoElement {
        // Access the global theme by dereferencing the State<Theme>.
        let theme = use_cn_theme().read();

        // Compute colors based on the variant (matching shadcn/ui).
        let (bg, hover_bg, text_color, border_color) = match self.variant {
            ButtonVariant::Default => {
                let bg = theme.primary;
                let hover = color_with_alpha(bg, 0.8); // hover:bg-primary/80
                (bg, hover, theme.primary_foreground, Color::TRANSPARENT)
            }
            ButtonVariant::Destructive => {
                let destructive = theme.destructive;
                // destructive variant uses destructive/10 bg, destructive/20 hover, text-destructive
                let bg = color_with_alpha(destructive, 0.1);
                let hover = color_with_alpha(destructive, 0.2);
                (bg, hover, destructive, Color::TRANSPARENT)
            }
            ButtonVariant::Outline => {
                // border-border, bg-background, hover:bg-muted, text-foreground
                (
                    theme.background,
                    theme.muted,
                    theme.foreground,
                    theme.border,
                )
            }
            ButtonVariant::Secondary => {
                let bg = theme.secondary;
                // hover: mix secondary with foreground (5%)
                let hover = blend_colors(bg, theme.foreground, 0.05);
                (bg, hover, theme.secondary_foreground, Color::TRANSPARENT)
            }
            ButtonVariant::Ghost => {
                // transparent bg, hover:bg-muted, text-foreground
                (
                    Color::TRANSPARENT,
                    theme.muted,
                    theme.foreground,
                    Color::TRANSPARENT,
                )
            }
            ButtonVariant::Link => {
                // text-primary, transparent bg, underline on hover via label style
                (
                    Color::TRANSPARENT,
                    Color::TRANSPARENT,
                    theme.primary,
                    Color::TRANSPARENT,
                )
            }
        };

        // Apply any user‑provided overrides.
        let final_bg = self.background.unwrap_or(bg);
        let final_text_color = self.text_color.unwrap_or(text_color);

        // Provide the icon colour context so children (icons) can inherit this colour.
        provide_context(IconColorContext(final_text_color));
        let final_hover_bg = if self.background.is_some() {
            hover_bg
        } else {
            hover_bg
        };

        // Determine size metrics: height, horizontal padding, gap, font size, and radius.
        let (height, pad_h, gap, font_size, radius) = match self.size {
            ButtonSize::Default => (32.0, 10.0, 6.0, 14.0, self.corner_radius),
            ButtonSize::Xs => (24.0, 8.0, 4.0, 12.0, self.corner_radius.min(10.0)),
            ButtonSize::Sm => (28.0, 10.0, 4.0, 12.8, self.corner_radius.min(12.0)),
            ButtonSize::Lg => (36.0, 10.0, 6.0, 14.0, self.corner_radius),
            ButtonSize::Icon => (32.0, 10.0, 0.0, 0.0, self.corner_radius),
            ButtonSize::IconXs => (24.0, 8.0, 0.0, 0.0, self.corner_radius.min(10.0)),
            ButtonSize::IconSm => (28.0, 10.0, 0.0, 0.0, self.corner_radius.min(12.0)),
            ButtonSize::IconLg => (36.0, 10.0, 0.0, 0.0, self.corner_radius),
        };

        // Whether the size is icon‑only (used to make the button square).
        let is_icon_only = matches!(
            self.size,
            ButtonSize::Icon | ButtonSize::IconXs | ButtonSize::IconSm | ButtonSize::IconLg
        );

        // Padding: for icon‑only, equal padding on all sides; otherwise top/bottom 0.
        let top_pad = if is_icon_only { pad_h } else { 0.0 };
        let bottom_pad = top_pad;
        let left_pad = pad_h;
        let right_pad = pad_h;

        // Build the underlying Freya Button primitive.
        let mut button = ButtonPrimitive::new()
            .enabled(self.enabled)
            .focusable(self.focusable)
            .cursor_icon(self.cursor_icon)
            .corner_radius(radius)
            .background(final_bg)
            .hover_background(final_hover_bg)
            .border_fill(border_color)
            .color(final_text_color)
            .padding(Gaps::new(top_pad, right_pad, bottom_pad, left_pad))
            .height(Size::px(height))
            .width(if is_icon_only {
                Size::px(height) // square
            } else {
                Size::auto()
            });

        // Link variant: remove border.
        if self.variant == ButtonVariant::Link {
            button = button.border_fill(Color::TRANSPARENT);
        }

        // Content container: horizontal flex, centered vertically and (when icon‑only) horizontally.
        let mut content = rect()
            .horizontal()
            .cross_align(Alignment::Center)
            .main_align(if is_icon_only || self.label.is_none() {
                Alignment::Center
            } else {
                Alignment::Start
            })
            .spacing(gap)
            .width(Size::auto())
            .height(Size::auto());

        // Render the label if present.
        if let Some(text) = &self.label {
            let mut label = label()
                .text(text.clone())
                .color(final_text_color)
                .font_size(font_size)
                .font_weight(FontWeight::MEDIUM);
            if self.variant == ButtonVariant::Link {
                label = label.text_decoration(TextDecoration::Underline);
            }
            content = content.child(label);
        }

        // Add any user‑provided children.
        for element in &self.elements {
            content = content.child(element.clone());
        }

        button.child(content)
    }
}

/// Constructor function for the button, following Freya’s component naming convention.
///
/// # Example
/// ```
/// # use freyacn::button::Button;
/// let my_button = Button()
///     .label("Submit")
///     .on_press(|_| println!("Submitted!"));
/// ```
#[allow(non_snake_case)]
pub fn Button() -> CNButton {
    CNButton::new()
}
