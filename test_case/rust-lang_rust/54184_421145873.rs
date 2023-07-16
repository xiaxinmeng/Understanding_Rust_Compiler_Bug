rust
#![feature(existential_type)]

fn main() {
    let v = vec![1, 2, 3, 4];
    let y = demoit(v.iter().cloned());
    for (i, elem) in y.enumerate() {
        println!("processing y[{}]: {:?}", i, elem);
    }

    let v = vec![1, 2, 3, 4];
    let y = choose(v.iter().cloned());
    for (i, elem) in y.enumerate() {
        println!("processing y[{}]: {:?}", i, elem);
    }
}

existential type WhyArgNeeded<I>: Iterator<Item = usize>;

fn demoit<I: Iterator<Item = usize>>(x: I) -> WhyArgNeeded<I> {
    // If you remove `I` from `WhyArgNeeded`, this will break.
    return Wrapped(x);
}

existential type MaybeWrapped<I>: Iterator<Item = usize>;

fn choose<I: Iterator<Item = usize>>(x: I) -> MaybeWrapped<I> {
    // `demoit` above shows that variant below would work.
    // return Wrapped(x);
    return x.collect::<Vec<usize>>().into_iter();
}

/// Trivial wrapper around an iterator.
struct Wrapped<I: Iterator>(I);
impl<I: Iterator> Iterator for Wrapped<I> {
    type Item = I::Item;
    fn next(&mut self) -> Option<I::Item> { self.0.next() }
}
