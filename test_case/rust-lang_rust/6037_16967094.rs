 rust
cond!(
    if x >= 0 => x / y,
    if y >  0 => { ((x + 1) / y) - 1 }
    _         => ((x + 1) / y) + 1,
)
