use macroquad::prelude::*;

const AMPLITUDE: f64 = 120.0;
const WIDTH: f64 = 900.0;
const HEIGHT: f64 = 600.0;

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
    let mut angle: f64 = 0.0;

    loop {
        clear_background(BLACK);

        draw_text("No-Nonsense Programming Animation", 120.0, 30.0, 20.0, LIGHTGRAY);

        // draw anything here
        let x = AMPLITUDE * angle.sin() + WIDTH / 2.0;
        let y = AMPLITUDE * angle.cos() + HEIGHT / 2.0;
        angle += 0.04;
        draw_circle(x as f32, y as f32, 50.0, RED);

        let x = AMPLITUDE * 0.5 * (angle - 3.0).sin() + WIDTH / 2.0;
        let y = AMPLITUDE * 0.5 * (angle - 3.0).cos() + HEIGHT / 2.0;
        draw_circle(x as f32, y as f32, 20.0, BLUE);

        let x = AMPLITUDE * 0.2 * (angle - 4.0).sin() + WIDTH / 2.0;
        let y = AMPLITUDE * 0.2 * (angle - 4.0).cos() + HEIGHT / 2.0;
        draw_circle(x as f32, y as f32, 12.0, GREEN);

        next_frame().await;
    }
}
