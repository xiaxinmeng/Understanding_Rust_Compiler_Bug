rust
struct S {}

trait T<'a>{
    type A;
}

impl T<'_> for S {
    type A = i32;
}

fn foo<'a>(x: impl Fn(<S as T>::A) -> <S as T<'a>>::A) {
    x(2i32);
}

fn main() {
    foo(|x| x);
}
