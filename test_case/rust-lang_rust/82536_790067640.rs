rust
let mut x = 1;
let c = || {
    drop(&mut x);
    match x { _ => () }
};
