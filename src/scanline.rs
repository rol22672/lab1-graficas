extern crate nalgebra_glm as glm;
extern crate image;

use glm::Vec2;
use image::{Rgb, RgbImage};

pub fn fill_polygon(img: &mut RgbImage, polygon: &Vec<(i32, i32)>, fill_color: Rgb<u8>, border_color: Rgb<u8>) {
    let mut vertices: Vec<Vec2> = polygon.iter().map(|&(x, y)| Vec2::new(x as f32, y as f32)).collect();
    let height = img.height() as i32;
    
    vertices.sort_by(|a, b| a.y.partial_cmp(&b.y).unwrap());
    
    let mut y = vertices[0].y as i32;
    while y <= vertices[vertices.len() - 1].y as i32 {
        let mut intersections = Vec::new();
        for i in 0..vertices.len() {
            let next = (i + 1) % vertices.len();
            let v1 = &vertices[i];
            let v2 = &vertices[next];
            
            if (v1.y as i32 <= y && v2.y as i32 > y) || (v1.y as i32 > y && v2.y as i32 <= y) {
                let x = v1.x + (y as f32 - v1.y) * (v2.x - v1.x) / (v2.y - v1.y);
                intersections.push(x);
            }
        }
        
        intersections.sort_by(|a, b| a.partial_cmp(&b).unwrap());
        
        for i in (0..intersections.len()).step_by(2) {
            if i + 1 < intersections.len() {
                let x_start = intersections[i] as i32;
                let x_end = intersections[i + 1] as i32;
                
                for x in x_start..x_end {
                    if x >= 0 && x < img.width() as i32 && y >= 0 && y < height {
                        img.put_pixel(x as u32, y as u32, fill_color);
                    }
                }
            }
        }
        
        y += 1;
    }
    
    // Dibujar el borde
    for i in 0..vertices.len() {
        let v1 = &vertices[i];
        let v2 = &vertices[(i + 1) % vertices.len()];
        draw_line(img, v1.x as i32, v1.y as i32, v2.x as i32, v2.y as i32, border_color);
    }
}

fn draw_line(img: &mut RgbImage, x0: i32, y0: i32, x1: i32, y1: i32, color: Rgb<u8>) {
    let dx = (x1 - x0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let dy = -(y1 - y0).abs();
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;
    let mut x0 = x0;
    let mut y0 = y0;

    loop {
        if x0 >= 0 && x0 < img.width() as i32 && y0 >= 0 && y0 < img.height() as i32 {
            img.put_pixel(x0 as u32, y0 as u32, color);
        }
        if x0 == x1 && y0 == y1 { break; }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x0 += sx;
        }
        if e2 <= dx {
            err += dx;
            y0 += sy;
        }
    }
}
