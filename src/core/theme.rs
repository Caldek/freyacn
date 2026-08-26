use freya::prelude::*;

/// Convert an RGB tuple into a Freya Color.
///
/// Freya 0.4.1 accepts RGB tuples anywhere a Color is expected.
const fn rgb(r: u8, g: u8, b: u8) -> Color {
    Color::from_rgb(r, g, b)
}

/// Complete FreyaCN color palette.
///
/// These are based on the Tailwind/shadcn-style palette.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Colors {
    // basic colors
    pub white: Color,
    pub black: Color,

    // Slate
    pub slate_50: Color,
    pub slate_100: Color,
    pub slate_200: Color,
    pub slate_300: Color,
    pub slate_400: Color,
    pub slate_500: Color,
    pub slate_600: Color,
    pub slate_700: Color,
    pub slate_800: Color,
    pub slate_900: Color,
    pub slate_950: Color,

    // Gray
    pub gray_50: Color,
    pub gray_100: Color,
    pub gray_200: Color,
    pub gray_300: Color,
    pub gray_400: Color,
    pub gray_500: Color,
    pub gray_600: Color,
    pub gray_700: Color,
    pub gray_800: Color,
    pub gray_900: Color,
    pub gray_950: Color,

    // Zinc
    pub zinc_50: Color,
    pub zinc_100: Color,
    pub zinc_200: Color,
    pub zinc_300: Color,
    pub zinc_400: Color,
    pub zinc_500: Color,
    pub zinc_600: Color,
    pub zinc_700: Color,
    pub zinc_800: Color,
    pub zinc_900: Color,
    pub zinc_950: Color,

    // Neutral
    pub neutral_50: Color,
    pub neutral_100: Color,
    pub neutral_200: Color,
    pub neutral_300: Color,
    pub neutral_400: Color,
    pub neutral_500: Color,
    pub neutral_600: Color,
    pub neutral_700: Color,
    pub neutral_800: Color,
    pub neutral_900: Color,
    pub neutral_950: Color,

    // Red
    pub red_50: Color,
    pub red_100: Color,
    pub red_200: Color,
    pub red_300: Color,
    pub red_400: Color,
    pub red_500: Color,
    pub red_600: Color,
    pub red_700: Color,
    pub red_800: Color,
    pub red_900: Color,
    pub red_950: Color,

    // Orange
    pub orange_50: Color,
    pub orange_100: Color,
    pub orange_200: Color,
    pub orange_300: Color,
    pub orange_400: Color,
    pub orange_500: Color,
    pub orange_600: Color,
    pub orange_700: Color,
    pub orange_800: Color,
    pub orange_900: Color,
    pub orange_950: Color,

    // Amber
    pub amber_50: Color,
    pub amber_100: Color,
    pub amber_200: Color,
    pub amber_300: Color,
    pub amber_400: Color,
    pub amber_500: Color,
    pub amber_600: Color,
    pub amber_700: Color,
    pub amber_800: Color,
    pub amber_900: Color,
    pub amber_950: Color,

    // Yellow
    pub yellow_50: Color,
    pub yellow_100: Color,
    pub yellow_200: Color,
    pub yellow_300: Color,
    pub yellow_400: Color,
    pub yellow_500: Color,
    pub yellow_600: Color,
    pub yellow_700: Color,
    pub yellow_800: Color,
    pub yellow_900: Color,
    pub yellow_950: Color,

    // Lime
    pub lime_50: Color,
    pub lime_100: Color,
    pub lime_200: Color,
    pub lime_300: Color,
    pub lime_400: Color,
    pub lime_500: Color,
    pub lime_600: Color,
    pub lime_700: Color,
    pub lime_800: Color,
    pub lime_900: Color,
    pub lime_950: Color,

    // Green
    pub green_50: Color,
    pub green_100: Color,
    pub green_200: Color,
    pub green_300: Color,
    pub green_400: Color,
    pub green_500: Color,
    pub green_600: Color,
    pub green_700: Color,
    pub green_800: Color,
    pub green_900: Color,
    pub green_950: Color,

    // Emerald
    pub emerald_50: Color,
    pub emerald_100: Color,
    pub emerald_200: Color,
    pub emerald_300: Color,
    pub emerald_400: Color,
    pub emerald_500: Color,
    pub emerald_600: Color,
    pub emerald_700: Color,
    pub emerald_800: Color,
    pub emerald_900: Color,
    pub emerald_950: Color,

    // Teal
    pub teal_50: Color,
    pub teal_100: Color,
    pub teal_200: Color,
    pub teal_300: Color,
    pub teal_400: Color,
    pub teal_500: Color,
    pub teal_600: Color,
    pub teal_700: Color,
    pub teal_800: Color,
    pub teal_900: Color,
    pub teal_950: Color,

    // Cyan
    pub cyan_50: Color,
    pub cyan_100: Color,
    pub cyan_200: Color,
    pub cyan_300: Color,
    pub cyan_400: Color,
    pub cyan_500: Color,
    pub cyan_600: Color,
    pub cyan_700: Color,
    pub cyan_800: Color,
    pub cyan_900: Color,
    pub cyan_950: Color,

    // Sky
    pub sky_50: Color,
    pub sky_100: Color,
    pub sky_200: Color,
    pub sky_300: Color,
    pub sky_400: Color,
    pub sky_500: Color,
    pub sky_600: Color,
    pub sky_700: Color,
    pub sky_800: Color,
    pub sky_900: Color,
    pub sky_950: Color,

    // Blue
    pub blue_50: Color,
    pub blue_100: Color,
    pub blue_200: Color,
    pub blue_300: Color,
    pub blue_400: Color,
    pub blue_500: Color,
    pub blue_600: Color,
    pub blue_700: Color,
    pub blue_800: Color,
    pub blue_900: Color,
    pub blue_950: Color,

    // Indigo
    pub indigo_50: Color,
    pub indigo_100: Color,
    pub indigo_200: Color,
    pub indigo_300: Color,
    pub indigo_400: Color,
    pub indigo_500: Color,
    pub indigo_600: Color,
    pub indigo_700: Color,
    pub indigo_800: Color,
    pub indigo_900: Color,
    pub indigo_950: Color,

    // Violet
    pub violet_50: Color,
    pub violet_100: Color,
    pub violet_200: Color,
    pub violet_300: Color,
    pub violet_400: Color,
    pub violet_500: Color,
    pub violet_600: Color,
    pub violet_700: Color,
    pub violet_800: Color,
    pub violet_900: Color,
    pub violet_950: Color,

    // Purple
    pub purple_50: Color,
    pub purple_100: Color,
    pub purple_200: Color,
    pub purple_300: Color,
    pub purple_400: Color,
    pub purple_500: Color,
    pub purple_600: Color,
    pub purple_700: Color,
    pub purple_800: Color,
    pub purple_900: Color,
    pub purple_950: Color,

    // Fuchsia
    pub fuchsia_50: Color,
    pub fuchsia_100: Color,
    pub fuchsia_200: Color,
    pub fuchsia_300: Color,
    pub fuchsia_400: Color,
    pub fuchsia_500: Color,
    pub fuchsia_600: Color,
    pub fuchsia_700: Color,
    pub fuchsia_800: Color,
    pub fuchsia_900: Color,
    pub fuchsia_950: Color,

    // Pink
    pub pink_50: Color,
    pub pink_100: Color,
    pub pink_200: Color,
    pub pink_300: Color,
    pub pink_400: Color,
    pub pink_500: Color,
    pub pink_600: Color,
    pub pink_700: Color,
    pub pink_800: Color,
    pub pink_900: Color,
    pub pink_950: Color,

    // Rose
    pub rose_50: Color,
    pub rose_100: Color,
    pub rose_200: Color,
    pub rose_300: Color,
    pub rose_400: Color,
    pub rose_500: Color,
    pub rose_600: Color,
    pub rose_700: Color,
    pub rose_800: Color,
    pub rose_900: Color,
    pub rose_950: Color,
}

