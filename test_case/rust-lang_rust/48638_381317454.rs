rust
pub trait D {}
pub struct DT;
impl D for DT {}

pub trait A<R: D>: Sized {
    type AS;
}

pub struct As<R: D>(R);

pub struct AT;
impl<R: D> A<R> for AT {
    type AS = As<R>;
}

#[repr(packed)] // comment to make this pass
struct S(<AT as A<DT>>::AS);

fn main() {}
