rust
// IntoIterator::zip
for (x, y) in xs.zip(ys) {}

// this or, iter::zip
for (x, y) in zip(xs, ys) {}

// current status
for (x, y) in xs.into_iter().zip(ys) {}
