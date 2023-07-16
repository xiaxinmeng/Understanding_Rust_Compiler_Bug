rust
for<'a> |first: &mut &'a T| {
    let second = yield;
    mem::swap(first, second);
}
