use image;
use image::{
    DynamicImage,
    GenericImage,
    GenericImageView,
};

pub fn resize_image() {
    // ToDo: create resize image function
    println!("resize image");
}

pub fn show_image_info(image: &DynamicImage) {
    println!("dimensions: {:?}", &image.dimensions());
    println!("color: {:?}", &image.color());
}