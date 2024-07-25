use crate::scanline;
use image::{Rgb, RgbImage};

pub fn draw() {
    let polygon1 = vec![
        (165, 380), (185, 360), (180, 330), (207, 345), (233, 330), (230, 360),
        (250, 380), (220, 385), (205, 410), (193, 383)
    ];

    let polygon2 = vec![
        (321, 335), (288, 286), (339, 251), (374, 302)
    ];

    let mut img = RgbImage::new(800, 600);
    scanline::fill_polygon(&mut img, &polygon1, Rgb([255, 255, 0]), Rgb([255, 255, 255])); // Amarillo con borde blanco
    scanline::fill_polygon(&mut img, &polygon2, Rgb([0, 0, 255]), Rgb([255, 255, 255])); // Azul con borde blanco
    img.save("polygon2.bmp").unwrap();
}
