Rust
struct Params;

pub trait Plugin<E: ?Sized> {
    type Error;
}

pub trait Pluggable {
    fn get_ref<P: Plugin<Self>>(&mut self) -> Option<P::Error> {
        None
    }
}

impl Plugin<()> for Params {
    type Error = ();
}

impl Pluggable for i32 {}

fn handle(req: &mut i32) {
    req.get_ref::<Params>();
}

fn main() {}
