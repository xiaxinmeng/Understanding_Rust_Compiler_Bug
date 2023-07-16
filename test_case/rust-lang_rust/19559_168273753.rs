
trait T {
    type U;
}

struct S;

impl T for S {
    type U = i32;
}

fn main() {
    let _: <S as T>::U = 43;  // Works today.
    let _: S::U = 43;  // Doesn't work today but should, this issue.
}
