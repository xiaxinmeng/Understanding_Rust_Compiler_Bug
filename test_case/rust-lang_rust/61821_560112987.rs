
promoted[0] in  main: i32 = {
    let mut _0: i32;                     // return place in scope 0 at src/main.rs:3:5: 3:20
    let mut _1: i32;                     // in scope 0 at src/main.rs:3:6: 3:20
    let mut _2: [i32; 3];                // in scope 0 at src/main.rs:3:7: 3:16
    let mut _3: usize;                   // in scope 0 at src/main.rs:3:17: 3:18

    bb0: {
        _2 = [const 1i32, const 2i32, const 3i32]; // bb0[0]: scope 0 at src/main.rs:3:7: 3:16
        _3 = const 4usize;               // bb0[1]: scope 0 at src/main.rs:3:17: 3:18
        _1 = _2[_3];                     // bb0[2]: scope 0 at src/main.rs:3:7: 3:19
        _0 = move _1;                    // bb0[3]: scope 0 at src/main.rs:3:5: 3:20
        return;                          // bb0[4]: scope 0 at src/main.rs:3:5: 3:20
    }
}
