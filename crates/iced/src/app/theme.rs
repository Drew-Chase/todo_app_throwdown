pub mod color {
    use crate::math::color::oklch;
    use iced::Color;

    // Base colors
    pub const BACKGROUND: Color = Color::from_rgb8(0x00, 0x00, 0x00);
    pub const FOREGROUND: Color = oklch(0.938, 0.002, 250.0);

    // Primitives
    pub const WHITE: Color = oklch(1.0, 0.0, 0.0);
    pub const BLACK: Color = oklch(0.0, 0.0, 0.0);
    pub const SNOW: Color = oklch(0.985, 0.0, 0.0);
    pub const ECLIPSE: Color = oklch(0.145, 0.002, 285.0);

    /// Gold ramp — accent, selection, focus, reclaimable totals
    pub struct Gold;
    impl Gold {
        pub const _50: Color = oklch(0.972, 0.024, 88.0);
        pub const _100: Color = oklch(0.941, 0.045, 88.0);
        pub const _200: Color = oklch(0.897, 0.072, 87.0);
        pub const _300: Color = oklch(0.851, 0.096, 86.0);
        pub const _400: Color = oklch(0.797, 0.113, 85.0);
        pub const _500: Color = oklch(0.735, 0.115, 84.0);
        pub const _600: Color = oklch(0.661, 0.106, 82.0);
        pub const _700: Color = oklch(0.567, 0.090, 79.0);
        pub const _800: Color = oklch(0.451, 0.070, 77.0);
        pub const _900: Color = oklch(0.342, 0.052, 76.0);
    }

    /// Silver ramp — secondary data, metadata, dividers
    pub struct Silver;
    impl Silver {
        pub const _50: Color = oklch(0.976, 0.002, 250.0);
        pub const _100: Color = oklch(0.938, 0.003, 250.0);
        pub const _200: Color = oklch(0.885, 0.004, 250.0);
        pub const _300: Color = oklch(0.812, 0.005, 250.0);
        pub const _400: Color = oklch(0.723, 0.006, 250.0);
        pub const _500: Color = oklch(0.634, 0.007, 250.0);
        pub const _600: Color = oklch(0.541, 0.007, 250.0);
        pub const _700: Color = oklch(0.438, 0.006, 250.0);
        pub const _800: Color = oklch(0.330, 0.005, 250.0);
        pub const _900: Color = oklch(0.238, 0.004, 250.0);
    }

    /// Ink ramp — the near-black chassis
    pub struct Ink;
    impl Ink {
        pub const _950: Color = oklch(0.000, 0.0, 0.0);
        pub const _900: Color = oklch(0.126, 0.002, 285.0);
        pub const _850: Color = oklch(0.163, 0.002, 285.0);
        pub const _800: Color = oklch(0.201, 0.003, 285.0);
        pub const _750: Color = oklch(0.238, 0.003, 285.0);
        pub const _700: Color = oklch(0.276, 0.004, 285.0);
    }

    /// Surfaces: cards, panels, table chrome
    pub struct Surface;
    impl Surface {
        pub const DEFAULT: Color = Color::from_rgb8(0x0A, 0x0A, 0x0B);
        pub const FOREGROUND: Color = FOREGROUND;
        pub const SECONDARY: Color = Color::from_rgb8(0x10, 0x10, 0x13);
        pub const SECONDARY_FOREGROUND: Color = FOREGROUND;
        pub const TERTIARY: Color = Color::from_rgb8(0x17, 0x17, 0x1A);
        pub const TERTIARY_FOREGROUND: Color = FOREGROUND;
    }

    /// Overlays: popovers, tooltips, modals
    pub struct Overlay;
    impl Overlay {
        pub const DEFAULT: Color = Color::from_rgb8(0x0E, 0x0E, 0x11);
        pub const FOREGROUND: Color = FOREGROUND;
    }

    // Semantic colors
    pub const MUTED: Color = oklch(0.541, 0.007, 250.0);
    pub const PRIMARY: Color = oklch(0.797, 0.113, 85.0);
    pub const PRIMARY_LIGHT: Color = oklch(0.851, 0.096, 86.0);
    pub const PRIMARY_DARK: Color = oklch(0.735, 0.115, 84.0);
    pub const ACCENT: Color = PRIMARY;

    pub const PRIMARY_FOREGROUND: Color = oklch(0.145, 0.02, 85.0);
    pub const ACCENT_FOREGROUND: Color = SECONDARY_FOREGROUND;

    pub const SECONDARY_FOREGROUND: Color = Color::from_rgb8(0x14, 0x14, 0x14);
    pub const SECONDARY_LIGHT: Color = oklch(0.851, 0.096, 86.0);
    pub const SECONDARY_DARK: Color = oklch(0.735, 0.115, 84.0);
    pub const SECONDARY: Color = SNOW;

    /// Neutral action surface — silver-tinted graphite
    pub const DEFAULT: Color = Color::from_rgb8(0x17, 0x17, 0x1A);
    pub const DEFAULT_FOREGROUND: Color = oklch(0.885, 0.004, 250.0);

    /// Form fields
    pub struct Field;
    impl Field {
        pub const BACKGROUND: Color = Color::from_rgb8(0x10, 0x10, 0x13);
        pub const FOREGROUND: Color = FOREGROUND;
        pub const PLACEHOLDER: Color = oklch(0.438, 0.006, 250.0);
        pub const BORDER: Color = oklch(0.276, 0.004, 285.0);
    }

    /// Status colors
    pub struct Status;
    impl Status {
        pub const SUCCESS: Color = oklch(0.760, 0.150, 155.0);
        pub const SUCCESS_LIGHT: Color = oklch(0.820, 0.150, 155.0);
        pub const SUCCESS_DARK: Color = oklch(0.700, 0.150, 155.0);
        pub const SUCCESS_FOREGROUND: Color = ECLIPSE;
        pub const WARNING: Color = oklch(0.820, 0.139, 76.0);
        pub const WARNING_LIGHT: Color = oklch(0.880, 0.139, 76.0);
        pub const WARNING_DARK: Color = oklch(0.760, 0.139, 76.0);
        pub const WARNING_FOREGROUND: Color = ECLIPSE;
        pub const DANGER: Color = oklch(0.615, 0.185, 25.0);
        pub const DANGER_LIGHT: Color = oklch(0.675, 0.185, 25.0);
        pub const DANGER_DARK: Color = oklch(0.555, 0.185, 25.0);
        pub const DANGER_FOREGROUND: Color = SNOW;
    }

    // Component colors
    pub const SEGMENT: Color = Color::from_rgb8(0x1B, 0x1B, 0x1E);
    pub const SEGMENT_FOREGROUND: Color = FOREGROUND;
    pub const BORDER: Color = oklch(0.263, 0.003, 285.0);
    pub const SEPARATOR: Color = oklch(0.221, 0.003, 285.0);
    pub const FOCUS: Color = ACCENT;
    pub const LINK: Color = oklch(0.851, 0.096, 86.0);

    /// Sift-specific surfaces
    pub struct Todo;
    impl Todo {
        /// Titlebar / statusbar
        pub const CHROME: Color = Color::from_rgb8(0x07, 0x07, 0x08);
        pub const CHROME_FOREGROUND: Color = oklch(0.723, 0.006, 250.0);
        /// Sidebars, rails
        pub const PANEL: Color = Color::from_rgb8(0x08, 0x08, 0x0A);
        /// Treemap canvas
        pub const MAP: Color = Color::from_rgb8(0x05, 0x05, 0x06);
        /// Numeric/size column ink
        pub const SILVER: Color = oklch(0.885, 0.004, 250.0);
    }

    /// Treemap category colors
    pub struct Category;
    impl Category {
        pub const APPS: Color = Color::from_rgb8(0xC9, 0xA2, 0x4A);
        pub const GAMES: Color = Color::from_rgb8(0xE8, 0xC8, 0x78);
        pub const MEDIA: Color = Color::from_rgb8(0xD8, 0xDC, 0xE2);
        pub const SYSTEM: Color = Color::from_rgb8(0x8E, 0x93, 0x9B);
        pub const DOCUMENTS: Color = Color::from_rgb8(0xA7, 0xAC, 0xB4);
        pub const CACHE: Color = Color::from_rgb8(0x8A, 0x6E, 0x28);
        pub const ARCHIVES: Color = Color::from_rgb8(0x6E, 0x74, 0x7E);
        pub const OTHER: Color = Color::from_rgb8(0x4A, 0x4E, 0x55);
    }
}

