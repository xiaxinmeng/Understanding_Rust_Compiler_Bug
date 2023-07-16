 rust
#![crate_type = "lib"]

// these two are equivalent
struct Rev1<I> where I: DoubleEndedIterator {
    it: I,
}

struct Rev2<I: DoubleEndedIterator> {
    it: I,
}

// these two don't work because `Rev<I>` doesn't imply that `I: DoubleEndedIterator`
fn foo1<I>(mut rev: Rev1<I>) {
    rev.it.next_back();
}

fn foo2<I>(mut rev: Rev2<I>) {
    rev.it.next_back();
}

// even these won't work unless you explicitly add `I: DoubleEndedIterator` as a bound
fn bar1<I>(mut rev: Rev1<I>) {}
fn bar2<I>(mut rev: Rev2<I>) {}
