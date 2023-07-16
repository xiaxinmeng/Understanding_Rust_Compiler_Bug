rust
struct LargeStruct([u8; 1024]);

fn caller() {
    let _1: LargeStruct;
    let _2: bool;

    bb0: {
        _1.0 = [0;1024];
        call callee(copy _1) -> bb1;
    }

    bb1: {
        _2 = _1.0[0] == 0;
        assert(move _2) -> bb2;
    }

    bb2: {
        return;
    }
}

fn callee(_1: LargeStruct) {
    bb0: {
        _1.0[0] = 42;
        return;
    }
}
