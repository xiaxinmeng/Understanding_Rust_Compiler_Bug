
struct Dummy;

impl Iterator for Dummy {
    type Item = i32;
    fn next(&mut self) -> Option<Iterator::Item> {
    }
}

fn main() {
}
