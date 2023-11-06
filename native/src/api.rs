
use image::{RgbImage, Rgb};
use rand::Rng;
use std::io::Cursor;
// use std::fs::File;
// use std::io::Read;

// END: the code for test flag --use_bridge_in_method

pub fn publish_message(message: String)-> String {

  
    String::from(  format!("{message}  🦀"))
}
pub fn publish_message_bienvenida(message: String)-> String {

  
    String::from(  format!("{message}  🦀"))
}


pub fn draw_tree() -> Vec<u8> {
    // Define las constantes del sistema de funciones iteradas de Barnsley.
    let mut x = 0.0;
    let mut y = 0.0;

    let width = 800;
    let height = 600;
    let mut image = RgbImage::new(width, height);

    let mut rng = rand::thread_rng();

    // Realiza iteraciones para generar el fractal y representarlo en una imagen.
    for _ in 0..100000 {
        let r: f64 = rng.gen_range(0.0..1.0);

        if r < 0.01 {
            // Función 1
            let new_x = 0.0;
            let new_y = 0.16 * y;
            x = new_x;
            y = new_y;
        } else if r < 0.86 {
            // Función 2
            let new_x = 0.85 * x + 0.04 * y;
            let new_y = -0.04 * x + 0.85 * y + 1.6;
            x = new_x;
            y = new_y;
        } else if r < 0.93 {
            // Función 3
            let new_x = 0.2 * x - 0.26 * y;
            let new_y = 0.23 * x + 0.22 * y + 1.6;
            x = new_x;
            y = new_y;
        } else {
            // Función 4
            let new_x = -0.12 * x + 0.28 * y;
            let new_y = 0.26 * x + 0.24 * y + 0.44;
            x = new_x;
            y = new_y;
        }

        // Mapea las coordenadas al espacio de la imagen y colorea el píxel.
        let px = ((x + 2.5) / 5.0 * width as f64) as u32;
        let py = ((-y + 10.0) / 12.0 * height as f64) as u32;

        if px < width && py < height {
            image.put_pixel(px, py, Rgb([0, 255, 0])); // Color verde
        }
    }

    let mut image_bytes = Vec::new();
    let mut cursor = Cursor::new(&mut image_bytes);
    image.write_to(&mut cursor, image::ImageOutputFormat::Jpeg(85)).unwrap();

    image_bytes
}