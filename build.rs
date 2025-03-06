use std::{fs, path::Path};

const OUT_DIR: &str = "assets/output";

fn load_font() {
    let jost_font_bytes = include_bytes!("assets/Jost.ttf");
    let jost_encoded = rbase64::encode(jost_font_bytes);
    let font_out_path = Path::new(&OUT_DIR).join("Jost.ttf.bin");
    fs::write(font_out_path, jost_encoded).expect("WRITE FAILURE: font");
}

fn load_logo() {
    let logo_bytes = include_bytes!("assets/logobanner.png");
    let logo_encoded = rbase64::encode(logo_bytes);
    let logo_out_path = Path::new(&OUT_DIR).join("logobanner.png.bin");
    fs::write(logo_out_path, logo_encoded).expect("WRITE FAILURE: logo");
}

fn main() {
    load_font();
    load_logo();
}