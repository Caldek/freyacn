use freya::prelude::*;
use freyacn::core::theme::Theme as CNTheme;

use freyacn::components::{Button, ButtonSize, ButtonVariant};

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)));
}

fn app() -> impl IntoElement {
    use_init_theme(dark_theme);
    let theme = use_theme();
    let is_dark = theme.read().name == "dark";
    let cn_theme = if is_dark {
        CNTheme::dark()
    } else {
        CNTheme::light()
    };

    rect()
        .width(Size::fill())
        .height(Size::fill())
        .background(cn_theme.colors.background)
        .padding(Gaps::new_all(32.0))
        .child(
            rect()
                .spacing(12.0)
                .child(label().font_size(24.0).text("Button"))
                .child(Button::new("Default"))
                .child(Button::new("Destructive").variant(ButtonVariant::Destructive))
                .child(Button::new("Outline").variant(ButtonVariant::Outline))
                .child(Button::new("Secondary").variant(ButtonVariant::Secondary))
                .child(Button::new("Ghost").variant(ButtonVariant::Ghost))
                .child(Button::new("Link").variant(ButtonVariant::Link))
                .child(Button::new("Small").size(ButtonSize::Sm))
                .child(Button::new("Large").size(ButtonSize::Lg)),
        )
}
