rust
fn bug<T>() -> impl Iterator<Item = [(); { |x: u32| { x }; 4 }]> {
    std::iter::empty()
}

fn ok<T>() -> Box<dyn Iterator<Item = [(); { |x: u32| { x }; 4 }]>> {
    Box::new(std::iter::empty())
}
