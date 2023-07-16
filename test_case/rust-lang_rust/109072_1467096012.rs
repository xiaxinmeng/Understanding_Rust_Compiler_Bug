rust
trait Lt<'a> {
    type T;
}

impl Lt<'missing> for () {
//      ^^^^^^^^
    type T = ();
}

fn main() {
    let v:<() as Lt>::T = ();
}
