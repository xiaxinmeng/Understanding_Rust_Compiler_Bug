
/// A
for elt in slice.iter_mut() { *elt = value; }

/// B
let mut sum = 0; /* must be an integer, not float (side track) */
for elt in slice.iter() { sum += *elt; }
