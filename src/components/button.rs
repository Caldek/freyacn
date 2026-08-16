use crate::core::theme::Theme as CNTheme;
use freya::components::Button as ButtonPrimitive;
use freya::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ButtonVariant {
    Default,
    Destructive,
    Outline,
    Secondary,
    Ghost,
    Link,
}

// todo add radius metho using cornerradiusext trait
// todo add icon and change label to text
// todo make icon and text optional

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

#[derive(PartialEq)]
pub struct Button {
    variant: ButtonVariant,
    size: ButtonSize,
    label: String,
    on_click: Option<EventHandler<Event<PressEventData>>>,
}

impl Button {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            variant: ButtonVariant::Default,
            size: ButtonSize::Default,
            label: label.into(),
            on_click: None,
        }
    }

    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }

    pub fn on_click(mut self, callback: impl Into<EventHandler<Event<PressEventData>>>) -> Self {
        self.on_click = Some(callback.into());
        self
    }
}

impl Component for Button {
    fn render(&self) -> impl IntoElement {
        let theme = use_theme();
        let is_dark = theme.read().name == "dark";
        let cn_theme = if is_dark {
            CNTheme::dark()
        } else {
            CNTheme::light()
        };

        let mut button = match self.variant {
            ButtonVariant::Default => ButtonPrimitive::new()
                .background(cn_theme.colors.primary)
                .rounded_lg()
                .border_fill(cn_theme.colors.primary)
                .hover_background(cn_theme.colors.primary)
                .child(
                    label()
                        .text(self.label.clone())
                        .color(cn_theme.colors.primary_foreground),
                ),
            ButtonVariant::Destructive => ButtonPrimitive::new()
                .background(cn_theme.colors.destructive)
                .hover_background(cn_theme.colors.destructive)
                .border_fill(cn_theme.colors.destructive)
                .rounded_lg()
                .child(
                    label()
                        .text(self.label.clone())
                        .color(cn_theme.colors.destructive_foreground),
                ),
            ButtonVariant::Outline => ButtonPrimitive::new()
                .background(cn_theme.colors.background)
                .hover_background(cn_theme.colors.background)
                .rounded_lg()
                .child(
                    label()
                        .text(self.label.clone())
                        .color(cn_theme.colors.foreground),
                ),
            ButtonVariant::Secondary => ButtonPrimitive::new()
                .background(cn_theme.colors.secondary)
                .rounded_lg()
                .border_fill(cn_theme.colors.secondary)
                .hover_background(cn_theme.colors.secondary)
                .child(
                    label()
                        .text(self.label.clone())
                        .color(cn_theme.colors.secondary_foreground),
                ),
            ButtonVariant::Ghost => ButtonPrimitive::new()
                .background(cn_theme.colors.background)
                .rounded_lg()
                .border_fill(cn_theme.colors.background)
                .hover_background(cn_theme.colors.background)
                .child(
                    label()
                        .text(self.label.clone())
                        .color(cn_theme.colors.foreground),
                ),
            ButtonVariant::Link => ButtonPrimitive::new()
                .background(cn_theme.colors.background)
                .outline()
                .rounded_sm()
                .child(self.label.clone()),
        };

        if let Some(on_click) = self.on_click.clone() {
            button = button.on_press(on_click);
        }

        button
    }
}
