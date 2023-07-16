rust
let (items, errors): (Vec<_>, Vec<_>) = data.variants.iter().map(FromVariant::from_variant).collect();