impl Colors {
    pub fn new() -> Self {
        Self {
            // basic
            white: rgb(255, 255, 255),
            black: rgb(0, 0, 0),

            // Slate
            slate_50: rgb(248, 250, 252),
            slate_100: rgb(241, 245, 249),
            slate_200: rgb(226, 232, 240),
            slate_300: rgb(203, 213, 225),
            slate_400: rgb(148, 163, 184),
            slate_500: rgb(100, 116, 139),
            slate_600: rgb(71, 85, 105),
            slate_700: rgb(51, 65, 85),
            slate_800: rgb(30, 41, 59),
            slate_900: rgb(15, 23, 42),
            slate_950: rgb(2, 6, 23),

            // Gray
            gray_50: rgb(249, 250, 251),
            gray_100: rgb(243, 244, 246),
            gray_200: rgb(229, 231, 235),
            gray_300: rgb(209, 213, 219),
            gray_400: rgb(156, 163, 175),
            gray_500: rgb(107, 114, 128),
            gray_600: rgb(75, 85, 99),
            gray_700: rgb(55, 65, 81),
            gray_800: rgb(31, 41, 55),
            gray_900: rgb(17, 24, 39),
            gray_950: rgb(3, 7, 18),

            // Zinc
            zinc_50: rgb(250, 250, 250),
            zinc_100: rgb(244, 244, 245),
            zinc_200: rgb(228, 228, 231),
            zinc_300: rgb(212, 212, 216),
            zinc_400: rgb(161, 161, 170),
            zinc_500: rgb(113, 113, 122),
            zinc_600: rgb(82, 82, 91),
            zinc_700: rgb(63, 63, 70),
            zinc_800: rgb(39, 39, 42),
            zinc_900: rgb(24, 24, 27),
            zinc_950: rgb(9, 9, 11),

            // Neutral
            neutral_50: rgb(250, 250, 250),
            neutral_100: rgb(245, 245, 245),
            neutral_200: rgb(229, 229, 229),
            neutral_300: rgb(212, 212, 212),
            neutral_400: rgb(163, 163, 163),
            neutral_500: rgb(115, 115, 115),
            neutral_600: rgb(82, 82, 82),
            neutral_700: rgb(64, 64, 64),
            neutral_800: rgb(38, 38, 38),
            neutral_900: rgb(23, 23, 23),
            neutral_950: rgb(10, 10, 10),

            // Red
            red_50: rgb(254, 242, 242),
            red_100: rgb(254, 226, 226),
            red_200: rgb(254, 202, 202),
            red_300: rgb(252, 165, 165),
            red_400: rgb(248, 113, 113),
            red_500: rgb(239, 68, 68),
            red_600: rgb(220, 38, 38),
            red_700: rgb(185, 28, 28),
            red_800: rgb(153, 27, 27),
            red_900: rgb(127, 29, 29),
            red_950: rgb(69, 10, 10),

            // Orange
            orange_50: rgb(255, 247, 237),
            orange_100: rgb(255, 237, 213),
            orange_200: rgb(254, 215, 170),
            orange_300: rgb(253, 186, 116),
            orange_400: rgb(251, 146, 60),
            orange_500: rgb(249, 115, 22),
            orange_600: rgb(234, 88, 12),
            orange_700: rgb(194, 65, 12),
            orange_800: rgb(154, 52, 18),
            orange_900: rgb(124, 45, 18),
            orange_950: rgb(67, 20, 7),

            // Amber
            amber_50: rgb(255, 251, 235),
            amber_100: rgb(254, 243, 199),
            amber_200: rgb(253, 230, 138),
            amber_300: rgb(252, 211, 77),
            amber_400: rgb(251, 191, 36),
            amber_500: rgb(245, 158, 11),
            amber_600: rgb(217, 119, 6),
            amber_700: rgb(180, 83, 9),
            amber_800: rgb(146, 64, 14),
            amber_900: rgb(120, 53, 15),
            amber_950: rgb(69, 26, 3),

            // Yellow
            yellow_50: rgb(254, 252, 232),
            yellow_100: rgb(254, 249, 195),
            yellow_200: rgb(254, 240, 138),
            yellow_300: rgb(253, 224, 71),
            yellow_400: rgb(250, 204, 21),
            yellow_500: rgb(234, 179, 8),
            yellow_600: rgb(202, 138, 4),
            yellow_700: rgb(161, 98, 7),
            yellow_800: rgb(133, 77, 14),
            yellow_900: rgb(113, 63, 18),
            yellow_950: rgb(66, 32, 6),

            // Lime
            lime_50: rgb(247, 254, 231),
            lime_100: rgb(236, 252, 203),
            lime_200: rgb(217, 249, 157),
            lime_300: rgb(190, 242, 100),
            lime_400: rgb(163, 230, 53),
            lime_500: rgb(132, 204, 22),
            lime_600: rgb(101, 163, 13),
            lime_700: rgb(77, 124, 15),
            lime_800: rgb(63, 98, 18),
            lime_900: rgb(54, 83, 20),
            lime_950: rgb(26, 46, 5),

            // Green
            green_50: rgb(240, 253, 244),
            green_100: rgb(220, 252, 231),
            green_200: rgb(187, 247, 208),
            green_300: rgb(134, 239, 172),
            green_400: rgb(74, 222, 128),
            green_500: rgb(34, 197, 94),
            green_600: rgb(22, 163, 74),
            green_700: rgb(21, 128, 61),
            green_800: rgb(22, 101, 52),
            green_900: rgb(20, 83, 45),
            green_950: rgb(5, 46, 22),

            // Emerald
            emerald_50: rgb(236, 253, 245),
            emerald_100: rgb(209, 250, 229),
            emerald_200: rgb(167, 243, 208),
            emerald_300: rgb(110, 231, 183),
            emerald_400: rgb(52, 211, 153),
            emerald_500: rgb(16, 185, 129),
            emerald_600: rgb(5, 150, 105),
            emerald_700: rgb(4, 120, 87),
            emerald_800: rgb(6, 95, 70),
            emerald_900: rgb(6, 78, 59),
            emerald_950: rgb(2, 44, 34),

            // Teal
            teal_50: rgb(240, 253, 250),
            teal_100: rgb(204, 251, 241),
            teal_200: rgb(153, 246, 228),
            teal_300: rgb(94, 234, 212),
            teal_400: rgb(45, 212, 191),
            teal_500: rgb(20, 184, 166),
            teal_600: rgb(13, 148, 136),
            teal_700: rgb(15, 118, 110),
            teal_800: rgb(17, 94, 89),
            teal_900: rgb(19, 78, 74),
            teal_950: rgb(4, 47, 46),

            // Cyan
            cyan_50: rgb(236, 254, 255),
            cyan_100: rgb(207, 250, 254),
            cyan_200: rgb(165, 243, 252),
            cyan_300: rgb(103, 232, 249),
            cyan_400: rgb(34, 211, 238),
            cyan_500: rgb(6, 182, 212),
            cyan_600: rgb(8, 145, 178),
            cyan_700: rgb(14, 116, 144),
            cyan_800: rgb(21, 94, 117),
            cyan_900: rgb(22, 78, 99),
            cyan_950: rgb(8, 51, 68),

            // Sky
            sky_50: rgb(240, 249, 255),
            sky_100: rgb(224, 242, 254),
            sky_200: rgb(186, 230, 253),
            sky_300: rgb(125, 211, 252),
            sky_400: rgb(56, 189, 248),
            sky_500: rgb(14, 165, 233),
            sky_600: rgb(2, 132, 199),
            sky_700: rgb(3, 105, 161),
            sky_800: rgb(7, 89, 133),
            sky_900: rgb(12, 74, 110),
            sky_950: rgb(8, 47, 73),

            // Blue
            blue_50: rgb(239, 246, 255),
            blue_100: rgb(219, 234, 254),
            blue_200: rgb(191, 219, 254),
            blue_300: rgb(147, 197, 253),
            blue_400: rgb(96, 165, 250),
            blue_500: rgb(59, 130, 246),
            blue_600: rgb(37, 99, 235),
            blue_700: rgb(29, 78, 216),
            blue_800: rgb(30, 64, 175),
            blue_900: rgb(30, 58, 138),
            blue_950: rgb(23, 37, 84),

            // Indigo
            indigo_50: rgb(238, 242, 255),
            indigo_100: rgb(224, 231, 255),
            indigo_200: rgb(199, 210, 254),
            indigo_300: rgb(165, 180, 252),
            indigo_400: rgb(129, 140, 248),
            indigo_500: rgb(99, 102, 241),
            indigo_600: rgb(79, 70, 229),
            indigo_700: rgb(67, 56, 202),
            indigo_800: rgb(55, 48, 163),
            indigo_900: rgb(49, 46, 129),
            indigo_950: rgb(30, 27, 75),

            // Violet
            violet_50: rgb(245, 243, 255),
            violet_100: rgb(237, 233, 254),
            violet_200: rgb(221, 214, 254),
            violet_300: rgb(196, 181, 253),
            violet_400: rgb(167, 139, 250),
            violet_500: rgb(139, 92, 246),
            violet_600: rgb(124, 58, 237),
            violet_700: rgb(109, 40, 217),
            violet_800: rgb(91, 33, 182),
            violet_900: rgb(76, 29, 149),
            violet_950: rgb(46, 16, 101),

            // Purple
            purple_50: rgb(250, 245, 255),
            purple_100: rgb(243, 232, 255),
            purple_200: rgb(233, 213, 255),
            purple_300: rgb(216, 180, 254),
            purple_400: rgb(192, 132, 252),
            purple_500: rgb(168, 85, 247),
            purple_600: rgb(147, 51, 234),
            purple_700: rgb(126, 34, 206),
            purple_800: rgb(107, 33, 168),
            purple_900: rgb(88, 28, 135),
            purple_950: rgb(59, 7, 100),

            // Fuchsia
            fuchsia_50: rgb(253, 244, 255),
            fuchsia_100: rgb(250, 232, 255),
            fuchsia_200: rgb(245, 208, 254),
            fuchsia_300: rgb(240, 171, 252),
            fuchsia_400: rgb(232, 121, 249),
            fuchsia_500: rgb(217, 70, 239),
            fuchsia_600: rgb(192, 38, 211),
            fuchsia_700: rgb(162, 28, 175),
            fuchsia_800: rgb(134, 25, 143),
            fuchsia_900: rgb(112, 26, 117),
            fuchsia_950: rgb(74, 4, 78),

            // Pink
            pink_50: rgb(253, 242, 248),
            pink_100: rgb(252, 231, 243),
            pink_200: rgb(251, 207, 232),
            pink_300: rgb(249, 168, 212),
            pink_400: rgb(244, 114, 182),
            pink_500: rgb(236, 72, 153),
            pink_600: rgb(219, 39, 119),
            pink_700: rgb(190, 24, 93),
            pink_800: rgb(157, 23, 77),
            pink_900: rgb(131, 24, 67),
            pink_950: rgb(80, 7, 36),

            // Rose
            rose_50: rgb(255, 241, 242),
            rose_100: rgb(255, 228, 230),
            rose_200: rgb(254, 205, 211),
            rose_300: rgb(253, 164, 175),
            rose_400: rgb(251, 113, 133),
            rose_500: rgb(244, 63, 94),
            rose_600: rgb(225, 29, 72),
            rose_700: rgb(190, 18, 60),
            rose_800: rgb(159, 18, 57),
            rose_900: rgb(136, 19, 55),
            rose_950: rgb(76, 5, 25),
        }
    }
}

