rust
extern crate itertools;
use itertools::Itertools;

fn main() {
    let _ = vec![(0, 0)]
        .iter()
        .group_by(|(x, _)| x)
        .into_iter()
        .filter(|(_, mut _x)| true)
        .map(|(x, _)| x)
        .collect::<Vec<_>>();
}
