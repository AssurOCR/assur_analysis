use anyhow::Result;

use opencv::{
    imgproc,
    prelude::*,
    core,
};

use opencv::core::{
    bitwise_not
};

use opencv::imgcodecs::{
    imwrite,
    imread
};

fn compute_skew(input_image: &Mat) -> Result<f64> {
    let mut image = Mat::default();
    bitwise_not(input_image, &mut image, &Mat::default())?;

    let size = image.size()?;
    let mut lines = opencv::types::VectorOfVec4i::new();


    imgproc::hough_lines_p(
        &image,
        &mut lines,
        1.0,
        core::CV_PI / 180.0,
        100,
        (size.width as f64) *0.6,
        80.0,
    )?;

    let mut disp_line = image.clone();

    let mut angle = 0.0;

    for line in lines.iter() {

        let temp_num = (line[3] - line[1]) as f64;

        let atan_num = temp_num.atan2((line[2] - line[0]) as f64);

        angle += atan_num;

        imgproc::line(
            &mut disp_line,
            core::Point::new(line[0], line[1]),
            core::Point::new(line[2], line[3]),
            core::Scalar::new(255.0, 0.0, 0.0, 0.0),
            5,
            8,
            0,
        )?;

        println!("Line: x1: {}, y1: {}, x2: {}, y2: {} | Angle: {}", line[0], line[1], line[2], line[3], atan_num);
    }

    imwrite("lines.jpeg", &disp_line, &core::Vector::new())?;

    angle /= lines.len() as f64;

    Ok(angle)
}


fn rotate(image_input: &Mat, angle: f64) -> Result<Mat> {


    let mut image = Mat::default();
    bitwise_not(&image_input, &mut image, &Mat::default())?;


    let mut rotated = Mat::default();

    let center = core::Point2f::new(image.cols() as f32 / 2.0, image.rows() as f32 / 2.0);

    let rotation_matrix = imgproc::get_rotation_matrix_2d(center, (angle * 180.0 / core::CV_PI), 1.0)?;

    imgproc::warp_affine(
        &image_input,
        &mut rotated,
        &rotation_matrix,
        image.size()?,
        imgproc::INTER_CUBIC,
        core::BORDER_REPLICATE,
        core::Scalar::default()
    )?;


    //imwrite("rotated.jpeg", &rotated, &core::Vector::new())?;

    Ok(rotated)
}

fn threshold(image: &Mat, dest: &mut Mat) -> Result<()> {
    imgproc::threshold(
        image,
        dest,
        0.0,
        255.0,
        imgproc::THRESH_OTSU,
    )?;

    Ok(())
}


pub fn deskew(page: &Mat) -> Result<(Mat, f64)> {
    let mut dark_page = Mat::default();

    threshold(page, &mut dark_page)?;

    let angle = compute_skew(&dark_page)?;

    let rotated = rotate(&dark_page, angle)?;

    Ok((rotated, angle))
}


