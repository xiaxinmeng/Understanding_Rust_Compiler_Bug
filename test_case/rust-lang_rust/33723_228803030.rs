 rust
// Heavily reduced version of `quickcheck` with an `fn main`that ICE's the compiler.
//
// This is for issue #33723.

pub trait Arbitrary : Clone + 'static {
    fn arbitrary<G>() -> Self { unimplemented! () }
    fn shrink(&self) -> Box<Iterator<Item=Self>> { unimplemented!() }
}

impl<A: Arbitrary> Arbitrary for Option<A> { }

impl<A: Arbitrary> Arbitrary for (A,)
{
    fn shrink(&self) -> Box<Iterator<Item=(A,)>> {
        let (ref a, ) = *self;
        let srest = a
            .shrink()
            .scan((), |_, _| unimplemented!());
        Box::new(srest)
    }
}

impl<A: Arbitrary, B: Arbitrary> Arbitrary for (A, B,)
{
    fn shrink(&self) -> Box<Iterator<Item=(A, B)>> {
        let (ref a, ref b) = *self;
        let srest = (b.clone(),)
            .shrink()
            .scan(a.clone(), |_, _| unimplemented!());
        Box::new(srest)
    }
}

pub fn quickcheck<A: Testable>(f: A) {
    f.result::<A>();
}

pub trait Testable : Send + 'static {
    fn result<G>(&self) { unimplemented!() }
}

impl Testable for () { }

impl<T: Testable, A: Arbitrary, B: Arbitrary> Testable for fn(A, B) -> T
{
    fn result<G_>(&self) {
        let a: (A, B) = Arbitrary::arbitrary::<G_>();
        a.shrink();
    }
}

#[derive(Clone, Debug)]
struct PackedNode;
impl Arbitrary for PackedNode { }

fn main() {
    fn with_nodes(_: PackedNode, _: Option<PackedNode>) { unimplemented!() }
    quickcheck(with_nodes as fn(PackedNode, Option<PackedNode>));
}
