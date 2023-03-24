use image::imageops::overlay;
use image::{DynamicImage, GenericImage, GenericImageView, ImageBuffer, RgbImage};
use serde_json::json;
pub fn generate(pathToSaveTo: &str, file_names: Vec<String>) {
    let mut first_image = image::open(&file_names[0]).unwrap();
    let mut combined_image = DynamicImage::new_rgba8(first_image.width(), first_image.height());
    combined_image.copy_from(&first_image, 0, 0).unwrap();
    for i in 1..file_names.len() {
        let next_image = image::open(&file_names[i]).unwrap();
        overlay(&mut combined_image, &next_image, 0, 0);
    }

    let save_operation = combined_image.save_with_format_and_density(pathToSaveTo, image::ImageFormat::Png, 150);
    match save_operation {
        Ok(_) => println!("{} generated", pathToSaveTo),
        Err(_) => println!("Image not saved {:?}", save_operation),
    }
}
