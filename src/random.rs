use crate::component::*;
use rand::Rng;

pub struct Random {
    matrix: Vec<Vec<iced::Color>>,
}

impl Random {
    fn get_color(rng: &mut rand::rngs::ThreadRng) -> iced::Color {
        let coef = rng.gen_range(0..128) as f32 * 0.1;
        let formula = |offset| f32::sin(coef + offset) * 127.0;
        let joy_offset = 100;

        let r = formula(0.0) as u8 + joy_offset;
        let g = formula(2.0) as u8 + joy_offset;
        let b = formula(4.0) as u8 + joy_offset;
        iced::Color::from_rgb8(r, g, b)
    }
}

impl Component for Random {
    fn init(&mut self) {
        println!("dd")
    }

    fn set_grid(&mut self, x_squares: usize, y_squares: usize) {
        let mut rng = rand::thread_rng();

        for _x in 0..x_squares {
            let mut v = Vec::new();
            for _y in 0..y_squares {
                v.push(Random::get_color(&mut rng))
            }
            self.matrix.push(v);
        }
    }

    fn get_tile(&self, x: usize, y: usize) -> Option<iced::Color> {
        self.matrix.get(x)?.get(y).cloned()
    }

    fn set_time(&mut self, _time: OffsetDateTime) {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0..self.matrix.len());
        let y = rng.gen_range(0..self.matrix[0].len());

        self.matrix[x][y] = Random::get_color(&mut rng);
    }

    fn key_event(&mut self, _event: iced::keyboard::Event) {}
}

impl Default for Random {
    fn default() -> Self {
        Random { matrix: Vec::new() }
    }
}
