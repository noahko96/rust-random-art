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

    for row in 0..600 {
        for col in 0..800 {
            let background = match (row, col) {
                (0..=299, 0..=266) => bg1,
                (0..=299, 267..=533) => bg2,
                (0..=299, _) => bg3,
                (_, 0..=266) => bg4,
                (_, 267..=533) => bg5,
                _ => bg6,
            };

            let (bgr, bgg, bgb) = background;

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
