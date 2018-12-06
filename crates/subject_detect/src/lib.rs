#[macro_use]
extern crate helix;
extern crate image;
extern crate imageproc;

use std::env;
use std::path::Path;
use image::{open, ImageBuffer};
use imageproc::hog::*;

ruby! {
  class SubjectDetect {
    def get_number() -> i32 {
      let path = Path::new("/Users/raphael/Desktop/IMG_1109.JPG");
      crop(&path);
      calculate(1234123423)
    }
  }
}

fn calculate(input: i32) -> i32 {
  input
}

fn crop(input: &Path) {
  let image = open(input)
    .expect(&format!("Could not load image at {:?}", input))
    .to_luma();
  let (width, height) = image.dimensions();
  // Crop image to a suitable size
  let (cropped_width, cropped_height) = (width - 200, height - 200);
  let mut cropped = ImageBuffer::new(cropped_width, cropped_height);
  for y in 0..cropped_height {
    for x in 0..cropped_width {
      cropped.put_pixel(x, y, *image.get_pixel(x, y));
    }
  }
  cropped.save(input.with_file_name("cropped.png")).unwrap();
}
