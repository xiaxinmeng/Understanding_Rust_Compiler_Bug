
fn main() -> () {
    let mut _0: ();                      // return place in scope 0 at main.rs:2:11: 2:11
    let _1: std::option::Option<u8>;     // in scope 0 at main.rs:3:9: 3:10
    let mut _2: !;                       // in scope 0 at main.rs:4:26: 6:6
    let _3: ();                          // in scope 0 at /home/ben/rust-master/library/std/src/panic.rs:19:9: 19:50
    let mut _4: !;                       // in scope 0 at /home/ben/rust-master/library/std/src/panic.rs:19:9: 19:50
    let mut _6: isize;                   // in scope 0 at main.rs:4:9: 4:16
    scope 1 {
        debug x => _1;                   // in scope 1 at main.rs:3:9: 3:10
        let _5: u8;                      // in scope 1 at main.rs:4:14: 4:15
        scope 2 {
            debug y => _5;               // in scope 2 at main.rs:4:14: 4:15
        }
    }

    bb0: {
        StorageLive(_1);                 // scope 0 at main.rs:3:9: 3:10
        Deinit(_1);                      // scope 0 at main.rs:3:25: 3:32
        ((_1 as Some).0: u8) = const 1_u8; // scope 0 at main.rs:3:25: 3:32
        discriminant(_1) = 1;            // scope 0 at main.rs:3:25: 3:32
        StorageLive(_5);                 // scope 1 at main.rs:4:14: 4:15
        _6 = discriminant(_1);           // scope 1 at main.rs:4:19: 4:20
        switchInt(move _6) -> [1_isize: bb1, otherwise: bb2]; // scope 1 at main.rs:4:9: 4:16
    }

    bb1: {
        StorageLive(_5);                 // scope 1 at main.rs:4:14: 4:15
        _5 = ((_1 as Some).0: u8);       // scope 1 at main.rs:4:14: 4:15
        _0 = const ();                   // scope 0 at main.rs:2:11: 7:2
        StorageDead(_5);                 // scope 1 at main.rs:7:1: 7:2
        StorageDead(_1);                 // scope 0 at main.rs:7:1: 7:2
        return;                          // scope 0 at main.rs:7:2: 7:2
    }

    bb2: {
        StorageDead(_5);                 // scope 1 at main.rs:7:1: 7:2
        StorageLive(_3);                 // scope 1 at /home/ben/rust-master/library/std/src/panic.rs:18:12: 20:6
        StorageLive(_4);                 // scope 1 at /home/ben/rust-master/library/std/src/panic.rs:19:9: 19:50
        _4 = begin_panic::<&str>(const "explicit panic"); // scope 1 at /home/ben/rust-master/library/std/src/panic.rs:19:9: 19:50
                                         // mir::Constant
                                         // + span: /home/ben/rust-master/library/std/src/panic.rs:19:9: 19:32
                                         // + literal: Const { ty: fn(&str) -> ! {begin_panic::<&str>}, val: Value(<ZST>) }
                                         // mir::Constant
                                         // + span: /home/ben/rust-master/library/std/src/panic.rs:19:33: 19:49
                                         // + literal: Const { ty: &str, val: Value(Slice(..)) }
    }
}
