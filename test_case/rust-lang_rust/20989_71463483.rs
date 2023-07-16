 rust
#![allow(dead_code)]

#[derive(Show)]
struct Cell;

struct Grid {
    cols: [[Cell; 6]; 6]
}

impl Grid {
    fn iter(&self) -> Box<Iterator<Item = &Cell>> {
        Box::new(self.cols.iter().flat_map(|x| x.iter()))
    }
    fn fill(&mut self) {
        for cell in self.iter() {
            println!("here's a cell: {:?}", cell);
        }
    }
}

fn main() {
}
