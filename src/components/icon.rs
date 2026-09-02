use freya::components::SvgViewer;
use freya::prelude::*;

use crate::core::CNExt;
use crate::core::theme::Theme;

/// A flexible SVG icon component.
#[derive(PartialEq, Clone)]
pub struct CNIcon {
    svg_data: Bytes,
    width: f32,
    height: f32,
    color: Option<Color>,
    key: DiffKey,
}

impl CNIcon {
    /// Create a new icon with the given SVG data (e.g., from `icons::lucide::heart()`).
    pub fn new(icon: Bytes) -> Self {
        Self {
            svg_data: icon,
            width: 24.0,
            height: 24.0,
            color: None,
            key: DiffKey::None,
        }
    }

    /// Set both width and height.
    pub fn size(mut self, size: f32) -> Self {
        self.width = size;
        self.height = size;
        self
    }

    /// Set width only.
    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    /// Set height only.
    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    /// Set the icon color (fill/stroke).
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Apply the theme's foreground color.
    pub fn color_theme(mut self, theme: &Theme) -> Self {
        self.color = Some(theme.foreground);
        self
    }

    // ============================================================
    // Size helpers
    // ============================================================
    pub fn size_1(self) -> Self {
        self.size(12.0)
    }
    pub fn size_2(self) -> Self {
        self.size(14.0)
    }
    pub fn size_3(self) -> Self {
        self.size(16.0)
    }
    pub fn size_4(self) -> Self {
        self.size(18.0)
    }
    pub fn size_5(self) -> Self {
        self.size(20.0)
    }
    pub fn size_6(self) -> Self {
        self.size(24.0)
    }
    pub fn size_7(self) -> Self {
        self.size(28.0)
    }
    pub fn size_8(self) -> Self {
        self.size(32.0)
    }
    pub fn size_9(self) -> Self {
        self.size(36.0)
    }

    pub fn size_xs(self) -> Self {
        self.size_1()
    }
    pub fn size_sm(self) -> Self {
        self.size_3()
    }
    pub fn size_md(self) -> Self {
        self.size_5()
    }
    pub fn size_lg(self) -> Self {
        self.size_6()
    }
    pub fn size_xl(self) -> Self {
        self.size_8()
    }
    pub fn size_2xl(self) -> Self {
        self.size(40.0)
    }
    pub fn size_3xl(self) -> Self {
        self.size(48.0)
    }

    pub fn size_12(self) -> Self {
        self.size(12.0)
    }
    pub fn size_14(self) -> Self {
        self.size(14.0)
    }
    pub fn size_16(self) -> Self {
        self.size(16.0)
    }
    pub fn size_18(self) -> Self {
        self.size(18.0)
    }
    pub fn size_20(self) -> Self {
        self.size(20.0)
    }
    pub fn size_24(self) -> Self {
        self.size(24.0)
    }
    pub fn size_28(self) -> Self {
        self.size(28.0)
    }
    pub fn size_32(self) -> Self {
        self.size(32.0)
    }
    pub fn size_36(self) -> Self {
        self.size(36.0)
    }
    pub fn size_40(self) -> Self {
        self.size(40.0)
    }
    pub fn size_48(self) -> Self {
        self.size(48.0)
    }
    pub fn size_64(self) -> Self {
        self.size(64.0)
    }
    pub fn size_96(self) -> Self {
        self.size(96.0)
    }
}

impl KeyExt for CNIcon {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl Component for CNIcon {
    fn render(&self) -> impl IntoElement {
        let base = SvgViewer::new(self.svg_data.clone())
            .width(Size::px(self.width))
            .height(Size::px(self.height));

        if let Some(color) = self.color {
            base.color(color)
        } else {
            base
        }
    }
}

/// Constructor for the Icon component.
#[allow(non_snake_case)]
pub fn Icon(icon: Bytes) -> CNIcon {
    CNIcon::new(icon)
}

// ------------------------------------------------------------
// CNExt implementation
// ------------------------------------------------------------
impl CNExt for CNIcon {
    fn background(self, _color: Color) -> Self {
        self
    }

    fn color(self, color: Color) -> Self {
        self.color(color)
    }
}
