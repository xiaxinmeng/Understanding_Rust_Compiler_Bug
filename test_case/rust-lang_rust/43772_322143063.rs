
fn main() -> () {
    let mut _0: ();                      // return pointer
    scope 1 {
        let _1: std::boxed::Box<i32>;    // "x" in scope 1 at src/main.rs:4:9: 4:10
    }
    let mut _2: std::boxed::Box<i32>;

    bb0: {
        StorageLive(_1);                 // scope 0 at src/main.rs:4:9: 4:10
        _2 = Box(i32);                   // scope 0 at src/main.rs:4:13: 4:21
        (*_2) = const 4i32;              // scope 0 at src/main.rs:4:17: 4:21
        _1 = _2;                         // scope 0 at src/main.rs:4:13: 4:21
        StorageDead(_2);                 // scope 0 at src/main.rs:4:21: 4:21
        _0 = ();                         // scope 0 at src/main.rs:3:11: 5:2
        drop(_1) -> bb1;                 // scope 0 at src/main.rs:5:2: 5:2
    }

    bb1: {
        StorageDead(_1);                 // scope 0 at src/main.rs:5:2: 5:2
        return;                          // scope 0 at src/main.rs:5:2: 5:2
    }
}
