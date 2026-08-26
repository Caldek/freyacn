use freya::prelude::*;
use freya_icons::lucide::arrow_left;

use freyacn::components::button::Button;
use freyacn::core::theme::Theme as CNTheme;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)));
}

fn app() -> impl IntoElement {
    let theme = CNTheme::light();

    provide_root_context(theme);

    rect()
        .width(Size::fill())
        .height(Size::fill())
        .background(theme.background)
        .padding(Gaps::new_all(32.0))
        .child(
            rect()
                .spacing(12.0)
                .child(label().font_size(24.0).text("Button"))
                .child(Button().label("Small").size_sm())
                .child(Button().label("Default").outline())
                .child(Button().label("Extra Small").destructive().size_xs())
                .child(Button().label("Large").secondary().size_lg())
                .child(
                    Button()
                        .label("Icon")
                        .outline()
                        .icon(SvgViewer::new(("arrow_left", arrow_left()))),
                ),
        )
}
