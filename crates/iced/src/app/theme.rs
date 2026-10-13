pub mod color {
    use iced::Color;


    pub struct Text;

    impl Text {
        pub const DEFAULT: Color = Color::from_rgb8(0x18, 0x18, 0x1b);
        pub const PLACEHOLDER: Color = Color::from_rgb8(0x8e, 0x8e, 0x95);
    }

    pub struct Surface;
    impl Surface {
        pub const APPLICATION: Color = Color::from_rgb8(0xe3, 0xe3, 0xea);
        pub const CARD: Color = Color::from_rgb8(0xff, 0xff, 0xff);
        pub const TITLEBAR: Color = Color::from_rgb8(0xf3, 0xf4, 0xf6);
        pub const BUTTON: Color = Color::from_rgb8(0xf1, 0x44, 0x4e);
        pub const BUTTON_HOVER: Color = Color::from_rgb8(0xf5, 0x5c, 0x5f);
        pub const CLOSE_BUTTON_HOVER: Color = Color::from_rgb8(0xff, 0x46, 0x44);
        pub const PRIMARY_DARK: Color = Color::from_rgb8(0xaa, 0x3b, 0x3f);
    }

    pub mod buttons {
        use iced::Color;

        pub struct Primary;
        impl Primary {
            pub const FOREGROUND: Color = Color::WHITE;
            pub const BACKGROUND: Color = Color::from_rgb8(0xf1, 0x44, 0x4e); // Primary Red
            pub const BACKGROUND_HOVER: Color = Color::from_rgb8(0xf5, 0x5c, 0x5f);
            pub const BACKGROUND_DISABLED: Color = Color::from_rgb8(0xf8, 0xa1, 0xa6);
        }

        pub struct Secondary;
        impl Secondary {
            pub const FOREGROUND: Color = super::Text::DEFAULT;
            pub const BACKGROUND: Color = Color::TRANSPARENT;
            pub const BACKGROUND_HOVER: Color = Color::from_rgb8(0xe5, 0xe7, 0xeb);
        }

        pub struct Danger;
        impl Danger {
            pub const FOREGROUND: Color = Color::from_rgb8(0xa4,0x3a,0x37);
            pub const FOREGROUND_HOVER: Color = Color::WHITE;
            pub const BACKGROUND: Color = Color::TRANSPARENT;
            pub const BACKGROUND_HOVER: Color = Color::from_rgb8(0xff,0x46,0x44);
        }
    }
}

pub mod fonts {
    use iced::Font;
    use iced::font::{Family, Weight};

    pub struct Roboto;
    impl Roboto {
        pub const BOLD: Font = Font {
            family: Family::Name("Roboto"),
            weight: Weight::Bold,
            ..Font::DEFAULT
        };
        pub const REGULAR: Font = Font {
            family: Family::Name("Roboto"),
            weight: Weight::Normal,
            ..Font::DEFAULT
        };
    }
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
