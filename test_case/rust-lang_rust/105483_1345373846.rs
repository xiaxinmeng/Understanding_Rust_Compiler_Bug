rust
struct Sequence;

impl<Idx> std::ops::Index<Idx> for Sequence
where
    Idx: std::slice::SliceIndex<[u8]>,
{
    type Output = Idx::Output;

    fn index(&self, index: Idx) -> &Self::Output {
        todo!()
    }
}

impl<'a,Idx> std::ops::Index<Idx> for &'a Sequence
where
    Idx: std::slice::SliceIndex<[u8]>,
{
    type Output = Idx::Output;

    fn index(&self, index: Idx) -> &'a Self::Output {
        todo!()
    }
}
