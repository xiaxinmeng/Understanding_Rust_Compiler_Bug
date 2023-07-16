rust
trait T {
    type A: S<C<i32 = ()> = ()>;
}

trait Q {}

trait S {
    type C: Q;
}

fn main() {}
