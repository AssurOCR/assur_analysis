use image::{DynamicImage, GrayImage, ImageFormat};
use std::error::Error;
use std::path::Path;
use imageproc::contrast::otsu_level;

pub fn clean_page(image: &DynamicImage) -> GrayImage {
    let gimage = image.to_luma8();

    println!("Calculating threshold!");
    // Apply Otsu threshold to the image
    let threshold = otsu_level(&gimage);


    println!("Applying threshold");

    imageproc::contrast::threshold(&gimage, threshold)
}


