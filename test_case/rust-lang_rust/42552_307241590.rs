
extern crate itertools;
use itertools::Itertools;

fn main() {
    [0].iter()
        .map(|n| (n, 0))
        .group_by(|_| 0)
        .into_iter()
        .map(|(_, grp)| 
            grp.into_iter()
                .map(|_| 0)
                .group_by(|_| 0)
                .into_iter()
                .map(|(_, grpinner)| grpinner.into_iter().max())
                .max()
        )
        .max()
        .unwrap();
}

