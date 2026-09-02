use crate::core::CNExt;
use crate::core::theme::Theme as CNTheme;
use freya::prelude::{Button as ButtonPrimitive, *};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ButtonVariant {
    Default,
    Destructive,
    Outline,
    Secondary,
    Ghost,
    Link,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ButtonSize {
    Default,
    Xs,
    Sm,
    Lg,
    Icon,
    IconXs,
    IconSm,
    IconLg,
}

#[derive(PartialEq, Clone)]
pub struct CNButton {
    variant: ButtonVariant,
    size: ButtonSize,
    corner_radius: f32,
    label: Option<String>,
    elements: Vec<Element>,

    on_press: Option<EventHandler<Event<PressEventData>>>,
    on_secondary_down: Option<EventHandler<Event<PressEventData>>>,
    on_pointer_down: Option<EventHandler<Event<PointerEventData>>>,

    key: DiffKey,
    enabled: bool,
    focusable: bool,

    icon: Option<Element>,
    icon_position: IconPosition,
    cursor_icon: CursorIcon,

    /// Explicit component background override.
    background: Option<Color>,
    /// Explicit text color override.
    text_color: Option<Color>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IconPosition {
    Left,
    Right,
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

            icon: None,
            icon_position: IconPosition::Left,
            cursor_icon: CursorIcon::default(),

            background: None,
            text_color: None,
        }
    }

    // ------------------------------------------------------------
    // Configuration
    // ------------------------------------------------------------

    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }

    pub fn enabled(mut self, enabled: impl Into<bool>) -> Self {
        self.enabled = enabled.into();
        self
    }

    pub fn focusable(mut self, focusable: impl Into<bool>) -> Self {
        self.focusable = focusable.into();
        self
    }

    pub fn on_press(mut self, on_press: impl Into<EventHandler<Event<PressEventData>>>) -> Self {
        self.on_press = Some(on_press.into());
        self
    }

    pub fn on_secondary_down(
        mut self,
        on_secondary_down: impl Into<EventHandler<Event<PressEventData>>>,
    ) -> Self {
        self.on_secondary_down = Some(on_secondary_down.into());
        self
    }

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

    pub fn default(self) -> Self {
        self.variant(ButtonVariant::Default)
    }
    pub fn destructive(self) -> Self {
        self.variant(ButtonVariant::Destructive)
    }
    pub fn outline(self) -> Self {
        self.variant(ButtonVariant::Outline)
    }
    pub fn secondary(self) -> Self {
        self.variant(ButtonVariant::Secondary)
    }
    pub fn ghost(self) -> Self {
        self.variant(ButtonVariant::Ghost)
    }
    pub fn link(self) -> Self {
        self.variant(ButtonVariant::Link)
    }

    // ------------------------------------------------------------
    // Size shortcuts
    // ------------------------------------------------------------

    pub fn size_default(self) -> Self {
        self.size(ButtonSize::Default)
    }
    pub fn size_xs(self) -> Self {
        self.size(ButtonSize::Xs)
    }
    pub fn size_sm(self) -> Self {
        self.size(ButtonSize::Sm)
    }
    pub fn size_lg(self) -> Self {
        self.size(ButtonSize::Lg)
    }
    pub fn size_icon(self) -> Self {
        self.size(ButtonSize::Icon)
    }
    pub fn size_icon_xs(self) -> Self {
        self.size(ButtonSize::IconXs)
    }
    pub fn size_icon_sm(self) -> Self {
        self.size(ButtonSize::IconSm)
    }
    pub fn size_icon_lg(self) -> Self {
        self.size(ButtonSize::IconLg)
    }

    // ------------------------------------------------------------
    // Appearance
    // ------------------------------------------------------------

    pub fn cursor_icon(mut self, cursor_icon: impl Into<CursorIcon>) -> Self {
        self.cursor_icon = cursor_icon.into();
        self
    }

    pub fn corner_radius(mut self, corner_radius: f32) -> Self {
        self.corner_radius = corner_radius;
        self
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set an icon for the button.
    /// Accepts any `Element` – typically a `Icon` component:
    /// ```
    /// Button::new().icon(Icon(icons::lucide::heart()).size_24())
    /// ```
    pub fn icon(mut self, icon: impl Into<Element>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn icon_left(mut self) -> Self {
        self.icon_position = IconPosition::Left;
        self
    }

    pub fn icon_right(mut self) -> Self {
        self.icon_position = IconPosition::Right;
        self
    }
}

// ------------------------------------------------------------
// Freya corner radius integration
// ------------------------------------------------------------

impl CornerRadiusExt for CNButton {
    fn with_corner_radius(self, corner_radius: f32) -> Self {
        self.corner_radius(corner_radius)
    }
}

// ------------------------------------------------------------
// CNExt implementation – background and color overrides
// ------------------------------------------------------------

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
// Helpers – using Freya 0.4.1 API
// ------------------------------------------------------------

/// Create a color with a given alpha channel (0.0 – 1.0)
fn color_with_alpha(color: Color, alpha: f32) -> Color {
    let r = color.r();
    let g = color.g();
    let b = color.b();
    let a = (alpha * 255.0) as u8;
    Color::from_argb(a, r, g, b)
}

/// Blend two colors with a ratio (0.0 = base, 1.0 = blend)
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
// Component
// ------------------------------------------------------------

impl Component for CNButton {
    fn render(&self) -> impl IntoElement {
        let theme: CNTheme = use_consume();

        // --- Compute variant colors (based on shadcn design) ---
        let (bg, hover_bg, text_color, border_color) = match self.variant {
            ButtonVariant::Default => {
                let bg = theme.primary;
                let hover = color_with_alpha(bg, 0.8);
                (bg, hover, theme.primary_foreground, Color::TRANSPARENT)
            }
            ButtonVariant::Destructive => {
                let bg = theme.destructive; // bg-destructive
                let hover = color_with_alpha(theme.destructive, 0.8); // hover:bg-destructive/80
                (bg, hover, theme.destructive_foreground, Color::TRANSPARENT) // text-destructive
            }
            ButtonVariant::Outline => (
                theme.background,
                theme.muted,
                theme.foreground,
                theme.border,
            ),
            ButtonVariant::Secondary => {
                let bg = theme.secondary;
                let hover = blend_colors(bg, theme.foreground, 0.05);
                (bg, hover, theme.secondary_foreground, Color::TRANSPARENT)
            }
            ButtonVariant::Ghost => (
                Color::TRANSPARENT,
                theme.muted,
                theme.foreground,
                Color::TRANSPARENT,
            ),
            ButtonVariant::Link => (
                Color::TRANSPARENT,
                Color::TRANSPARENT,
                theme.primary,
                Color::TRANSPARENT,
            ),
        };

        // Override background and text color if explicitly set
        let final_bg = self.background.unwrap_or(bg);
        let final_text_color = self.text_color.unwrap_or(text_color);
        let final_hover_bg = if self.background.is_some() {
            hover_bg
        } else {
            hover_bg
        };

        // --- Size metrics (based on shadcn design) ---
        let (height, padding_h, gap, font_size, icon_size, radius) = match self.size {
            ButtonSize::Default => (32.0, 10.0, 6.0, 14.0, 16.0, self.corner_radius),
            ButtonSize::Xs => (24.0, 8.0, 4.0, 12.0, 12.0, self.corner_radius.min(10.0)),
            ButtonSize::Sm => (28.0, 10.0, 4.0, 12.8, 14.0, self.corner_radius.min(12.0)),
            ButtonSize::Lg => (36.0, 10.0, 6.0, 14.0, 16.0, self.corner_radius),
            ButtonSize::Icon => (32.0, 0.0, 0.0, 0.0, 20.0, self.corner_radius),
            ButtonSize::IconXs => (24.0, 0.0, 0.0, 0.0, 14.0, self.corner_radius.min(10.0)),
            ButtonSize::IconSm => (28.0, 0.0, 0.0, 0.0, 16.0, self.corner_radius.min(12.0)),
            ButtonSize::IconLg => (36.0, 0.0, 0.0, 0.0, 24.0, self.corner_radius),
        };

        // --- Build Freya Button primitive ---
        let mut button = ButtonPrimitive::new()
            .enabled(self.enabled)
            .focusable(self.focusable)
            .cursor_icon(self.cursor_icon)
            .corner_radius(radius)
            .background(final_bg)
            .hover_background(final_hover_bg)
            .border_fill(border_color)
            .color(final_text_color)
            .padding(Gaps::new(0.0, padding_h, 0.0, padding_h))
            .height(Size::px(height))
            .width(Size::auto());

        // For Link variant, ensure border is transparent
        if self.variant == ButtonVariant::Link {
            button = button.border_fill(Color::TRANSPARENT);
        }

        // --- Build content container ---
        let mut content = rect()
            .horizontal()
            .cross_align(Alignment::Center)
            .spacing(gap)
            .width(Size::auto())
            .height(Size::auto());

        // Icon element – wrap in a container with fixed size to enforce button's icon size
        let icon_element = self.icon.as_ref().map(|icon| {
            rect()
                .height(Size::px(icon_size))
                .width(Size::px(icon_size))
                .child(icon.clone())
        });

        // Label element
        let label_element = self.label.as_ref().map(|text| {
            let mut label = label()
                .text(text.clone())
                .color(final_text_color)
                .font_size(font_size)
                .font_weight(FontWeight::MEDIUM);
            if self.variant == ButtonVariant::Link {
                label = label.text_decoration(TextDecoration::Underline);
            }
            label
        });

        // Add children in correct order (icon left or right)
        if self.icon_position == IconPosition::Left {
            if let Some(el) = icon_element {
                content = content.child(el);
            }
            if let Some(el) = label_element {
                content = content.child(el);
            }
        } else {
            if let Some(el) = label_element {
                content = content.child(el);
            }
            if let Some(el) = icon_element {
                content = content.child(el);
            }
        }

        // Add user children
        for element in &self.elements {
            content = content.child(element.clone());
        }

        button.child(content)
    }
}

// ------------------------------------------------------------
// Component constructor
// ------------------------------------------------------------

#[allow(non_snake_case)]
pub fn Button() -> CNButton {
    CNButton::new()
}
