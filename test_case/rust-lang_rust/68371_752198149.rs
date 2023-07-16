rust
iter::once(one()).chain((1..).scan(one(), |state: &mut BigUint, x: usize| {
    *state *= x;
    Some(state.clone())
}))
