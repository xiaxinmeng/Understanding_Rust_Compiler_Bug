
struct Inner<I, V> {
    iterator: I,
    item: V,
}

struct Outer<I: Iterator> {
    inner: Inner<I, I::Item>,
}

fn outer<I>(iterator: I) -> Outer<I>
    where I: Iterator,
          I::Item: Default,
{
    Outer {
        inner: Inner {
            iterator: iterator,
            item: Default::default(),
        }
    }
}

fn main() {
    outer(std::iter::once(&1).cloned());
}
