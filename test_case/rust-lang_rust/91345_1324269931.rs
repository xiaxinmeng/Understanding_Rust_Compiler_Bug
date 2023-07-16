rust
let x: Option<i32> = option
    .tap!(|x| dbg!(x))
    .tap_matches!(|Some(x)| Some(x + 1))
    .inspect!(|x| println!("{x}"))
    .inspect_matches!(|None| println!("x is None"));
