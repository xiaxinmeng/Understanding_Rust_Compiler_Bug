 rust
let (r, g, b) = rgb();
let rgbas: Vec<_> = alphas.into_iter()
    .flat_map(|a| [r, g, b, a].iter().cloned()) // This fails due to an insufficient lifetime
    // .flat_map(|a| [r, g, b, a]) // This would be sooo nice
    .collect();
