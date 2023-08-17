use iced::{
    application, executor, theme,
    widget::{text_input, Container},
    window::{Level, WindowButtons, PlatformSpecific},
    Application, Background, BorderRadius, Color, Command, Element, Font, Length, Settings, Theme,
};

#[derive(Debug, Default)]
struct CommandBar {
    text: String,
}

#[derive(Debug, PartialEq, Clone)]
enum Message {
    InputChanged(String),
}

impl Application for CommandBar {
    type Message = Message;
    type Executor = executor::Default;
    type Flags = ();
    type Theme = Theme;

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("cmdbar")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::InputChanged(text) => self.text = text,
        }
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        let field = iced::widget::text_input("Search", &self.text)
            .width(Length::Fill)
            .font(Font {
                family: iced::font::Family::Name("SF Pro Text"),
                weight: iced::font::Weight::Bold,
                stretch: Default::default(),
                monospaced: false,
            })
            .style(iced::theme::TextInput::Custom(Box::new(SearchInputStyles)))
            .padding(8.0)
            .on_input(Message::InputChanged);

        Container::new(field)
            .center_x()
            .center_y()
            .height(Length::Fill)
            .width(Length::Fill)
            .into()
    }

    fn style(&self) -> <Self::Theme as application::StyleSheet>::Style {
        theme::Application::Custom(Box::new(ApplicationStyles))
    }
}

struct ApplicationStyles;

impl application::StyleSheet for ApplicationStyles {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> application::Appearance {
        application::Appearance {
            background_color: Color::TRANSPARENT,
            text_color: Color::TRANSPARENT,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct SearchInputStyles;

impl text_input::StyleSheet for SearchInputStyles {
    type Style = iced::Theme;

    fn active(&self, _: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: Background::Color(Color::TRANSPARENT),
            border_radius: BorderRadius::from(0.0),
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            icon_color: Default::default(),
        }
    }

    fn focused(&self, style: &Self::Style) -> text_input::Appearance {
        self.active(style)
    }

    fn placeholder_color(&self, _: &Self::Style) -> Color {
        Color {
            r: 0.25490198,
            g: 0.25490198,
            b: 0.25490198,
            a: 1.0,
        }
    }

    fn value_color(&self, _: &Self::Style) -> Color {
        Color::WHITE
    }

    fn disabled_color(&self, _style: &Self::Style) -> Color {
        Color::TRANSPARENT
    }

    fn selection_color(&self, _: &Self::Style) -> Color {
        Color {
            r: 0.454_901_9,
            g: 0.643_137_3,
            b: 0.737_254_9,
            a: 1.0,
        }
    }

    fn disabled(&self, _style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: Background::Color(Color::TRANSPARENT),
            border_radius: BorderRadius::from(0.0),
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            icon_color: Default::default(),
        }
    }

    fn hovered(&self, style: &Self::Style) -> text_input::Appearance {
        self.focused(style)
    }
}

fn main() -> iced::Result {

    let platform_window_settings = if cfg!(target_os="macos") {
        PlatformSpecific {
            title_hidden: true,
            titlebar_transparent: true,
            fullsize_content_view: true,
        }
    } else {
        Default::default()
    };

    let window_settings = iced::window::Settings {
        size: (800, 50),
        position: Default::default(),
        min_size: None,
        max_size: None,
        visible: true,
        resizable: false,
        decorations: true,
        transparent: true,
        level: Level::AlwaysOnTop,
        icon: None,
        enabled_buttons: WindowButtons::empty(),
        platform_specific: platform_window_settings
    };

    CommandBar::run(Settings {
        id: None,
        window: window_settings,
        flags: Default::default(),
        default_font: Default::default(),
        default_text_size: 18.0,
        antialiasing: true,
        exit_on_close_request: true,
    })
}
