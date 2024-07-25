use minifb::{Key, Window, WindowOptions};
use std::fs::File;
use std::io::{self, Write};

pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>,
    background_color: u32,
    current_color: u32,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize, background_color: u32) -> Framebuffer {
        Framebuffer {
            width,
            height,
            buffer: vec![background_color; width * height],
            background_color,
            current_color: 0,
        }
    }

    pub fn clear(&mut self) {
        self.buffer.fill(self.background_color);
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: u32) {
        if x < self.width && y < self.height {
            let inverted_y = self.height - 1 - y; // Invertir la coordenada y
            self.buffer[inverted_y * self.width + x] = color;
        }
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> u32 {
        if x < self.width && y < self.height {
            let inverted_y = self.height - 1 - y; // Invertir la coordenada y
            self.buffer[inverted_y * self.width + x]
        } else {
            self.background_color
        }
    }

    pub fn set_background_color(&mut self, color: u32) {
        self.background_color = color;
    }

    pub fn set_current_color(&mut self, color: u32) {
        self.current_color = color;
    }

    pub fn get_current_color(&self) -> u32 {
        self.current_color
    }

    pub fn point(&mut self, x: isize, y: isize) {
        if x >= 0 && y >= 0 && (x as usize) < self.width && (y as usize) < self.height {
            self.set_pixel(x as usize, y as usize, self.current_color);
        }
    }

    pub fn draw_line(&mut self, x1: isize, y1: isize, x2: isize, y2: isize) {
        let mut x = x1;
        let mut y = y1;
        let dx = (x2 - x1).abs();
        let dy = (y2 - y1).abs();
        let sx = if x1 < x2 { 1 } else { -1 };
        let sy = if y1 < y2 { 1 } else { -1 };
        let mut err = if dx > dy { dx / 2 } else { -dy / 2 };

        loop {
            self.point(x, y);
            if x == x2 && y == y2 {
                break;
            }
            let e2 = err;
            if e2 > -dx {
                err -= dy;
                x += sx;
            }
            if e2 < dy {
                err += dx;
                y += sy;
            }
        }
    }

    pub fn draw_polygon(&mut self, points: &[(isize, isize)], outline_color: u32, fill_color: u32) {
        // Dibujar el relleno primero
        self.set_current_color(fill_color);
        self.fill_polygon(points);

        // Luego dibujar el contorno
        self.set_current_color(outline_color);
        for i in 0..points.len() {
            let (x1, y1) = points[i];
            let (x2, y2) = if i == points.len() - 1 {
                points[0]
            } else {
                points[i + 1]
            };
            self.draw_line(x1, y1, x2, y2);
        }
    }

    pub fn fill_polygon(&mut self, points: &[(isize, isize)]) {
        let min_y = points.iter().map(|&(_, y)| y).min().unwrap_or(0);
        let max_y = points.iter().map(|&(_, y)| y).max().unwrap_or(0);

        for y in min_y..=max_y {
            let mut nodes: Vec<isize> = Vec::new();
            let mut j = points.len() - 1;
            for i in 0..points.len() {
                let (x1, y1) = points[i];
                let (x2, y2) = points[j];
                if (y1 < y && y2 >= y) || (y2 < y && y1 >= y) {
                    nodes.push(x1 + (y - y1) * (x2 - x1) / (y2 - y1));
                }
                j = i;
            }
            nodes.sort_unstable();
            for i in (0..nodes.len()).step_by(2) {
                if i + 1 < nodes.len() {
                    let x1 = nodes[i];
                    let x2 = nodes[i + 1];
                    for x in x1..=x2 {
                        self.point(x, y);
                    }
                }
            }
        }
    }

    pub fn render_buffer(&self, file_path: &str) -> io::Result<()> {
        write_bmp_file(file_path, &self.buffer, self.width, self.height)
    }

    pub fn display(&self) {
        let mut window = Window::new(
            "Framebuffer - Press ESC to exit",
            self.width,
            self.height,
            WindowOptions::default(),
        )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

        // Convertir el buffer a un formato que minifb pueda usar
        let buffer: Vec<u32> = self.buffer.iter().map(|&color| {
            // Convertir de 0xAARRGGBB a 0x00RRGGBB
            let r = (color >> 16) & 0xFF;
            let g = (color >> 8) & 0xFF;
            let b = color & 0xFF;
            (r << 16) | (g << 8) | b
        }).collect();

        // Mostrar la ventana hasta que el usuario presione ESC
        while window.is_open() && !window.is_key_down(Key::Escape) {
            window.update_with_buffer(&buffer, self.width, self.height).unwrap();
        }
    }
}

pub fn write_bmp_file(file_path: &str, buffer: &[u32], width: usize, height: usize) -> io::Result<()> {
    let mut file = File::create(file_path)?;

    // BMP Header
    let file_size: u32 = 14 + 40 + (width * height * 4) as u32;
    let offset_to_pixel_data: u32 = 54;
    let dib_header_size: u32 = 40;
    let planes: u16 = 1;
    let bits_per_pixel: u16 = 32;
    let compression: u32 = 0;
    let image_size: u32 = (width * height * 4) as u32;
    let ppm: i32 = 2835; // 72 DPI * 39.3701 inches per meter

    // File Header
    file.write_all(b"BM")?;
    file.write_all(&file_size.to_le_bytes())?;
    file.write_all(&[0; 4])?;
    file.write_all(&offset_to_pixel_data.to_le_bytes())?;

    // DIB Header
    file.write_all(&dib_header_size.to_le_bytes())?;
    file.write_all(&(width as u32).to_le_bytes())?;
    file.write_all(&(height as u32).to_le_bytes())?;
    file.write_all(&(planes as u16).to_le_bytes())?;
    file.write_all(&(bits_per_pixel as u16).to_le_bytes())?;
    file.write_all(&(compression as u32).to_le_bytes())?;
    file.write_all(&image_size.to_le_bytes())?;
    file.write_all(&ppm.to_le_bytes())?;
    file.write_all(&ppm.to_le_bytes())?;
    file.write_all(&[0; 8])?;

    // Pixel Data
    for pixel in buffer {
        file.write_all(&pixel.to_le_bytes())?;
    }

    Ok(())
}
