use image::io::Reader as ImageReader;
use std::env;

use image::{DynamicImage, EncodableLayout}; // Using image crate: https://github.com/image-rs/image
use webp::{Encoder, WebPMemory}; // Using webp crate: https://github.com/jaredforth/webp

use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    // 引数を取得
    let args: Vec<String> = env::args().collect();
    let input_path = &args[1];
    // let quality = &args[2].parse::<f32>().unwrap_or_else(|_| 65f32);
    let args2 = &args.get(2);
    let quality;

    match args2 {
        Some(n) => {
            quality = n.parse::<f32>().unwrap();
        }
        None => {
            quality = 65f32;
        }
    }

    let current_dir = env::current_dir().unwrap();
    // let file_name: String = "./ani.jpg".to_string();
    let file_name: String = format!("{}\\{}", current_dir.to_str().unwrap(), input_path);
    let webp_path = image_to_webp(&file_name, quality);

    println!("{} -> {}", file_name, webp_path.unwrap());
}

fn image_to_webp(file_path: &String, quality: f32) -> Option<String> {
    // Open path as DynamicImage
    //let image: DynamicImage = ImageReader::open(file_path).unwrap().decode().unwrap();
    let image = ImageReader::open(file_path);
    let image: DynamicImage = match image {
        Ok(img) => img.with_guessed_format().unwrap().decode().unwrap(), //ImageReader::with_guessed_format() function guesses if image needs to be opened in JPEG or PNG format.
        Err(e) => {
            println!("Error: {}", e);
            return None;
        }
    };

    // Make webp::Encoder from DynamicImage.
    let encoder: Encoder = Encoder::from_image(&image).unwrap();

    // Encode image into WebPMemory.
    let encoded_webp: WebPMemory = encoder.encode(quality);

    // Put webp-image in a separate webp-folder in the location of the original image.
    let path: &Path = Path::new(file_path);

    // Get filename of original image.
    let filename_original_image = path.file_stem().unwrap().to_str().unwrap();

    // Make full output path for webp-image.
    let webp_image_path = format!("{}.webp", filename_original_image);

    // Make File-stream for WebP-result and write bytes into it, and save to path "output.webp".
    let mut webp_image = File::create(webp_image_path.to_string()).unwrap();
    match webp_image.write_all(encoded_webp.as_bytes()) {
        Err(e) => {
            println!("Error: {}", e);
            return None;
        }
        Ok(_) => return Some(webp_image_path),
    }
}
