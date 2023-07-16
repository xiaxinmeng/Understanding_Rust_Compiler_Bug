
fn main::{{closure}}(_1: [closure@<anon>:2:36: 4:6]) -> () {
    let mut _0: ();                      // return pointer
    let mut _2: std::cell::RefCell<[i8; 22]>;
    let mut _3: [i8; 22];

    bb0: {
        StorageLive(_3);                 // scope 0 at <anon>:3:33: 3:42
        _3 = [const 0i8; const 22usize]; // scope 0 at <anon>:3:33: 3:42
        _2 = <std::cell::RefCell<T>><[i8; 22]>::new(_3) -> bb1; // scope 0 at <anon>:3:9: 3:43
    }

    bb1: {
        StorageDead(_3);                 // scope 0 at <anon>:3:33: 3:42
        _0 = ();                         // scope 0 at <anon>:2:39: 4:6
        return;                          // scope 0 at <anon>:2:36: 4:6
    }
}
