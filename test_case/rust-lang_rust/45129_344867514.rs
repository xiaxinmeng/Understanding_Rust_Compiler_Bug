rust
struct Foo { x: usize }

static mut SFOO : Foo = Foo{x: 23 }; 

impl Foo {
    fn x(&mut self) -> &mut usize { &mut self.x }
}

fn main() {
    unsafe {
        let x = SFOO.x();
        SFOO.x += 1; // ?
        *x += 1; 
    }
}
