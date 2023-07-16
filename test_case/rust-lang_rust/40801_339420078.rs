rust
match Err(E) { 
    Ok(v) => v, // yields an unconstrainted type variable `?T`
    Err(e) => return Err(e) // yields an unconstrained type variable `?U` with fallback
}
