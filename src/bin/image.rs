use opencv::core::{Mat, MatTrait, Point, Scalar, Size, BORDER_CONSTANT};
use opencv::imgcodecs::{imread, IMREAD_COLOR};
use opencv::prelude::MatTraitConstManual;
use opencv::core::Vector;

fn main() {
    let image = imread("test/dataset/IMG_0089_peccary.JPG", IMREAD_COLOR).unwrap();
    let largest_dim = image
        .size()
        .unwrap()
        .width
        .max(image.size().unwrap().height);
    println!("largest_dim: {}", largest_dim);

    // Letterbox the image.
    let mut padded_image = Mat::default();
    let mut top = 0;
    let mut bottom = 0;
    let mut left = 0;
    let mut right = 0;

    if image.size().unwrap().width > image.size().unwrap().height {
        top = (largest_dim - image.size().unwrap().height) / 2;
        bottom = top;
        left = 0;
        right = 0;
    } else {
        top = 0;
        bottom = 0;
        left = (largest_dim - image.size().unwrap().width) / 2;
        right = left;
    }

    opencv::core::copy_make_border(
        &image,
        &mut padded_image,
        top,
        bottom,
        left,
        right,
        BORDER_CONSTANT,
        Scalar::new(0f64, 0f64, 0f64, 0f64),
    )

    .unwrap();

    // Resize the image.
    let mut resized_image = Mat::default();
    opencv::imgproc::resize(
        &padded_image,
        &mut resized_image,
        Size::new(640, 640),
        0f64,
        0f64,
        opencv::imgproc::INTER_LINEAR,
    )
    .unwrap();

    // Write the image.
    opencv::imgcodecs::imwrite("output.jpg", &resized_image, &Vector::<i32>::new())
        .unwrap();

    println!("{:?}", image.size());
}
