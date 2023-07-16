rust
trait Bar {
    fn bar(&mut self);
}

fn foo(a1: &mut Bar, j: usize) {
    //let a = [a1];
    // why doesn't vec work?
    let a = vec![a1];
    a[0].bar();
    let x: usize = j % 2;
    a[x].bar();
}

fn main() {}
