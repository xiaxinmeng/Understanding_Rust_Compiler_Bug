rust
#![feature(generic_const_exprs)]
#![feature(specialization)]

trait ValTypeSelect {
    type ValTypeInner: std::fmt::Display + From<i8>;
}

struct UseSmallInt<const N: bool> {}

impl<const N: bool> ValTypeSelect for UseSmallInt<N> {
    default type ValTypeInner = i32;
}
impl ValTypeSelect for UseSmallInt<true> {
    type ValTypeInner = i8;
}

struct MyStruct<const VAL: usize>
where
    [(); (VAL != 0) as usize]:,
{
    val: <UseSmallInt<{ VAL != 0 }> as ValTypeSelect>::ValTypeInner,
}

fn main() {
    let x = MyStruct::<0> { val: 0.into() };
    println!("{}", x.val);
}