pub mod layout {
    /// Geometry — squarer than typical rounded UI
    pub struct Radius;
    impl Radius {
        pub const DEFAULT: f32 = 6.0; // 6px
        pub const FIELD: f32 = 7.2; // DEFAULT * 1.2
        pub const XL: f32 = 12.0; // 12px
        pub const LG: f32 = 8.0; // 8px
        pub const MD: f32 = 6.0; // 6px
        pub const SM: f32 = 4.0; // 4px
        pub const NONE: f32 = 0.0;
        pub const FULL: f32 = 9999.0;
    }

    pub struct Border;
    impl Border {
        pub const WIDTH: f32 = 1.0;
        pub const FIELD_WIDTH: f32 = 1.0;
        pub const RING_OFFSET_WIDTH: f32 = 2.0;
    }

    /// Chassis metrics
    pub struct Spacing;
    impl Spacing {
        /// One table row — 27px
        pub const ROW: f32 = 1.6875;
        pub const TOOLBAR: f32 = 2.875;
        pub const STATUSBAR: f32 = 1.625;
    }

    pub const DISABLED_OPACITY: f32 = 0.45;
}

pub mod utility {
    use iced::widget::svg;
    use iced::{Color, Element, Length};

    pub fn icon<'a, M: 'a>(
        markup: &'static str,
        size: impl Into<Length> + std::marker::Copy,
        colors: Option<(Color, Color)>,
    ) -> Element<'a, M> {
        let colors = colors.unwrap_or((Color::WHITE, Color::WHITE));
        svg(svg::Handle::from_memory(markup.as_bytes()))
            .width(size)
            .height(size)
            .style(move |_theme, status| svg::Style {
                color: Some(match status {
                    svg::Status::Hovered => colors.0,
                    _ => colors.1,
                }),
            })
            .into()
    }

    pub fn alpha(color: Color, alpha: f32) -> Color {
        Color { a: alpha, ..color }
    }
}

pub trait ColorExt {
    fn alpha(self, alpha: f32) -> Self;
}

impl ColorExt for iced::Color {
    fn alpha(mut self, alpha: f32) -> Self {
        self.a = alpha;
        self
    }
}
