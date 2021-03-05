use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData};


#[wasm_bindgen]
pub struct Universe {
    b_rule : [bool; 9],
    s_rule : [bool; 9],
    width: usize,
    height: usize,
    cells: Vec<u8>,
    gen : u32,
    raw_image : Vec<u8>, // represent the image as rgba array
}

#[wasm_bindgen]
impl Universe {
    #[wasm_bindgen(constructor)]
    pub fn new(birth_rule : &str, survive_rule : &str, width : usize, height: usize) -> Universe{

        // translate string into rule

        let char_to_ix = |c:char| c.to_digit(10).unwrap() as usize;

        let mut b_rule = [false; 9];
        for l in birth_rule.chars() {b_rule[char_to_ix(l)] = true}

        let mut s_rule = [false; 9];
        for l in survive_rule.chars() {s_rule[char_to_ix(l)] = true}


        let cells = vec![0u8; width*height];
        let raw_image = vec![255; width*height*16];

        Universe {b_rule, s_rule, width, height, cells, gen: 0, raw_image}
    }

    pub fn add_cell(&mut self, x: usize, y: usize){
        self.cells[y*self.width + x] = 1u8;
    }
    
    fn draw_pixel(&mut self, x:usize, y:usize, transparency:u8){
        let ix = 4*(2*y*self.width + x);
        self.raw_image[ix + 3] = transparency;
    }

    fn draw_square(&mut self, x:usize, y:usize, state:bool){
        let trans = if state {255} else {0};
        self.draw_pixel(2*x,          2*y       , trans); // top left
        self.draw_pixel(2*x+1usize,   2*y       , trans); // top right
        self.draw_pixel(2*x,          2*y+1usize, trans); // bottom left
        self.draw_pixel(2*x+1usize,   2*y+1usize, trans); // bottom right
    }

    pub fn step(&mut self) -> u32{
        let mut next: Vec<bool> = Vec::new();

        for y in 0..self.height {
            for x in 0..self.width { 
                let state = self.cells[y*self.width + x] == 1;

                self.draw_square(x, y, state);
                
                // index on a torus: x=-1 is the same as x=width-2
                let index = |dx: usize, dy: usize| 
                    self.cells[((y+dy)%self.height)*self.width + (x+dx)%self.width];

                let xmax = self.width-1;
                let ymax = self.height-1;

                // number of alive cells in the 8
                let neighbours = (
                            index(xmax, ymax) + index(xmax, 0) + index(xmax, 1)
                          + index(0, ymax)                     + index(0, 1)
                          + index(1, ymax)    + index(1, 0)    + index(1, 1)
                          ) as usize;

                next.push(
                    if state {self.s_rule[neighbours]} // if alive, look at survival rule
                    else     {self.b_rule[neighbours]} // else look at birth rule
                );
            }
        }

        for (p, new) in self.cells.iter_mut().zip(next){
            // old becomes new: update all grid
            *p = new as u8;
        }
        self.gen
    }

    pub fn draw(&self, ctx:&CanvasRenderingContext2d) -> Result<(), JsValue>{

        // draw image on a js context
        let data = ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(&self.raw_image[..]), 
            self.width as u32 * 2,
            self.height as u32 * 2)?;

        ctx.put_image_data(&data, 0.0, 0.0)
    }
}
