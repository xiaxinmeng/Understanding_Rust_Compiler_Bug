rust
trait Bar {
    fn bar(&mut self);
}

fn foo(a1: &mut Bar, j: usize) {
    let a = [a1];
    a[0].bar(); //compilation ok
    a[j % 2].bar();
}

fn main() {}
