rust
let b: &dyn for<'a> FnMut(&'a usize) -> bool =
        &compose_mut(predicate, bool::not);
