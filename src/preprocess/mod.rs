use opencv::imgcodecs::{
    imwrite,
    imread
};

use anyhow::Result;

use opencv::{
    prelude::*,
    core,
};

use std::path::Path;

pub mod denoise;
pub mod deskew;

/*
pub trait SavableImage {
    fn save_now<Q>(&self, path: Q) -> ImageResult<()>
        where
        Q: AsRef<Path>;
}


impl SavableImage for GrayImage {
    fn save_now<Q>(&self, path: Q) -> ImageResult<()>
        where
        Q: AsRef<Path>,
    {
        self.save(path)
    }
}

impl SavableImage for DynamicImage {
    fn save_now<Q>(&self, path: Q) -> ImageResult<()>
        where
        Q: AsRef<Path>,
    {
        self.save(path)
    }
}




pub fn save_image<T: SavableImage> ( image: T, path: &Path) -> ImageResult<()> {
    match image.save_now(path) {
        Ok(_) => Ok(()),
        Err(err) => {
            println!("Unable to save image! {}", path.to_string_lossy());
            Err(err)
        },
    }
}


pub fn open_image(path: &Path) -> Result<DynamicImage, Box<dyn Error>> {
    // Open the image file
    let image = match image::open(path) {
        Ok(image) => image,
        Err(err) => {
            println!("Unable to open image!");
            return Err(Box::new(err))
        },
    };

    Ok(image)
}
*/

pub fn open_image(path: &Path) -> Result<Mat, opencv::Error> {
    // Open the image file
    let image = match imread(path.to_str().unwrap(), 0) {
        Ok(image) => image,
        Err(err) => {
            println!("Unable to open image: '{}'!", path.to_string_lossy());
            return Err(err)
        },
    };

    Ok(image)
}

pub fn save_image(image: &Mat, path: &Path) -> Result<(), opencv::Error> {
    // Open the image file
    match imwrite(path.to_str().unwrap(), &image, &core::Vector::new()) {
        Ok(_) => Ok(()),
        Err(err) => {
            println!("Unable to save image! {}", path.to_string_lossy());
            Err(err)
        },
    }
}