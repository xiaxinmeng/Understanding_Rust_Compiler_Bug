rust
fn make_me_a_float() -> f64 {
    let mut _0: f64;                     // return place in scope 0 at /app/example.rs:1:29: 1:32

    bb0: {
        _0 = const 0f64;                 // scope 0 at /app/example.rs:2:5: 2:16
        return;                          // scope 0 at /app/example.rs:3:2: 3:2
    }
}
