
// MIR for `main`
// node_id = 18
// pass_name = TypeckMir
// disambiguator = before

fn main() -> () {
    let mut _0: ();                      // return pointer
    let mut _1: std::option::Option<T>;
    let mut _2: T;
    let mut _3: ();

    bb0: {
        StorageLive(_2);                 // scope 0 at xfail.rs:14:14: 14:17
        _2 = T;                          // scope 0 at xfail.rs:14:14: 14:17
        _1 = std::option::Option<T>::Some(_2,); // scope 0 at xfail.rs:14:9: 14:18
        drop(_1) -> [return: bb3, unwind: bb2]; // scope 0 at xfail.rs:14:9: 14:18
    }

    bb1: {
        resume;                          // scope 0 at xfail.rs:12:1: 16:2
    }

    bb2: {
        drop(_2) -> bb1;                 // scope 0 at xfail.rs:14:18: 14:18
    }

    bb3: {
        drop(_2) -> bb4;                 // scope 0 at xfail.rs:14:18: 14:18
    }

    bb4: {
        StorageDead(_2);                 // scope 0 at xfail.rs:14:18: 14:18
        _0 = ();                         // scope 0 at xfail.rs:13:5: 15:6
        return;                          // scope 0 at xfail.rs:16:2: 16:2
    }
}
rustc.node18.TypeckMir.before.mir (END)
