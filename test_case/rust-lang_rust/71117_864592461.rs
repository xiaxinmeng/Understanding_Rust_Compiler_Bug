rust
struct LargeStruct([u8; 1024]);

fn caller() {
    let _1: LargeStruct;
    let _2: bool;
    let _3: *mut LargeStruct;

    bb0: {
        _1.0 = [0;1024];
        _3 = &mut _1 as *mut _;
        call callee(move _1, copy _3) -> bb1;
    }

    bb1: {
        _2 = _1.0[0] == 1;
        assert(move _2) -> bb2;
    }

    bb2: {
        return;
    }
}

fn callee(_1: LargeStruct, _2: *mut LargeStruct) {
    bb0: {
        (*_2).0[0] = 1;
        _1.0[0] = 42;
        return;
    }
}
