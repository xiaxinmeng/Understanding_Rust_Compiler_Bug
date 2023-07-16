rust
// Do we want this?
let d = Duration::zero();
if d.is_zero() { ... }

// Or rather this?
let d = Duration::ZERO;
if d == Duration::ZERO { ... }
