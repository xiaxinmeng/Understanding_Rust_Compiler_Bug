 rust
pub fn fail(x: Option<& (Iterator+Send)>) -> Option<&Iterator> {
    inner(x)
}

pub fn inner(x: Option<& (Iterator+Send)>) -> Option<&(Iterator+Send)> {
    x
}

fn main() {}
