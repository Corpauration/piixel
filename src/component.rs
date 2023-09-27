pub use time::OffsetDateTime;

pub trait Component {
    fn init(&mut self);

    fn set_grid(&mut self, x_squares: usize, y_squares: usize);

    fn set_time(&mut self, time: OffsetDateTime);

    fn key_event(&mut self, event: iced::keyboard::Event);

    fn get_tile(&self, x: usize, y: usize) -> Option<iced::Color>;
}

pub struct Reverse {
    x_squares: usize,
    y_squares: usize,
    comp: Box<dyn Component>,
}

impl Reverse {
    pub fn new(comp: Box<dyn Component>) -> Self {
        Reverse {
            x_squares: 0,
            y_squares: 0,
            comp,
        }
    }
}

impl Component for Reverse {
    fn init(&mut self) {
        self.comp.init();
    }

    fn set_grid(&mut self, x_squares: usize, y_squares: usize) {
        self.x_squares = x_squares - 1;
        self.y_squares = y_squares - 1;
        self.comp.set_grid(x_squares, y_squares);
    }

    fn set_time(&mut self, time: OffsetDateTime) {
        self.comp.set_time(time);
    }

    fn key_event(&mut self, event: iced::keyboard::Event) {
        self.comp.key_event(event)
    }

    fn get_tile(&self, x: usize, y: usize) -> Option<iced::Color> {
        self.comp.get_tile(self.x_squares - x, self.y_squares - y)
    }
}

pub struct BNWFilter {
    comp: Box<dyn Component>,
}

impl BNWFilter {
    pub fn new(comp: Box<dyn Component>) -> Self {
        BNWFilter { comp }
    }
}

impl Component for BNWFilter {
    fn init(&mut self) {
        self.comp.init();
    }

    fn set_grid(&mut self, x_squares: usize, y_squares: usize) {
        self.comp.set_grid(x_squares, y_squares);
    }

    fn set_time(&mut self, time: OffsetDateTime) {
        self.comp.set_time(time);
    }

    fn key_event(&mut self, event: iced::keyboard::Event) {
        self.comp.key_event(event)
    }

    fn get_tile(&self, x: usize, y: usize) -> Option<iced::Color> {
        self.comp.get_tile(x, y).map(|c| {
            let sum = (c.r + c.g + c.b) / 3.0;
            iced::Color::from_rgba(sum, sum, sum, c.a)
        })
    }
}
