 rust
extern crate quickcheck;
#[test]
fn send_nodes_to_bytes_test() {
    use self::quickcheck::{Arbitrary, Gen, quickcheck};

    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    struct PackedNode;

    /// Valid, random PackedNode.
    impl Arbitrary for PackedNode {
        fn arbitrary<G: Gen>(_: &mut G) -> Self { unimplemented!() }
    }

    fn with_nodes(_: PackedNode, _: Option<PackedNode>) { unimplemented!() }
    quickcheck(with_nodes as fn(PackedNode, Option<PackedNode>));
}
