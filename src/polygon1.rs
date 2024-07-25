extern crate image;

use crate::scanline;
use image::{Rgb, RgbImage};

pub fn draw() {
    let polygon1 = vec![
        (165, 380), (185, 360), (180, 330), (207, 345), (233, 330), (230, 360),
        (250, 380), (220, 385), (205, 410), (193, 383)
    ];

    let mut img = RgbImage::new(800, 600);
    scanline::fill_polygon(&mut img, &polygon1, Rgb([255, 255, 0]), Rgb([255, 255, 255])); // Amarillo con borde blanco
    img.save("polygon1.bmp").unwrap();
}
