// Logging
extern crate pretty_env_logger;
#[macro_use]
extern crate log;

fn main() {
    pretty_env_logger::init_timed();

    const WIDTH: usize = 5_000;
    const EPOCHS: usize = 5_000;

    const PRINT_TO_CONSOLE: bool = false;
    const GENERATE_IMAGE: bool = true;

    let mut timeline: Vec<[bool; WIDTH]> = Vec::new();

    info!("Generating Epochs.");
    for epoch in 0..EPOCHS {
        trace!("Generating Epoch {} / {}", epoch + 1, EPOCHS);
        let mut current_generation: [bool; WIDTH] = [false; WIDTH];
        if epoch == 0 {
            current_generation = [false; WIDTH];
            current_generation[(WIDTH as f32 / 2 as f32).floor() as usize] = true;
            timeline.push(current_generation);
        } else {
            let last_generation = timeline.last().unwrap();
            for i in 0..WIDTH {
                // Edge Cases
                let left: bool;
                let right: bool;

                if i == 0 {
                    left = false;
                } else {
                    left = last_generation[i - 1];
                }
                if i == WIDTH - 1 {
                    right = false;
                } else {
                    right = last_generation[i + 1];
                }

                if left == right {
                    current_generation[i] = false;
                } else {
                    current_generation[i] = true;
                }
            }
            timeline.push(current_generation);
        }
    }

    if PRINT_TO_CONSOLE {
        for i in timeline.iter() {
            println!("");
            for j in i.iter() {
                if *j == true {
                    print!("X");
                } else {
                    print!(" ");
                }
            }
            println!("");
        }
    }

    info!("Generating Image.");
    if GENERATE_IMAGE {
        use image::*;

        let mut imgbuf: ImageBuffer<Rgb<u8>, Vec<u8>> =
            image::ImageBuffer::new(WIDTH as u32, EPOCHS as u32);

        let mut last_y: i128 = -1;

        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            if y as i128 != last_y {
                trace!("Generating Image Line {} / {}", y + 1, EPOCHS);
            }
            if timeline[y as usize][x as usize] {
                *pixel = image::Rgb([255, 255, 255]);
            } else {
                *pixel = image::Rgb([0, 0, 0]);
            }
            last_y = y as i128;
        }

        info!("Saving Image to disk. This might take a while!");

        imgbuf.save("image.png").unwrap();

        info!("Image saved to disk!");
    }
}
