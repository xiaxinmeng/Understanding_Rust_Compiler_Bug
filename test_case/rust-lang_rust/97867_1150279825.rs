rust
// if we infer `x` to be higher ranked in the future,
// this would cause a type error.
let x = match v {
    true => lt_in_fn_fn::<'a>(),
    false => lt_in_fn_fn::<'b>(),
};

let y: fn(fn(&'lower ())) = x;
