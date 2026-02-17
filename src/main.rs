
use std::io;
use std::io::{stdout, Write};

#[derive(Debug, Copy, Clone)]
struct Complex {
    x: f64,
    y: f64,
}

impl Complex {

    fn mult(p1: &Complex, p2: &Complex) -> Complex {
        let Complex { x: x1 , y: y1 } = p1;
        let Complex { x: x2 , y: y2 } = p2;
        Complex { x: x1 * x2 - y1 * y2, y: x1 * y2 + y1 * x2}
    }

    fn add(p1: &Complex, p2: &Complex) -> Complex {
        Complex { x: p1.x + p2.x, y: p1.y + p2.y}
    }

    fn abs(&self) -> f64 {
        f64::sqrt(self.x * self.x + self.y * self.y)
    }
}


fn mandlebrot(max_iter: i16, z: &Complex, c: &Complex) -> i16 {

    let mut num_iter: i16 = 0;
    let mut z = *z;

    while num_iter < max_iter && z.abs() < 2.0f64 {
        num_iter += 1;
        z = Complex::mult(&z, &z);
        z = Complex::add(&z, c);
    }

    num_iter

}

struct MandleSettings {
    size_x: i32,
    size_y: i32, 
    scale_x: f64,
    scale_y: f64,
    max_iterations: i16,
    offset: Complex
}

fn graph_mandle(settings: MandleSettings) {

    let MandleSettings { size_x, size_y, scale_x, scale_y, max_iterations, offset } = settings;

    for a_y in -size_y..size_y {
        for a_x in -size_x..size_x {
            let z_num = Complex { x: ((a_x as f64 / size_x as f64) - offset.x ) * scale_x, 
                                           y: ((a_y as f64 / size_y as f64) - offset.y ) * scale_y };
            let escape_time = mandlebrot(max_iterations, &z_num, &z_num);

            if escape_time > max_iterations - 1 {
                print!("1");
            }
            else {
                print!(" ");
            }
        }
        print!("\n")
        
    }

}

fn main() {

    let mut size_x = 70;
    let mut size_y = 30;
    let mut scale_x = 2.0;
    let mut scale_y = 2.0;
    let mut offset = Complex { x: 0.0, y: 0.0 };

    let mut max_iterations = 20;

    let move_amount = 0.1;

    loop {
        let mut user_input = String::new();
        io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");
        
        user_input = user_input.trim().to_string();
        
        for command in user_input.chars() {
            match command {
                'o' => {max_iterations += 1},
                'l' => {max_iterations -= 1},
                'i' => {size_x += 10},
                'k' => {size_x -= 10},
                'u' => {size_y += 10},
                'j' => {size_y -= 10},
                'y' => {scale_x = scale_x * 2.0},
                'h' => {scale_x = scale_x / 2.0},
                't' => {scale_y = scale_y * 2.0},
                'g' => {scale_y = scale_y / 2.0},
                'w' => {offset.y += move_amount},
                's' => {offset.y -= move_amount},
                'a' => {offset.x += move_amount},
                'd' => {offset.x -= move_amount},
                _ => {}
            }
        }

        if max_iterations < 1 {
            max_iterations = 1;
        }

        let settings = MandleSettings { size_x, size_y, scale_x, scale_y, max_iterations, offset };
        print!("\x1B[2J\x1B[1;1H");
        stdout().flush().expect("Failed to flush stdout");

        graph_mandle(settings);


    }

}