//! Simple button showcase with the FreyaCN theming system.
//!
//! Displays all button variants, sizes, icon placements, and states in
//! horizontally scrollable rows for easy browsing.

use freya::components::ScrollView;
use freya::prelude::*;
use freya_icons::lucide::{arrow_right, heart, settings};

use freyacn::components::Icon;
use freyacn::components::Label;
use freyacn::components::button::{Button, CNButton};
use freyacn::theme::{Theme, use_cn_theme, use_init_cn_theme};

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)));
}

fn app() -> impl IntoElement {
    let theme = Theme::neutral(true, "neutral");
    use_init_cn_theme(theme);

    rect()
        .width(Size::fill())
        .height(Size::fill())
        .background(theme.background)
        .child(ButtonShowcase)
}

#[derive(PartialEq, Clone)]
struct ButtonShowcase;

impl Component for ButtonShowcase {
    fn render(&self) -> impl IntoElement {
        let theme = use_cn_theme().read();

        // Outer scroll view for vertical scrolling
        ScrollView::new()
            .width(Size::fill())
            .height(Size::fill())
            .show_scrollbar(true)
            .child(
                rect()
                    .width(Size::fill())
                    .padding(Gaps::new_all(32.0))
                    .child(
                        // Card
                        rect()
                            .width(Size::fill())
                            .background(theme.card)
                            .corner_radius(16.0)
                            .padding(Gaps::new_all(32.0))
                            .shadow(Shadow::new().y(4.0).blur(12.0).color((0, 0, 0, 0.1)))
                            .child(
                                rect()
                                    .vertical()
                                    .spacing(28.0)
                                    .width(Size::fill())
                                    // Header
                                    .child(
                                        label()
                                            .font_size(28.0)
                                            .font_weight(FontWeight::BOLD)
                                            .color(theme.foreground)
                                            .text("Button Showcase"),
                                    )
                                    // Variants
                                    .child(
                                        label()
                                            .font_size(18.0)
                                            .font_weight(FontWeight::MEDIUM)
                                            .color(theme.foreground)
                                            .text("Variants"),
                                    )
                                    .child(scroll_row(vec![
                                        Button().child(Label("Default")),
                                        Button().child(Label("Destructive")).destructive(),
                                        Button().child(Label("Outline")).outline(),
                                        Button().child(Label("Secondary")).secondary(),
                                        Button().child(Label("Ghost")).ghost(),
                                        Button().child(Label("Link")).link(),
                                    ]))
                                    // Sizes
                                    .child(
                                        label()
                                            .font_size(18.0)
                                            .font_weight(FontWeight::MEDIUM)
                                            .color(theme.foreground)
                                            .text("Sizes"),
                                    )
                                    .child(scroll_row(vec![
                                        Button().child(Label("Default")),
                                        Button().child(Label("XS")).size_xs(),
                                        Button().child(Label("SM")).size_sm(),
                                        Button().child(Label("LG")).size_lg(),
                                    ]))
                                    // Icons
                                    .child(
                                        label()
                                            .font_size(18.0)
                                            .font_weight(FontWeight::MEDIUM)
                                            .color(theme.foreground)
                                            .text("Icons"),
                                    )
                                    .child(scroll_row(vec![
                                        Button().size_icon().child(Icon(heart())),
                                        Button().child(Label("Like")).child(Icon(heart())),
                                        Button().child(Label("Settings")).child(Icon(settings())),
                                        Button().child(Label("Next")).child(Icon(arrow_right())),
                                    ]))
                                    // States
                                    .child(
                                        label()
                                            .font_size(18.0)
                                            .font_weight(FontWeight::MEDIUM)
                                            .color(theme.foreground)
                                            .text("States"),
                                    )
                                    .child(scroll_row(vec![
                                        Button().child(Label("Default")),
                                        Button().child(Label("Disabled")).enabled(false),
                                        Button().child(Label("Focusable")).focusable(true),
                                        Button().child(Label("Custom")),
                                    ])),
                            ),
                    ),
            )
    }
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
        .max_height(Size::px(140.0))
        .show_scrollbar(true)
        .child(row)
}
