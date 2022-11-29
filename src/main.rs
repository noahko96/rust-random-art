use png::{BitDepth, ColorType, Encoder};
use rand::Rng;
use std::{fs::File, io::BufWriter, path::Path};

fn main() {
    let path = Path::new(r"example.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = Encoder::new(w, 800, 600);
    encoder.set_color(ColorType::Rgb);
    encoder.set_depth(BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    let mut vec = Vec::with_capacity(1_440_000);

    let info1 = (random_color(), random_color(), random_color(), 164, 112);
    let info2 = (random_color(), random_color(), random_color(), 164, 379);
    let info3 = (random_color(), random_color(), random_color(), 164, 645);
    let info4 = (random_color(), random_color(), random_color(), 464, 112);
    let info5 = (random_color(), random_color(), random_color(), 464, 379);
    let info6 = (random_color(), random_color(), random_color(), 464, 645);

    let big_radius = 66;
    let big_radius_squared = big_radius * big_radius;
    let small_radius = 32;
    let small_radius_squared = small_radius * small_radius;

    for row in 0..600 {
        for col in 0..800 {
            let (background, big_circle, small_circle, center_row, center_col) = match (row, col) {
                (0..=299, 0..=266) => info1,
                (0..=299, 267..=533) => info2,
                (0..=299, _) => info3,
                (_, 0..=266) => info4,
                (_, 267..=533) => info5,
                _ => info6,
            };

            let big_squared_sum = squared_sum(row, col, center_row, center_col);

            let center_row = center_row - 70;
            let center_col = center_col + 90;

            let small_squared_sum = squared_sum(row, col, center_row, center_col);

            let (r, g, b) = if big_squared_sum <= big_radius_squared {
                big_circle
            } else if small_squared_sum <= small_radius_squared {
                small_circle
            } else {
                background
            };

            vec.push(r);
            vec.push(g);
            vec.push(b);
        }
    }

    writer.write_image_data(&vec).unwrap();
}

fn random_sample() -> u8 {
    rand::thread_rng().gen_range(0..=255)
}

fn random_color() -> (u8, u8, u8) {
    (random_sample(), random_sample(), random_sample())
}

fn squared_sum(row: i32, col: i32, center_row: i32, center_col: i32) -> i32 {
    let row_diff = row - center_row;
    let col_diff = col - center_col;

    (row_diff * row_diff) + (col_diff * col_diff)
}
