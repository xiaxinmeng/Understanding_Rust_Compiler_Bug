
#![feature(self_in_typedefs, box_syntax)]
enum PeanoNumber {
    Zero,
    Succ(Box<Self>),
}
impl PeanoNumber {
    fn add_one(n: Self) -> Self {
        Self::Succ(box n)
    }
}
