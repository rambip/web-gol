use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData};


pub struct Drawer {
    pub width: usize,
    pub height: usize,
    pub frame: u32,
    raw_pixels: Vec<u8>,
    ctx: CanvasRenderingContext2d,
}


type Color = (u8, u8, u8, u8);


impl Drawer {
    pub fn new_with_uniform_color(width: usize, height: usize, ctx: CanvasRenderingContext2d, c: Color) -> Drawer {
        let mut raw_pixels = Vec::with_capacity(width*height*4);
        let (r, g, b, a) = c;
        for _ in 0..width*height {
            raw_pixels.push(r);
            raw_pixels.push(g);
            raw_pixels.push(b);
            raw_pixels.push(a);
        }

        Drawer {width, height, frame:0, raw_pixels, ctx}
    }

    pub fn draw_rect_transparency(&mut self, x: usize, y: usize, w: usize, h: usize, c: u8){
        for iy in y..y+h {
            for ix in x..x+w {
                // set alpha channel
                self.raw_pixels[4*(iy*self.width + ix) + 3] = c;
            }
        }
    }

    pub fn display_to_ctx(&self) -> Result<(), JsValue> {

        let data = ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(&self.raw_pixels[..]), 
            self.width as u32,
            self.height as u32)?;

        self.ctx.put_image_data(&data, 0.0, 0.0)
    }
}
