rust
fn convert(_1: Color) -> [u8; 4] {
    debug color => _1;                   // in scope 0 at /app/example.rs:20:16: 20:21
    let mut _0: [u8; 4];                 // return place in scope 0 at /app/example.rs:20:33: 20:40
    let mut _2: Color;                   // in scope 0 at /app/example.rs:21:5: 21:10

    bb0: {
        StorageLive(_2);                 // scope 0 at /app/example.rs:21:5: 21:10
        _2 = move _1;                    // scope 0 at /app/example.rs:21:5: 21:10
        _0 = Color::to_rgba8(move _2) -> bb1; // scope 0 at /app/example.rs:21:5: 21:21
    }

    bb1: {
        StorageDead(_2);                 // scope 0 at /app/example.rs:21:20: 21:21
        return;                          // scope 0 at /app/example.rs:22:2: 22:2
    }
}
