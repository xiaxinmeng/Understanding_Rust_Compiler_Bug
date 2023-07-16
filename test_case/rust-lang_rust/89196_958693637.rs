rust
struct E<A, B> {
    a: A,
    b: B,
}

struct S;

impl S {
    fn e<A, B>(&self, a: A, b: B)
        where Self: R<E<A, B>>
    {}
}

trait R<A> {}

impl<A> R<A> for S where A: Tr {}

trait Tr {}
trait HasItem {
    type Item;
}
impl<T: IntoIterator> HasItem for T {
    type Item = T::Item;
}
impl<T> HasItem for [T] {
    type Item = T;
}

impl<A, B> Tr for E<A, B>
where
    A: AsRef<[u8]>,
    B: HasItem,
    <B as HasItem>::Item: AsRef<[u8]>,
    for<'a> &'a B: IntoIterator<Item = &'a <B as HasItem>::Item>,
    for<'a> <&'a B as IntoIterator>::IntoIter: Clone,
{
}

const ST: &str = "";
const V: Vec<u8> = Vec::new();

fn main() {
    let arr: [Vec<u8>; 1] = [V];
    S.e(ST, arr);
}
