rust
trait SizeBiggerFour: Sized {
    const VALUE: bool = std::mem::size_of::<Self>() > 4;
}

impl<T> SizeBiggerFour for T {}

pub fn foo<T>() -> i32 { if <T as SizeBiggerFour>::VALUE { 3 } else { 5 } }
pub fn bar() -> i32 { foo::<isize>() }
