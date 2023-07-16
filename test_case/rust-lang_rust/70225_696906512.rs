rust
enum Foo {
    Bar,
}

fn main() {
    let p = [0; 0];
    p.print();
}

trait Print {
    fn print(&self) -> usize {
        3
    }
}

impl Print for [u32; Foo::Bar as usize] {
    // fn print(&self) -> usize {
    //    3
    // }
}
