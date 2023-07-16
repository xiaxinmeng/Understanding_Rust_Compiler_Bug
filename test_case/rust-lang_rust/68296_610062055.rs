rust
// MIR for `FOO` after CopyPropagation

const FOO: *const u32 = {
    let mut _0: *const u32;              // return place in scope 0 at src/test/ui/consts/dangling_raw_ptr.rs:1:12: 1:22
    let _1: u32;                         // in scope 0 at src/test/ui/consts/dangling_raw_ptr.rs:2:9: 2:10
    let _2: &u32;                        // in scope 0 at src/test/ui/consts/dangling_raw_ptr.rs:3:5: 3:7
    scope 1 {
        debug x => _1;                   // in scope 1 at src/test/ui/consts/dangling_raw_ptr.rs:2:9: 2:10
    }

    bb0: {
        nop;                             // bb0[0]: scope 0 at src/test/ui/consts/dangling_raw_ptr.rs:2:9: 2:10
        _1 = const 42u32;                // bb0[1]: scope 0 at src/test/ui/consts/dangling_raw_ptr.rs:2:13: 2:15
                                         // ty::Const
                                         // + ty: u32
                                         // + val: Value(Scalar(0x0000002a))
                                         // mir::Constant
                                         // + span: src/test/ui/consts/dangling_raw_ptr.rs:2:13: 2:15
                                         // + literal: Const { ty: u32, val: Value(Scalar(0x0000002a)) }
        StorageLive(_2);                 // bb0[2]: scope 1 at src/test/ui/consts/dangling_raw_ptr.rs:3:5: 3:7
        _2 = &_1;                        // bb0[3]: scope 1 at src/test/ui/consts/dangling_raw_ptr.rs:3:5: 3:7
        _0 = &raw const (*_2);           // bb0[4]: scope 1 at src/test/ui/consts/dangling_raw_ptr.rs:3:5: 3:7
        nop;                             // bb0[5]: scope 0 at src/test/ui/consts/dangling_raw_ptr.rs:4:1: 4:2
        StorageDead(_2);                 // bb0[6]: scope 0 at src/test/ui/consts/dangling_raw_ptr.rs:4:1: 4:2
        return;                          // bb0[7]: scope 0 at src/test/ui/consts/dangling_raw_ptr.rs:1:1: 4:3
    }
}
