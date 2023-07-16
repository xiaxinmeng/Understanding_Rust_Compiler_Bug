rust
// doesn't work
add1.call_fn(5);
// works, but at what cost
(add1 as fn(_) -> _).call_fn(5);
