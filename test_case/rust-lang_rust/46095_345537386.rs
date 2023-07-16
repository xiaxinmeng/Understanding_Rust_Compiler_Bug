rust
fn foo<'a>(a1: &'a mut Bar, a2: &'a mut Bar) {
    let a = [a1, a2];
    for i in 0..42 {
        let index2: usize = i % 2;
        a[i].bar();
        a[index2].bar();
    }
}
