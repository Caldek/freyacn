use freya::prelude::*;
use freya_icons::lucide::arrow_left;

use freyacn::components::button::Button;
use freyacn::components::icon::Icon;
use freyacn::core::theme::Theme as CNTheme;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)));
}

fn app() -> impl IntoElement {
    let theme = CNTheme::neutral(true, "neutral");

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
                .child(Button().label("Default"))
                .child(Button().label("Small").size_sm())
                .child(Button().label("Extra Small").destructive().size_xs())
                .child(Button().label("Large").secondary().size_lg())
                .child(Button().label("Icon").size_icon().icon(Icon(arrow_left()))),
        )
}
