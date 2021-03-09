use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

mod drawer;
use drawer::Drawer;

#[wasm_bindgen]
pub struct Universe {
    b_rule : [bool; 9],
    s_rule : [bool; 9],
    width: usize,
    height: usize,
    cells: Vec<u8>,
    drawer: Drawer,
}

#[wasm_bindgen]
impl Universe {
    #[wasm_bindgen(constructor)]
    pub fn new(birth_rule : &str, survive_rule : &str, width : usize, height: usize, ctx: CanvasRenderingContext2d) -> Universe{

        // translate string into rule

        let char_to_ix = |c:char| c.to_digit(10).unwrap() as usize;

        let mut b_rule = [false; 9];
        for l in birth_rule.chars() {b_rule[char_to_ix(l)] = true}

        let mut s_rule = [false; 9];
        for l in survive_rule.chars() {s_rule[char_to_ix(l)] = true}


        let cells = vec![0u8; width*height];
        let drawer = Drawer::new_with_uniform_color(2*width, 2*height, ctx, (200, 200, 200, 255));

        Universe {b_rule, s_rule, width, height, cells, drawer}
    }

    pub fn add_cell(&mut self, x: usize, y: usize){
        self.cells[y*self.width + x] = 1u8;
    }
    
    pub fn step(&mut self) -> u32{
        let mut next: Vec<bool> = Vec::new();

        for y in 0..self.height {
            for x in 0..self.width { 
                let state = self.cells[y*self.width + x] == 1;

                self.drawer.draw_rect_transparency(2*x, 2*y, 2, 2, 
                    if state {255} else {0});
                
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
        self.drawer.frame
    }

    pub fn draw(&self) -> Result<(), JsValue>{
        self.drawer.display_to_ctx()
    }
}
