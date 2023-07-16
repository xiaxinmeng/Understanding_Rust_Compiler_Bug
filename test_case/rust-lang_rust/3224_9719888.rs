
struct A { x: &mut int, drop { *(self.x) -= 1; error!("A"); } } 
struct B { x: A }

fn main() {
    let mut y = 3;
    let bp = ~B { x: A { x: &mut y } };
    let ~B { x: _a } = move bp;
    assert(y == 2);
}
