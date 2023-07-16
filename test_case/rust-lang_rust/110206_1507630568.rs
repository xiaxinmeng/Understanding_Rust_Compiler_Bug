`rust
mod BikeshedIntrinsicFrom {
    use std::mem::BikeshedIntrinsicFrom;
    pub fn is_transmutable<Src, Dst>()
    where
        Dst: BikeshedIntrinsicFrom<Src, Context, false, false, false, true>
    {}
}
