 rust
s.sort_by_key(Point::x);
// this is more readable
s.binary_search_by_key(&1, Point::x);
// than this
s.binary_search_by(|p| p.x().cmp(&1));
