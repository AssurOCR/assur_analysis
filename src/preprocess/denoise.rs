use opencv::{
    prelude::*,
};

use opencv::photo::{
    fast_nl_means_denoising
};

pub fn clean_page(image: &Mat) -> Mat {
    let mut denoised_image = Mat::default();
    fast_nl_means_denoising(image, &mut denoised_image, 30.0, 7, 21).unwrap();

    denoised_image
}