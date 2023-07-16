rust
struct S {}

trait T<'a> {
    type A;
}

impl T<'_> for S {
    type A = i32;
}

fn foo<'a>(x: impl Fn(<S as T>::A) -> <S as T<'a>>::A) {
    help(x, 2i32);
}

fn help<'a, TT: T<'a>, F>(f: F, t: <TT as T<'a>>::A) -> <TT as T<'a>>::A
where
    F: Fn(<TT as T>::A) -> <TT as T<'a>>::A,
{
    f(t)
}

fn main() {
    foo(|x| x)
}
