use image::{RgbImage, Rgb};

const WIDTH: u32 = 400;
const HEIGHT: u32 = 400;

fn main() {
    println!("Generating image ...");

    let mut img = RgbImage::new(WIDTH, HEIGHT);

    let mint = Rgb([48u8, 176, 120]); // comes from a RGB color picker
    let red = Rgb([255u8, 0, 0]);     // comes from a RGB color picker
    let mut stripe = false;

    for y in 0..HEIGHT {
        if y % 10 == 0 {
            stripe = !stripe;
        }
        for x in 0..WIDTH {
            if stripe {
                img.put_pixel(x, y, mint);
            } else {
                img.put_pixel(x, y, red);
            }
        }
    }

    img.save("out.png").unwrap();
    println!("Done.");
}
