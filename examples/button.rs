//! Simple button showcase with the FreyaCN theming system.
//!
//! Displays all button variants, sizes, icon placements, and states in
//! horizontally scrollable rows for easy browsing.

use freya::components::ScrollView;
use freya::prelude::*;
use freya_icons::lucide::{arrow_right, heart, settings};

use freyacn::components::button::{Button, CNButton};
use freyacn::components::icon::Icon;
use freyacn::core::theme::{Theme, use_init_cn_theme};

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)));
}

fn app() -> impl IntoElement {
    let theme = Theme::neutral(false, "neutral");
    use_init_cn_theme(theme);

    rect()
        .width(Size::fill())
        .height(Size::fill())
        .background(theme.background)
        .padding(Gaps::new_all(32.0))
        .child(
            rect()
                .vertical()
                .spacing(24.0)
                // Header
                .child(label().font_size(24.0).text("Button Showcase"))
                // Variants
                .child(label().font_size(18.0).text("Variants"))
                .child(scroll_row(vec![
                    Button().label("Default"),
                    Button().label("Destructive").destructive(),
                    Button().label("Outline").outline(),
                    Button().label("Secondary").secondary(),
                    Button().label("Ghost").ghost(),
                    Button().label("Link").link(),
                ]))
                // Sizes
                .child(label().font_size(18.0).text("Sizes"))
                .child(scroll_row(vec![
                    Button().label("Default"),
                    Button().label("XS").size_xs(),
                    Button().label("SM").size_sm(),
                    Button().label("LG").size_lg(),
                ]))
                // Icons
                .child(label().font_size(18.0).text("Icons"))
                .child(scroll_row(vec![
                    Button().size_icon().child(Icon(heart())),
                    Button().label("Like").child(Icon(heart())),
                    Button().label("Settings").child(Icon(settings())),
                    Button().label("Next").child(Icon(arrow_right())),
                ]))
                // States
                .child(label().font_size(18.0).text("States"))
                .child(scroll_row(vec![
                    Button().label("Default"),
                    Button().label("Disabled").enabled(false),
                    Button().label("Focusable").focusable(true),
                    Button().label("Custom"),
                ])),
        )
}

/// Helper to create a horizontally scrollable row of buttons.
fn scroll_row(buttons: Vec<CNButton>) -> impl IntoElement {
    let mut row = rect()
        .horizontal()
        .spacing(12.0)
        .padding(Gaps::new_all(4.0));

    for btn in buttons {
        row = row.child(btn);
    }

    ScrollView::new()
        .width(Size::fill())
        .height(Size::px(120.0)) // fixed height to accommodate buttons
        .child(row)
}