impl Default for Colors {
    fn default() -> Self {
        Self::new()
    }
}

/// Semantic FreyaCN theme.
///
/// Components should generally use these semantic colors instead
/// of directly depending on the palette.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Theme {
    pub colors: Colors,

    pub background: Color,
    pub foreground: Color,

    pub primary: Color,
    pub primary_foreground: Color,

    pub secondary: Color,
    pub secondary_foreground: Color,

    pub muted: Color,
    pub muted_foreground: Color,

    pub accent: Color,
    pub accent_foreground: Color,

    pub destructive: Color,
    pub destructive_foreground: Color,

    pub card: Color,
    pub card_foreground: Color,

    pub popover: Color,
    pub popover_foreground: Color,

    pub border: Color,
    pub input: Color,
    pub ring: Color,

    pub chart_1: Color,
    pub chart_2: Color,
    pub chart_3: Color,
    pub chart_4: Color,
    pub chart_5: Color,
}

impl Theme {
    /// shadcn-style light theme.
    pub fn light() -> Self {
        let colors = Colors::default();

        Self {
            colors: colors.clone(),

            background: colors.white,
            foreground: colors.slate_950,

            primary: colors.slate_900,
            primary_foreground: colors.slate_50,

            secondary: colors.slate_100,
            secondary_foreground: colors.slate_900,

            muted: colors.slate_100,
            muted_foreground: colors.slate_500,

            accent: colors.slate_100,
            accent_foreground: colors.slate_900,

            destructive: colors.red_500,
            destructive_foreground: colors.slate_50,

            card: colors.white,
            card_foreground: colors.slate_950,

            popover: colors.white,
            popover_foreground: colors.slate_950,

            border: colors.slate_200,
            input: colors.slate_200,
            ring: colors.slate_950,

            chart_1: colors.blue_500,
            chart_2: colors.green_500,
            chart_3: colors.yellow_500,
            chart_4: colors.red_500,
            chart_5: colors.violet_500,
        }
    }

    /// shadcn-style dark theme.
    pub fn dark() -> Self {
        let colors = Colors::default();

        Self {
            colors: colors.clone(),

            background: colors.black,
            foreground: colors.slate_50,

            primary: colors.slate_50,
            primary_foreground: colors.slate_900,

            secondary: colors.slate_800,
            secondary_foreground: colors.slate_50,

            muted: colors.slate_800,
            muted_foreground: colors.slate_400,

            accent: colors.slate_800,
            accent_foreground: colors.slate_50,

            destructive: colors.red_900,
            destructive_foreground: colors.slate_50,

            card: colors.slate_950,
            card_foreground: colors.slate_50,

            popover: colors.slate_950,
            popover_foreground: colors.slate_50,

            border: colors.slate_800,
            input: colors.slate_800,
            ring: colors.slate_300,

            chart_1: colors.blue_500,
            chart_2: colors.green_500,
            chart_3: colors.yellow_500,
            chart_4: colors.red_500,
            chart_5: colors.violet_500,
        }
    }
}

// todo create use_theme hook
// todo add methods like set_primary("neutral"), set_accent, etc which will set methods like primary_50, primary_100, etc
