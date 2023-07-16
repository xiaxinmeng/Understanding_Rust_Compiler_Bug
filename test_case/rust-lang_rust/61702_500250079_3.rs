
fn  main() -> () {
    let mut _0: ();                      // return place
    scope 1 {
        scope 3 {
        }
        scope 4 {
            let _2: i32;                 // "_val" in scope 4 at src/main.rs:3:9: 3:13
        }
    }
    scope 2 {
        let _1: std::cmp::Ordering;      // "val" in scope 2 at src/main.rs:2:9: 2:12
    }
    let mut _3: std::cmp::Ordering;

    bb0: {
        StorageLive(_1);                 // bb0[0]: scope 0 at src/main.rs:2:9: 2:12
        discriminant(_1) = 0;            // bb0[1]: scope 0 at src/main.rs:2:15: 2:39
        StorageLive(_2);                 // bb0[2]: scope 1 at src/main.rs:3:9: 3:13
        StorageLive(_3);                 // bb0[3]: scope 1 at src/main.rs:3:16: 3:19
        _3 = _1;                         // bb0[4]: scope 1 at src/main.rs:3:16: 3:19
        _2 = move _3 as i32 (Misc);      // bb0[5]: scope 1 at src/main.rs:3:16: 3:26
        StorageDead(_3);                 // bb0[6]: scope 1 at src/main.rs:3:25: 3:26
        StorageDead(_2);                 // bb0[7]: scope 1 at src/main.rs:4:1: 4:2
        StorageDead(_1);                 // bb0[8]: scope 0 at src/main.rs:4:1: 4:2
        return;                          // bb0[9]: scope 0 at src/main.rs:4:2: 4:2
    }
}
