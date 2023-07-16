text
fn foo(_1: !) -> u32 {
    debug x => _1;                       // in scope 0 at src/lib.rs:3:8: 3:9
    let mut _0: u32;                     // return place in scope 0 at src/lib.rs:3:17: 3:20

    bb0: {
        unreachable;                     // scope 0 at src/lib.rs:3:23: 3:24
    }
}
