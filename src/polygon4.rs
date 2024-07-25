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

    let polygon3 = vec![
        (377, 249), (411, 197), (436, 249)
    ];

    let polygon4 = vec![
        (413, 177), (448, 159), (502, 88), (553, 53), (535, 36), (676, 37), (660, 52),
        (750, 145), (761, 179), (672, 192), (659, 214), (615, 214), (632, 230), (580, 230),
        (597, 215), (552, 214), (517, 144), (466, 180)
    ];

    let polygon5 = vec![
        (682, 175), (708, 120), (735, 148), (739, 170)
    ];

    let mut img = RgbImage::new(800, 600);
    scanline::fill_polygon(&mut img, &polygon1, Rgb([255, 255, 0]), Rgb([255, 255, 255])); // Amarillo con borde blanco
    scanline::fill_polygon(&mut img, &polygon2, Rgb([0, 0, 255]), Rgb([255, 255, 255])); // Azul con borde blanco
    scanline::fill_polygon(&mut img, &polygon3, Rgb([255, 0, 0]), Rgb([255, 255, 255])); // Rojo con borde blanco
    scanline::fill_polygon(&mut img, &polygon4, Rgb([0, 255, 0]), Rgb([255, 255, 255])); // Verde con borde blanco
    
    // Agregar el agujero (polígono 5)
    scanline::fill_polygon(&mut img, &polygon5, Rgb([0, 0, 0]), Rgb([255, 255, 255])); // Negro (asumiendo que el fondo es negro)
    img.save("polygon4.bmp").unwrap();
}
