rust
#![feature(existential_type)]

fn main() {
    let mut v = Vec::new();
    let x = good(v.iter().cloned());
    let y = bad(v.iter().cloned()); // if you comment out this line, it compiles.
    v.push(3);
}

fn good<I: Iterator<Item = usize>>(x: I) -> std::vec::IntoIter<usize> {
    x.collect::<Vec<usize>>().into_iter()
}

existential type Foo: Iterator<Item = usize>;

fn bad<I: Iterator<Item = usize>>(x: I) -> Foo {
    x.collect::<Vec<usize>>().into_iter()
}
