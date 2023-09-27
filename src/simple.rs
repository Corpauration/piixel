use crate::component::*;

pub struct Simple;

impl Component for Simple {
    fn init(&mut self) {}

    fn set_grid(&mut self, _x_squares: usize, _y_squares: usize) {}

    fn set_time(&mut self, _time: OffsetDateTime) {}

    fn key_event(&mut self, _event: iced::keyboard::Event) {}

    fn get_tile(&self, x: usize, y: usize) -> Option<iced::Color> {
        if x % 2 == 0 || y % 2 == 0 {
            Some(iced::Color::WHITE)
        } else {
            Some(iced::Color::BLACK)
        }
    }
}

impl Default for Simple {
    fn default() -> Self {
        Simple
    }
}
