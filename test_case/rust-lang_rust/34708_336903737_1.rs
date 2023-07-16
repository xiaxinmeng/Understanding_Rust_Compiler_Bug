
fn main() -> () {
    let mut _0: ();                      // return pointer
    scope 1 {
        let _1: NoisyDrop;               // "x" in scope 1 at src/main.rs:10:10: 10:11
    }
    scope 2 {
    }
    let mut _2: [NoisyDrop; 1];
    let mut _3: NoisyDrop;

    bb0: {                              
        StorageLive(_2);                 // scope 1 at src/main.rs:10:15: 10:26
        StorageLive(_3);                 // scope 1 at src/main.rs:10:16: 10:25
        _3 = NoisyDrop::{{constructor}}; // scope 1 at src/main.rs:10:16: 10:25
        _2 = [_3];                       // scope 1 at src/main.rs:10:15: 10:26
        StorageDead(_3);                 // scope 1 at src/main.rs:10:26: 10:26
        StorageLive(_1);                 // scope 1 at src/main.rs:10:10: 10:11
        // COMMENT MINE - value moved out here
        _1 = _2[0 of 1];                 // scope 1 at src/main.rs:10:10: 10:11
        drop(_2) -> [return: bb3, unwind: bb2]; // scope 1 at src/main.rs:10:27: 10:27
    }

    bb1: {                               // cleanup
        resume;                          // scope 0 at src/main.rs:9:1: 11:2
    }

    bb2: {                               // cleanup
        drop(_1) -> bb1;                 // scope 0 at src/main.rs:11:2: 11:2
    }

    bb3: {                              
        StorageDead(_2);                 // scope 1 at src/main.rs:10:27: 10:27
        _0 = ();                         // scope 0 at src/main.rs:9:11: 11:2
        // COMMENT MINE - ...and local is dropped at end of scope here
        drop(_1) -> bb4;                 // scope 0 at src/main.rs:11:2: 11:2
    }

    bb4: {                              
        StorageDead(_1);                 // scope 0 at src/main.rs:11:2: 11:2
        return;                          // scope 0 at src/main.rs:11:2: 11:2
    }
}
