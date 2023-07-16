rust
let mut iter = std::iter::empty().chain(vec![0, 0]);
iter.next();        // to change the state to ChainState::Back
iter.nth_back(100); // to empty the iterator
assert_eq!(iter.next(), None);
