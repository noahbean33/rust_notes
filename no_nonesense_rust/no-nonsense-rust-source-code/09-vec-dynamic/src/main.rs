use macroquad::prelude::*;
use macroquad::rand::gen_range;

const WIDTH: f64 = 900.0;
const HEIGHT: f64 = 600.0;

struct Circle {
    d: f64,
    angle: f64,
    speed: f64,
    radius: f64,
    color: Color,
}

fn generate_circle() -> Circle {
    Circle {
        d: gen_range(10.0, 200.0),
        angle: 0.0,
        speed: gen_range(1.0, 9.0),
        radius: gen_range(5.0, 40.0),
        color: Color::new(
            gen_range(0.0, 1.0),
            gen_range(0.0, 1.0),
            gen_range(0.0, 1.0),
            1.0,
        ),
    }
}

fn draw_circles(circles: &[Circle]) {
    for circle in circles {
        let x = circle.d * circle.angle.sin() + WIDTH / 2.0;
        let y = circle.d * circle.angle.cos() + HEIGHT / 2.0;
        draw_circle(x as f32, y as f32, circle.radius as f32, circle.color);
    }
}

fn move_circles(circles: &mut [Circle], dt: f64) {
    for circle in circles {
        circle.angle += circle.speed * dt;
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Graphics in Rust".to_string(),
        window_width: WIDTH as i32,
        window_height: HEIGHT as i32,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut circles: Vec<Circle> = Vec::new();
    circles.push(generate_circle());

    loop {
        // check if user presses keyboard:
        if is_key_pressed(KeyCode::Enter) {
            println!("Key pressed!");
            circles.push(generate_circle());
        }

        let dt = get_frame_time() as f64;
        move_circles(&mut circles, dt);

        clear_background(BLACK);
        draw_text("No-Nonsense Programming Animation", 120.0, 30.0, 20.0, LIGHTGRAY);
        // draw anything here
        draw_circles(&circles);

        next_frame().await;
    }
}
