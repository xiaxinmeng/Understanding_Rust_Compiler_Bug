 rust
let to_cap_by_ref = 0;
let to_cap_by_val = 0;

let explicit_ref = &to_cap_by_ref;
foo.map(move |x| {
    to_cap_by_val + *explicit_ref;
});
