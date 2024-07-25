use crate::framebuffer::Framebuffer;

pub fn draw() {
    let width = 800;
    let height = 600;
    let mut framebuffer = Framebuffer::new(width, height, 0x000000); // Negro como color de fondo

    let polygon1 = vec![
        (165, 380), (185, 360), (180, 330), (207, 345), (233, 330), (230, 360),
        (250, 380), (220, 385), (205, 410), (193, 383)
    ];

    let polygon2 = vec![
        (321, 335), (288, 286), (339, 251), (374, 302)
    ];

    framebuffer.draw_polygon(&polygon1, 0xFFFFFF, 0xFFFF00); // Borde blanco, relleno amarillo
    framebuffer.draw_polygon(&polygon2, 0xFFFFFF, 0x0000FF); // Borde blanco, relleno azul
    framebuffer.render_buffer("polygon2.bmp").unwrap();
    framebuffer.display();
}
