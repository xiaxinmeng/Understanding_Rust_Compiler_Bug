rust
struct NotCopy;

trait Marker {}
impl<'a> Marker for &'a NotCopy {}
impl Marker for () {}

fn lifetime_issue() {
    let nc = NotCopy;
    let _foo = foo(&nc);
    drop(nc); // do something with nc
}

fn foo(_: impl Marker) -> impl Marker {}
