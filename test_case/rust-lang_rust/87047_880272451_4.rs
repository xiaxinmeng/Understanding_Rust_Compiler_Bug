rust
let mut items = Vec::with_capacity(data.variants.len());
let mut errors = Vec::new();
for v_result in data.variants.iter().map(FromVariant::from_variant) {
    match v_result {
        Ok(val) => items.push(val),
        Err(err) => errors.push(err),
    }
}
