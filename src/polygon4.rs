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

    framebuffer.draw_polygon(&polygon1, 0xFFFFFF, 0xFFFF00); // Borde blanco, relleno amarillo
    framebuffer.draw_polygon(&polygon2, 0xFFFFFF, 0x0000FF); // Borde blanco, relleno azul
    framebuffer.draw_polygon(&polygon3, 0xFFFFFF, 0xFF0000); // Borde blanco, relleno rojo
    framebuffer.draw_polygon(&polygon4, 0xFFFFFF, 0x00FF00); // Borde blanco, relleno verde
    
    // Agregar el agujero (polígono 5) con el color de fondo para simular un agujero
    framebuffer.draw_polygon(&polygon5, 0xFFFFFF, 0x000000); // Borde blanco, relleno negro (color de fondo)
    framebuffer.render_buffer("polygon4.bmp").unwrap();
    framebuffer.display();
}
