use crate::component::*;

static ZERO: [[bool; 3]; 5] = [
    [true, true, true],
    [true, false, true],
    [true, false, true],
    [true, false, true],
    [true, true, true],
];

static ONE: [[bool; 3]; 5] = [
    [true, true, false],
    [false, true, false],
    [false, true, false],
    [false, true, false],
    [true, true, true],
];

static TWO: [[bool; 3]; 5] = [
    [true, true, true],
    [false, false, true],
    [true, true, true],
    [true, false, false],
    [true, true, true],
];

static THREE: [[bool; 3]; 5] = [
    [true, true, true],
    [false, false, true],
    [true, true, true],
    [false, false, true],
    [true, true, true],
];

static FOUR: [[bool; 3]; 5] = [
    [true, false, true],
    [true, false, true],
    [true, true, true],
    [false, false, true],
    [false, false, true],
];

static FIVE: [[bool; 3]; 5] = [
    [true, true, true],
    [true, false, false],
    [true, true, true],
    [false, false, true],
    [true, true, true],
];

static SIX: [[bool; 3]; 5] = [
    [true, true, true],
    [true, false, false],
    [true, true, true],
    [true, false, true],
    [true, true, true],
];

static SEVEN: [[bool; 3]; 5] = [
    [true, true, true],
    [false, false, true],
    [false, false, true],
    [false, false, true],
    [false, false, true],
];

static EIGHT: [[bool; 3]; 5] = [
    [true, true, true],
    [true, false, true],
    [true, true, true],
    [true, false, true],
    [true, true, true],
];

static NINE: [[bool; 3]; 5] = [
    [true, true, true],
    [true, false, true],
    [true, true, true],
    [false, false, true],
    [true, true, true],
];

fn digit_to_matrix(i: u8) -> &'static [[bool; 3]; 5] {
    match i {
        0 => &ZERO,
        1 => &ONE,
        2 => &TWO,
        3 => &THREE,
        4 => &FOUR,
        5 => &FIVE,
        6 => &SIX,
        7 => &SEVEN,
        8 => &EIGHT,
        9 => &NINE,
        _ => &ZERO,
    }
}

pub struct Clock {
    time: OffsetDateTime,
    x_offset: i32,
    real_xoffset: usize,
    y_offset: i32,
    real_yoffset: usize,
}

impl Clock {
    pub fn new(x_offset: i32, y_offset: i32) -> Self {
        Clock {
            time: OffsetDateTime::UNIX_EPOCH,
            x_offset,
            real_xoffset: 0,
            y_offset,
            real_yoffset: 0,
        }
    }
}

impl Component for Clock {
    fn init(&mut self) {}

    fn set_grid(&mut self, x_squares: usize, y_squares: usize) {
        self.real_xoffset = if self.x_offset.is_positive() || self.x_offset == 0 {
            self.x_offset as usize
        } else {
            (x_squares as i32 + self.x_offset - 5) as usize
        };

        self.real_yoffset = if self.y_offset.is_positive() || self.y_offset == 0 {
            self.y_offset as usize
        } else {
            (y_squares as i32 + self.y_offset - 4) as usize
        };
    }

    fn set_time(&mut self, time: OffsetDateTime) {
        self.time = time;
    }

    fn get_tile(&self, x: usize, y: usize) -> Option<iced::Color> {
        if (y as i32 - self.real_yoffset as i32) >= 0
            && y - self.real_yoffset < 5
            && x >= self.real_xoffset
        {
            let x = x - self.real_xoffset;
            let y = y - self.real_yoffset;
            if x < 3 {
                if digit_to_matrix(self.time.hour() / 10)[y][x] {
                    return Some(iced::Color::WHITE);
                }
            }

            if x > 3 && x < 7 {
                if digit_to_matrix(self.time.hour() % 10)[y][x - 4] {
                    return Some(iced::Color::WHITE);
                }
            }

            if x > 7 && x < 11 {
                if digit_to_matrix(self.time.minute() / 10)[y][x - 8] {
                    return Some(iced::Color::WHITE);
                }
            }

            if x > 11 && x < 15 {
                if digit_to_matrix(self.time.minute() % 10)[y][x - 12] {
                    return Some(iced::Color::WHITE);
                }
            }

            if x > 15 && x < 19 {
                if digit_to_matrix(self.time.second() / 10)[y][x - 16] {
                    return Some(iced::Color::WHITE);
                }
            }

            if x > 19 && x < 23 {
                if digit_to_matrix(self.time.second() % 10)[y][x - 20] {
                    return Some(iced::Color::WHITE);
                }
            }
        }
        Some(iced::Color::TRANSPARENT)
    }

    fn key_event(&mut self, _event: iced::keyboard::Event) {}
}

impl Default for Clock {
    fn default() -> Self {
        Clock::new(1, -2)
    }
}
