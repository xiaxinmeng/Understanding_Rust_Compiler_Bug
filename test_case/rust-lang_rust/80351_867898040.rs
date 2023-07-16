rust
trait Viewable<'a> {
    type View;
}

impl<'a> Viewable<'a> for u64 {
    type View = u64;
}

fn main() {
    let _f: for<'a> fn(<u64 as Viewable<'a>>::View) = |_| {};
}
