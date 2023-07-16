rust
const TLC: usize = 4;
trait Tr { fn doit(&self); }
impl Tr for [usize; TLC] {
    fn doit(&self) { println!("called 4"); }
}

fn main() {
    let s = [0,1,2,3];
    s.doit(); // which .doit is called depends on architecture
}
