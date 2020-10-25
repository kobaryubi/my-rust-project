use crate::video::video_processor::{
    resize_image,
    show_image_info,
};
use image::{
    DynamicImage,
    GenericImage,
    GenericImageView,
};
use image;

pub fn exe_module(module_name: &str, args: Vec<String>) {
    if module_name == "resize_image" {
        if args.len() >= 1 {
            // ToDo: create resize image function
            resize_image();
        }
    } else if module_name == "open_and_save_image" {
        if args.len() >= 1 {
            // ToDo: get image path from args
            let image: DynamicImage = image::open("sample/images/lena.jpg").unwrap();
            show_image_info(&image);
            image.save("sample/images/lena.png").unwrap();
        }
    }
}