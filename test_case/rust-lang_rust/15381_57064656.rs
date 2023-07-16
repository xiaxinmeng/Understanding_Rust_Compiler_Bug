
pub struct It;

impl Iterator<[uint, ..3]> for It {
    fn next(&mut self) -> Option<[uint, ..3]> {
        None
    }
}

fn main() {
    for [x, y, z] in It {
    }
}
