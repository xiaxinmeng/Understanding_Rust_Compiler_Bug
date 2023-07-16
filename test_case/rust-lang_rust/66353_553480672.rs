rust
trait _Func<T> {
    fn func(_: Self);
}

trait _A {
    type AssocT;
}

fn crash() {
    _Func::< <() as _A>::AssocT >::func(());
}
