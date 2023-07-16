rust
fn foo(a1: &mut Bar, j: usize) {
    let a = [a1];
    a[0].bar(); 
    let x: usize = j % 2;
    a[x].bar();
}
