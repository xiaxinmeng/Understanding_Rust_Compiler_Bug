rust
#![feature(generic_const_exprs)]
#![feature(associated_type_defaults)]
trait OnChain {
    const len: usize;
    type OnChainT = [u8;Self::len];
}
