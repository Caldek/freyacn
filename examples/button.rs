//! Simple button showcase with the FreyaCN theming system.

use freya::prelude::*;
use freya_icons::lucide::{arrow_right, heart, settings};

use freyacn::components::button::Button;
use freyacn::components::icon::Icon;
use freyacn::core::CNExt;
use freyacn::core::theme::{Theme, use_init_cn_theme};

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)));
}

fn app() -> impl IntoElement {
    let theme = Theme::neutral(true, "neutral");
    use_init_cn_theme(theme);

    rect()
        .width(Size::fill())
        .height(Size::fill())
        .background(theme.colors.black)
        .padding(Gaps::new_all(32.0))
        .child(
            rect()
                .vertical()
                .spacing(16.0)
                // Header
                .child(label().font_size(24.0).text("Button Showcase"))
                // Variants
                .child(label().font_size(18.0).text("Variants"))
                .child(Button().label("Default"))
                .child(Button().label("Destructive").destructive())
                .child(Button().label("Outline").outline())
                .child(Button().label("Secondary").secondary())
                .child(Button().label("Ghost").ghost())
                .child(Button().label("Link").link())
                // Sizes
                .child(label().font_size(18.0).text("Sizes"))
                .child(Button().label("Default"))
                .child(Button().label("XS").size_xs())
                .child(Button().label("SM").size_sm())
                .child(Button().label("LG").size_lg())
                // Icons
                .child(label().font_size(18.0).text("Icons"))
                .child(Button().size_icon().child(Icon(heart())))
                .child(Button().label("Like").child(Icon(heart())))
                .child(Button().label("Settings").child(Icon(settings())))
                .child(Button().label("Next").child(Icon(arrow_right())))
                // States
                .child(label().font_size(18.0).text("States"))
                .child(Button().label("Disabled").enabled(false))
                .child(Button().label("Focusable").focusable(true))
                .child(
                    Button()
                        .label("Custom")
                        .background(theme.colors.blue_500)
                        .color(theme.colors.white),
                ),
        )
}
