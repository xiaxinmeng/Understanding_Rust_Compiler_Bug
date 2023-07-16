rust
use std::iter::FromIterator;

fn main() {
    let mut scores = vec![1, 2, 3];
    let indices = BTreeSet::from_iter(vec![0, 2]);

    let selected_scores: Vec<_> = uniq_refs(&mut scores, &indices).collect();
    for score in selected_scores {
        *score += 1;
    }
}
