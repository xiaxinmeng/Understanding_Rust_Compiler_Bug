rust
struct S {}

trait T<'a>{
    type A;
}

impl T<'_> for S {
    type A = u32;
}

fn foo(x: impl Fn(<S as T>::A) -> <S as T>::A) {
}
