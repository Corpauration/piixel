use crate::component::*;

pub struct Dvd {
    x: usize,
    y: usize,
    x_speed: i32,
    y_speed: i32,
    x_squares: usize,
    y_squares: usize,
    mili: usize,
}

impl Dvd {
    pub fn new() -> Self {
        Dvd {
            x: 0,
            y: 0,
            x_speed: 1,
            y_speed: 1,
            x_squares: 0,
            y_squares: 0,
            mili: 0,
        }
    }
}

impl Component for Dvd {
    fn init(&mut self) {}

    fn set_grid(&mut self, x_squares: usize, y_squares: usize) {
        self.x_squares = x_squares;
        self.y_squares = y_squares;

        self.x = x_squares / 2;
        self.y = y_squares / 2;
    }

    fn set_time(&mut self, time: OffsetDateTime) {
        let fps = 7;
        let mili = time.millisecond() / (1000 / fps);

        if self.mili != mili as usize {
            self.mili = (self.mili + 1) % fps as usize;

            if self.x + 2 == self.x_squares || self.x == 0 {
                self.x_speed = -self.x_speed;
            }

            if self.y + 2 == self.y_squares || self.y == 0 {
                self.y_speed = -self.y_speed;
            }

            self.x = (self.x as i32 + self.x_speed) as usize;
            self.y = (self.y as i32 + self.y_speed) as usize;
        }
    }

    fn key_event(&mut self, _event: iced::keyboard::Event) {}

    fn get_tile(&self, x: usize, y: usize) -> Option<iced::Color> {
        if (x == self.x || x == self.x + 1) && (y == self.y || y == self.y + 1) {
            Some(iced::Color::WHITE)
        } else {
            Some(iced::Color::TRANSPARENT)
        }
    }
}

impl Default for Dvd {
    fn default() -> Self {
        Dvd::new()
    }
}
