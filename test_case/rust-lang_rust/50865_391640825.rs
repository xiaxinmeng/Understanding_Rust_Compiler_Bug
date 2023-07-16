rust
// lib.rs
pub trait Baz{}
pub struct Bar;
impl Baz for Bar{}

pub fn bar<P: Baz>( // Error won't happen if "bar" is not generic
    _baz: P,
) {
    let draw_data: Vec<u32> = iterate().collect();
    
}

fn iterate() -> impl Iterator<Item=u32> { // Error won't happen if "iterate" hasn't impl Trait
    std::iter::once(0).map(foo)
}


fn foo(input: u32) -> u32 { // Error won't happen if "foo" isn't used in "iterate"
    input
}
