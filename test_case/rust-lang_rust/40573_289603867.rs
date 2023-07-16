Rust
#![crate_type="rlib"]

pub struct Big {
    drop_me: [Option<Box<u8>>; 1024]
}

pub fn supersize_me(big: Big, may_panic: fn(), cond: bool) {
    may_panic();
    let _big2 = big;
    may_panic();
    if cond {
        let _big3 = _big2;
        may_panic();
        let _big4 = _big3;
        may_panic();
        let _big5 = _big4;
        may_panic();
        let _big6 = _big5;
    } else {
        let _big7 = _big2;
        may_panic();
        let _big8 = _big7;
        may_panic();
        let _big9 = _big8;
        may_panic();
        let _bigA = _big9;
    }
}
