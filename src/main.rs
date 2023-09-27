#![allow(arithmetic_overflow)]

mod clock;
mod component;
mod dvd;
mod random;
mod simple;

use crate::clock::Clock;
use crate::component::*;
use crate::dvd::Dvd;
use crate::random::Random;
use crate::simple::Simple;

use iced::keyboard::KeyCode;
use iced::theme::Palette;
use iced::{window, Application, Command, Element, Rectangle};
use iced_native::renderer::Quad;
use iced_native::Widget;

#[derive(Debug, Copy, Clone)]
struct RenderData {
    x_offset: f32,
    y_offset: f32,
    x_squares: usize,
    y_squares: usize,
}

impl RenderData {
    pub fn new(width: f32, height: f32, size: f32, gap: f32) -> Self {
        let x_squares = (width / (gap + size)) as usize;
        let x_offset = ((width - x_squares as f32 * (gap + size)) / 2.0).ceil();

        let y_squares = (height / (gap + size)) as usize;
        let y_offset = ((height - y_squares as f32 * (gap + size)) / 2.0).ceil();

        RenderData {
            x_offset,
            y_offset,
            x_squares,
            y_squares,
        }
    }
}

struct Surface<'a> {
    pub gap: f32,
    pub radius: f32,
    pub size: f32,
    pub comps: &'a [Box<dyn Component>],
    pub rd: Option<RenderData>,
}

impl<'a> Surface<'a> {
    fn quad(&self, x: f32, y: f32) -> Quad {
        Quad {
            bounds: Rectangle {
                x,
                y,
                width: self.size,
                height: self.size,
            },
            border_radius: self.radius.into(),
            border_width: 0.0,
            border_color: iced::Color::TRANSPARENT,
        }
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer> for Surface<'a>
where
    Renderer: iced_native::Renderer,
{
    fn width(&self) -> iced::Length {
        iced::Length::Fill
    }

    fn height(&self) -> iced::Length {
        iced::Length::Fill
    }

    fn layout(
        &self,
        _renderer: &Renderer,
        limits: &iced_native::layout::Limits,
    ) -> iced_native::layout::Node {
        iced_native::layout::Node::new(limits.max())
    }

    fn draw(
        &self,
        _state: &iced_native::widget::Tree,
        renderer: &mut Renderer,
        _theme: &<Renderer as iced_native::Renderer>::Theme,
        _style: &iced_native::renderer::Style,
        _layout: iced_native::Layout<'_>,
        _cursor_position: iced::Point,
        _viewport: &iced::Rectangle,
    ) {
        if self.rd.is_none() {
            return;
        }

        let rd = self.rd.unwrap();

        for c in self.comps {
            (0..rd.x_squares).for_each(|i| {
                (0..rd.y_squares).for_each(|j| {
                    let x_gap = if i == 0 { self.gap / 2.0 } else { self.gap };
                    let y_gap = if j == 0 { self.gap / 2.0 } else { self.gap };
                    let x = (self.size + x_gap) * i as f32 + rd.x_offset;
                    let y = (self.size + y_gap) * j as f32 + rd.y_offset;
                    renderer.fill_quad(
                        self.quad(x, y),
                        c.get_tile(i, j).unwrap_or(iced::Color::TRANSPARENT),
                    )
                    // renderer.fill_quad(self.quad(x, y), iced::Color::from_rgb8(32, 64, 128))
                });
            });
        }
    }
}

impl<'a, Message, Renderer> From<Surface<'a>> for Element<'a, Message, Renderer>
where
    Renderer: iced_native::Renderer,
{
    fn from(surface: Surface<'a>) -> Self {
        Self::new(surface)
    }
}

#[derive(Debug, Clone)]
enum Message {
    Exit,
    Ignored,
    Tick(time::OffsetDateTime),
    KeyboardEvent(iced::keyboard::Event),
    LayoutUpdate(f32, f32),
}

struct App {
    comps: Vec<Box<dyn Component>>,
    render_data: Option<RenderData>,
    gaps: f32,
    radius: f32,
    size: f32,
}

impl Application for App {
    type Message = Message;
    type Theme = iced::Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            App::default(),
            iced::window::change_mode(iced::window::Mode::Fullscreen),
            // Command::none(),
        )
    }

    fn title(&self) -> String {
        "azlock".to_owned()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Exit => window::close(),
            Message::LayoutUpdate(w, h) => {
                if self.render_data.is_none() {
                    for c in self.comps.iter_mut() {
                        c.init();
                    }
                }
                let rd = RenderData::new(w, h, self.size, self.gaps);
                self.render_data = Some(rd);
                for c in self.comps.iter_mut() {
                    c.set_grid(rd.x_squares, rd.y_squares);
                }
                Command::none()
            }
            Message::Tick(t) => {
                for c in self.comps.iter_mut() {
                    c.set_time(t)
                }
                Command::none()
            }
            Message::KeyboardEvent(k) => {
                for c in self.comps.iter_mut() {
                    c.key_event(k);
                }
                Command::none()
            }
            _ => Command::none(),
        }
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        let tick = iced::time::every(std::time::Duration::from_millis(50)).map(|_| {
            Message::Tick(
                time::OffsetDateTime::now_local()
                    .unwrap_or_else(|_| time::OffsetDateTime::now_utc()),
            )
        });

        let events = iced::subscription::events().map(|e| match e {
            iced::Event::Keyboard(k) => match k {
                iced::keyboard::Event::KeyReleased {
                    key_code,
                    modifiers: _,
                } => {
                    if key_code == KeyCode::Q {
                        Message::Exit
                    } else {
                        Message::KeyboardEvent(k)
                    }
                }
                _ => Message::KeyboardEvent(k),
            },
            iced::Event::Window(window::Event::Resized { width, height }) => {
                Message::LayoutUpdate(width as f32, height as f32)
            }
            _ => Message::Ignored,
        });
        iced::Subscription::batch(vec![tick, events])
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        Surface {
            gap: self.gaps,
            size: self.size,
            radius: self.radius,
            comps: &self.comps,
            rd: self.render_data,
        }
        .into()
    }

    fn theme(&self) -> Self::Theme {
        iced::Theme::custom(Palette {
            background: iced::Color::TRANSPARENT,
            primary: iced::Color::TRANSPARENT,
            text: iced::Color::TRANSPARENT,
            ..Palette::LIGHT
        })
    }
}

impl Default for App {
    fn default() -> Self {
        App {
            comps: vec![Box::new(Random::default()), Box::new(Clock::default())],
            render_data: None,
            radius: 3.0,
            gaps: 2.0,
            size: 19.0,
        }
    }
}

fn main() {
    _ = App::run(iced::Settings {
        // exit_on_close_request: false,
        window: iced::window::Settings {
            transparent: true,
            decorations: true,
            ..Default::default()
        },
        ..iced::Settings::default()
    });
}
