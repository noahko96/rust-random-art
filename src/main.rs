use png::{BitDepth, ColorType, Encoder, ScaledFloat, SourceChromaticities};
use rand::Rng;
use std::{fs::File, io::BufWriter, path::Path};

fn main() {
    let path = Path::new(r"example.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = Encoder::new(w, 800, 600);
    encoder.set_color(ColorType::Rgb);
    encoder.set_depth(BitDepth::Eight);
    encoder.set_source_gamma(ScaledFloat::from_scaled(45455));
    let source_chromaticities = SourceChromaticities::new(
        (0.31270, 0.32900),
        (0.64000, 0.33000),
        (0.30000, 0.60000),
        (0.15000, 0.06000)
    );
    encoder.set_source_chromaticities(source_chromaticities);
    let mut writer = encoder.write_header().unwrap();

    let mut vec = Vec::with_capacity(1_440_000);

    let bg1 = random_color();
    let bg2 = random_color();
    let bg3 = random_color();
    let bg4 = random_color();
    let bg5 = random_color();
    let bg6 = random_color();

    for i in 0..1_800 {
        let (background, num) = match i {
            0..=899 => {
                match i {
                    _ if i % 3 == 0 => (bg1, 267),
                    _ if i % 3 == 1 => (bg2, 267),
                    _ => (bg3, 266),
                }
            },
            _ => {
                match i {
                    _ if i % 3 == 0 => (bg4, 267),
                    _ if i % 3 == 1 => (bg5, 267),
                    _ => (bg6, 266),
                }
            },
        };

        let (bgr, bgg, bgb) = background;

        for _j in 0..num {
            vec.push(bgr);
            vec.push(bgg);
            vec.push(bgb);
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
