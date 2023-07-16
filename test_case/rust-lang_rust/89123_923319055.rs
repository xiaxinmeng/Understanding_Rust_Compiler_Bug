rust
let mut vec = Vec::new();
for value in iter {
    // push() will not reallocate
    vec.try_reserve(1).and_then(|v| v.push(value))?;
}
Ok(vec)
