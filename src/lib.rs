use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Universe {
    b_rule : [bool; 9],
    s_rule : [bool; 9],
    width: usize,
    height: usize,
    cells: Vec<u8>,
    gen : u32,
}

#[wasm_bindgen]
impl Universe {
    pub fn new(birth_rule : &str, survive_rule : &str, width : usize, height: usize) -> Universe{
        let mut b_rule = [false; 9];
        for l in birth_rule.chars() {
            let n = l.to_digit(10).unwrap() as usize;
            b_rule[n] = true;
        }

        let mut s_rule = [false; 9];
        for l in survive_rule.chars() {
            let n = l.to_digit(10).unwrap() as usize;
            s_rule[n] = true;
        }

        let cells = vec![0u8; width*height];

        Universe {b_rule, s_rule, width, height, cells, gen: 0}
    }

    pub fn add_cell(&mut self, x: usize, y: usize){
        self.cells[y*self.width + x] = 1u8;
    }

    pub fn step(&mut self) -> u32{
        let mut next: Vec<bool> = Vec::new();

        for y in 0..self.height {
            for x in 0..self.width {
                let index = |dx: usize, dy: usize| self.cells[((y+dy)%self.height)*self.width + (x+dx)%self.width];
                
                let n = index(self.width-1, self.height-1) + index(self.width-1, 0) + index(self.width-1, 1)
                          + index(0, self.height-1)                                     + index(0, 1)
                          + index(1, self.height-1)            + index(1, 0)            + index(1, 1);

                next.push(self.b_rule[n as usize] || self.s_rule[n as usize] && (self.cells[y*self.width + x]==1));
            }
        }

        for (p, new) in self.cells.iter_mut().zip(next){
            *p = new as u8;
        }
        self.gen
    }

    pub fn cell(&self, x: usize, y: usize) -> bool {
        self.cells[y*self.width + x] == 1
    }
}