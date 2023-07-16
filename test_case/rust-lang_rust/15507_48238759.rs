
// Doubling sequence: 1, 2, 4, 8, ..
std::iter::Unfold::new(1i, |st| {let elt = *st; *st *= 2; Some(elt) });
