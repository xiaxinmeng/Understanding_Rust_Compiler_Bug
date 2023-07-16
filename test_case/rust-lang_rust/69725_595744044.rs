
use other_crate::Struct;

fn crash<A>() {
    let _ = Struct::<A>::new().clone();
}

fn main() {}
