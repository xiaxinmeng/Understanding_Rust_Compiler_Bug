rust
#![feature(nll)]

pub trait Hasher {
    type Out: Eq;
}

pub trait NodeCodec<H: Hasher> {
    const HASHED_NULL_NODE: H::Out;
}

pub trait Trie<H: Hasher, C: NodeCodec<H>> {
    fn root(&self) -> &H::Out;

    fn is_empty(&self) -> bool { *self.root() == C::HASHED_NULL_NODE }
}

fn main() { }
