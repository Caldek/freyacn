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
    cursor_icon: CursorIcon,

    /// Explicit component background override.
    background: Option<Color>,
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
            corner_radius: 10.0, // default radius
            label: None,
            elements: Vec::new(),

            on_press: None,
            on_secondary_down: None,
            on_pointer_down: None,

            key: DiffKey::None,

            enabled: true,
            focusable: true,

            icon: None,
            cursor_icon: CursorIcon::default(),

            background: None,
        }
    }

    // ------------------------------------------------------------
    // Button configuration
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
    // Variants
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
    // Sizes
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

    /// Set an icon for the button. The icon can be any Element, typically a FreyaCN `Icon` component.
    pub fn icon(mut self, icon: impl Into<Element>) -> Self {
        self.icon = Some(icon.into());
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
// FreyaCN extension trait (without theme() – it's not usable)
// ------------------------------------------------------------

impl CNExt for CNButton {
    fn background(mut self, color: Color) -> Self {
        self.background = Some(color);
        self
    }
}

// ------------------------------------------------------------
// Component
// ------------------------------------------------------------

impl Component for CNButton {
    fn render(&self) -> impl IntoElement {
        // Global FreyaCN theme.
        let theme: CNTheme = use_consume();

        // Compute colors based on variant.
        let (bg, hover_bg, border_color, text_color) = match self.variant {
            ButtonVariant::Default => (
                theme.primary,
                theme.primary,
                theme.primary,
                theme.primary_foreground,
            ),
            ButtonVariant::Destructive => (
                theme.destructive,
                theme.destructive,
                theme.destructive,
                theme.destructive_foreground,
            ),
            ButtonVariant::Outline => (
                theme.background,
                theme.accent,
                theme.border,
                theme.foreground,
            ),
            ButtonVariant::Secondary => (
                theme.secondary,
                theme.secondary,
                theme.secondary,
                theme.secondary_foreground,
            ),
            ButtonVariant::Ghost => (
                theme.background,
                theme.accent,
                theme.background,
                theme.foreground,
            ),
            ButtonVariant::Link => (
                theme.background,
                theme.background,
                theme.background,
                theme.primary,
            ),
        };

        let mut button = ButtonPrimitive::new();

        let mut rectangle = rect()
            .horizontal()
            .cross_align(Alignment::Center)
            .spacing(5.0)
            .width(Size::auto())
            .height(Size::auto());

        // Base properties.
        button = button
            .corner_radius(self.corner_radius)
            .enabled(self.enabled)
            .focusable(self.focusable)
            .cursor_icon(self.cursor_icon);

        // Events.
        if let Some(on_press) = self.on_press.clone() {
            button = button.on_press(on_press);
        }
        if let Some(on_secondary_down) = self.on_secondary_down.clone() {
            button = button.on_secondary_down(on_secondary_down);
        }
        if let Some(on_pointer_down) = self.on_pointer_down.clone() {
            button = button.on_pointer_down(on_pointer_down);
        }

        // Apply variant colors.
        button = button
            .background(bg)
            .hover_background(hover_bg)
            .border_fill(border_color)
            .color(text_color);

        // Override background if explicitly set.
        if let Some(background) = self.background {
            button = button.background(background);
        }

        // For Link variant, remove borders and underline? The primitive might not support underline,
        // but we can set a transparent border and maybe add underline via a separate label style.
        if self.variant == ButtonVariant::Link {
            button = button.border_fill(theme.background).outline();
        }

        // Size and padding.
        button = match self.size {
            ButtonSize::Default => button
                .padding(Gaps::new(8., 10., 8., 10.))
                .width(Size::auto())
                .corner_radius(self.corner_radius),

            ButtonSize::Xs => button
                .padding(Gaps::new(6., 8., 6., 8.))
                .width(Size::auto())
                .corner_radius(self.corner_radius),

            ButtonSize::Sm => button
                .padding(Gaps::new(8., 10., 8., 10.))
                .width(Size::auto())
                .corner_radius(self.corner_radius),

            ButtonSize::Lg => button
                .padding(Gaps::new(8., 12., 8., 12.))
                .width(Size::auto())
                .corner_radius(self.corner_radius),

            ButtonSize::Icon => button
                .width(Size::px(36.))
                .height(Size::px(36.))
                .padding(Gaps::new_all(0.))
                .corner_radius(self.corner_radius),

            ButtonSize::IconXs => button
                .width(Size::px(24.))
                .height(Size::px(24.))
                .padding(Gaps::new_all(0.))
                .corner_radius(self.corner_radius),

            ButtonSize::IconSm => button
                .width(Size::px(32.))
                .height(Size::px(32.))
                .padding(Gaps::new_all(0.))
                .corner_radius(self.corner_radius),

            ButtonSize::IconLg => button
                .width(Size::px(40.))
                .height(Size::px(40.))
                .padding(Gaps::new_all(0.))
                .corner_radius(self.corner_radius),
        };

        // Determine font and icon sizes based on button size.
        let (font_size, icon_size) = match self.size {
            ButtonSize::Default => (14., 20.),
            ButtonSize::Xs => (12., 14.),
            ButtonSize::Sm => (12., 16.),
            ButtonSize::Lg => (16., 24.),
            ButtonSize::Icon => (14., 20.),
            ButtonSize::IconXs => (12., 14.),
            ButtonSize::IconSm => (12., 16.),
            ButtonSize::IconLg => (16., 24.),
        };

        // Render label if present.
        if let Some(label_text) = &self.label {
            let label = label()
                .text(label_text.clone())
                .color(text_color) // use the variant's text color
                .font_size(font_size)
                .font_weight(FontWeight::MEDIUM);

            // Add underline for Link variant.
            let label = if self.variant == ButtonVariant::Link {
                label.text_decoration(TextDecoration::Underline)
            } else {
                label
            };

            rectangle = rectangle.child(label);
        }

        // Render icon if present.
        if let Some(icon) = &self.icon {
            // Wrap icon in a container with the desired size.
            rectangle = rectangle.child(
                rect()
                    .height(Size::px(icon_size))
                    .width(Size::px(icon_size))
                    .child(icon.clone()),
            );
        }

        // User children.
        for element in &self.elements {
            rectangle = rectangle.child(element.clone());
        }

        button.child(rectangle)
    }
}

// ------------------------------------------------------------
// Component constructor
// ------------------------------------------------------------

#[allow(non_snake_case)]
pub fn Button() -> CNButton {
    CNButton::new()
}
