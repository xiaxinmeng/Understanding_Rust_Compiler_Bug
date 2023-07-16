
fn bar() -> i32{
    let mut _0: i32;
    scope 1 {
    }
    scope 2 {
        let _1: fn(i32, i32) -> i32 {foo};
    }
    let mut _2: fn(i32, i32) -> i32 {foo};
    bb0: {                              
        StorageLive(_1);
        _1 = const foo;
        StorageLive(_2);
        _2 = _1;
        _0 = move _2(const 1i32, const -1i32) -> bb1;
    }
    bb1: {                              
        StorageDead(_2);
        StorageDead(_1);
        return;
    }
    bb2: {
        resume;
    }
}
