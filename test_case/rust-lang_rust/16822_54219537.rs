 rust
extern crate lib;

use std::cell::RefCell;

struct App{
    i: int
}

impl lib::Update for App{
    fn update(&mut self){
        self.i += 1;
    }
}

fn main(){
    let app = App{i: 5};
    let window = lib::Window{data: RefCell::new(app)};
    window.update(1);
}
