rust
use std::cmp::Ordering;

#[derive(Debug, Eq, PartialEq, Ord, Copy, Clone)]
struct Coord {
    x: u32,
    y: u32,
}

impl PartialOrd for Coord {
    fn partial_cmp(&self, other: &Coord) -> Option<Ordering> {
        if self.x == other.x && self.y == other.y { Some(Ordering::Equal) }
        else if self.x <= other.x && self.y <= other.y { Some(Ordering::Less) }
        else if self.x >= other.x && self.y >= other.y { Some(Ordering::Greater) }
        else { None }
    }
}

fn main() {
    let c1 = Coord { x: 1, y: 2 };
    let c2 = Coord { x: 2, y: 1 };

    let mut v1 = vec![c1, c2];
    let mut v2 = vec![c2, c1];

    v1.sort();
    v2.sort();

    assert_eq!(v1, v2);
}
