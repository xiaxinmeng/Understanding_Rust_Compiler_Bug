
// MIR for `<cell::RefCell<T>>::new`
// node_id = 18741
// pass_name = PreTrans
// disambiguator = after

fn <cell::RefCell<T>>::new(_1: T) -> cell::RefCell<T> {
    let mut _0: cell::RefCell<T>;        // return pointer
    scope 1 {
        let _2: T;                       // "value" in scope 1 at ../../src/libcore/cell.rs:463:22: 463:27
    }
    let mut _3: cell::UnsafeCell<T>;
    let mut _4: ();
    let mut _5: T;
    let mut _6: cell::Cell<usize>;

    bb0: {
        StorageLive(_2);                 // scope 0 at ../../src/libcore/cell.rs:463:22: 463:27
        _2 = _1;                         // scope 0 at ../../src/libcore/cell.rs:463:22: 463:27
        StorageLive(_3);                 // scope 1 at ../../src/libcore/cell.rs:465:20: 465:42
        StorageLive(_5);                 // scope 1 at ../../src/libcore/cell.rs:465:36: 465:41
        _5 = _2;                         // scope 1 at ../../src/libcore/cell.rs:465:36: 465:41
        _3 = <cell::UnsafeCell<T>><T>::new(_5) -> [return: bb2, unwind: bb1]; // scope 1 at ../../src/libcore/cell.rs:465:20: 465:42
    }

    bb1: {
        resume;                          // scope 0 at ../../src/libcore/cell.rs:463:5: 468:6
    }

    bb2: {
        StorageLive(_6);                 // scope 1 at ../../src/libcore/cell.rs:466:21: 466:38
        _6 = <cell::Cell<T>><usize>::new(cell::UNUSED) -> [return: bb4, unwind: bb3]; // scope 1 at ../../src/libcore/cell.rs:466:21: 466:38
    }

    bb3: {
        drop(_3) -> bb1;                 // scope 1 at ../../src/libcore/cell.rs:465:20: 465:42
    }

    bb4: {
        _0 = cell::RefCell::<T> { borrow: _6, value: _3 }; // scope 1 at ../../src/libcore/cell.rs:464:9: 467:10
        StorageDead(_6);                 // scope 1 at ../../src/libcore/cell.rs:466:21: 466:38
        StorageDead(_3);                 // scope 1 at ../../src/libcore/cell.rs:465:20: 465:42
        StorageDead(_5);                 // scope 1 at ../../src/libcore/cell.rs:465:36: 465:41
        StorageDead(_2);                 // scope 0 at ../../src/libcore/cell.rs:463:22: 463:27
        return;                          // scope 1 at ../../src/libcore/cell.rs:463:5: 468:6
    }
}
