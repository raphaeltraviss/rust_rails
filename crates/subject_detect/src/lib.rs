#[macro_use]
extern crate helix;
extern crate image;
extern crate imageproc;

use std::path::Path;
use imageproc::hough::*;
use imageproc::edges::*;
use imageproc::map::*;
use image::{open, Rgb, ImageBuffer};

ruby! {
  class SubjectDetect {
    def get_number() -> i32 {
      let path = Path::new("/Users/raphael/Desktop/IMG_1109.JPG");
      crop(&path);
      55
    }

    def get_compression_rating() -> i32 {
      let path = Path::new("/Users/raphael/dev/rust_rails/crates/subject_detect/test_stock/squares.jpeg");
      let image = open(path)
        .expect(&format!("Could not load image at {:?}", path))
        .to_luma();

      // Convert image into an image of the edges.
      let edges = canny(&image, 50.0, 100.0);
      let edges_path = Path::new("/Users/raphael/dev/rust_rails/crates/subject_detect/test_stock/squares_edges.png");
      edges.save(&edges_path).unwrap();

      // Find the lines in the edge image.
      let options = LineDetectionOptions {
        vote_threshold: 40,
        suppression_radius: 8,
      };
      let lines = detect_lines(&edges, options);

      let white = Rgb([255, 255, 255]);
      let green = Rgb([0, 255, 0]);
      let black = Rgb([0, 0, 0]);

      // Convert edge image to colour
      let color_edges = map_colors(&edges, |p| if p[0] > 0 { white } else { black });

      // Draw lines on top of edge image
      let lines_image = draw_polar_lines(&color_edges, &lines, green);
      let lines_path = Path::new("/Users/raphael/dev/rust_rails/crates/subject_detect/test_stock/square_edges_lines.png");
      lines_image.save(&lines_path).unwrap();

      println!("hey, here I am!");
      let result = lines.len() as i32;
      println!("blah blah {}", result);
      result
    }
  }
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

