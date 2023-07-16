
// MIR for `gcd`
// source = MirSource { def_id: DefId(0/0:14 ~ bug[317d]::gcd[0]), promoted: None }
// pass_name = nll
// disambiguator = 0

| Free Region Mapping
| '_#0r    | Global   | ['_#0r, '_#1r]
| '_#1r    | Local    | ['_#1r]
|
| Inferred Region Values
| '_#0r    | {'_#0r, bb0[0..=5], bb1[0], bb2[0..=1], bb3[0..=7]}
| '_#1r    | {'_#1r, bb0[0..=5], bb1[0], bb2[0..=1], bb3[0..=7]}
| '_#2r    | {bb0[1..=5], bb2[0..=1]}
| '_#3r    | {bb0[2..=5], bb2[0..=1]}
| '_#4r    | {bb2[1]}
|
| Inference Constraints
| '_#0r live at {'_#0r, bb0[0..=5], bb1[0], bb2[0..=1], bb3[0..=7]}
| '_#1r live at {'_#1r, bb0[0..=5], bb1[0], bb2[0..=1], bb3[0..=7]}
| '_#2r live at {bb0[1]}
| '_#3r live at {bb0[2..=5], bb2[0..=1]}
| '_#4r live at {bb2[1]}
| '_#2r: '_#3r @ bb0[2] due to bug.rs:22:5: 22:6
| '_#3r: '_#4r @ bb2[1] due to bug.rs:22:5: 22:29
fn gcd(_1: T, _2: T) -> T{
    let mut _0: T;                       // return place
    let mut _3: ();
    let mut _4: &mut T;
    let mut _5: u32;
    let mut _6: T;
    let mut _7: T;

    | Live variables on entry to bb0: [_2]
    bb0: {                              
                                         | Live variables on entry to bb0[0]: [_2]
        StorageLive(_4);                 // bb0[0]: scope 0 at bug.rs:22:5: 22:6
                                         | Live variables on entry to bb0[1]: [_2]
        _4 = &mut _2;                    // bb0[1]: scope 0 at bug.rs:22:5: 22:6
                                         | Live variables on entry to bb0[2]: [_2, _4]
        StorageLive(_5);                 // bb0[2]: scope 0 at bug.rs:22:11: 22:29
                                         | Live variables on entry to bb0[3]: [_2, _4]
        StorageLive(_6);                 // bb0[3]: scope 0 at bug.rs:22:11: 22:12
                                         | Live variables on entry to bb0[4]: [_2, _4]
        _6 = _2;                         // bb0[4]: scope 0 at bug.rs:22:11: 22:12
                                         | Live variables on entry to bb0[5]: [_2, _4, _6]
        _5 = const HasTrailingZeros::trailing_zeros(move _6) -> [return: bb2, unwind: bb1]; // bb0[5]: scope 0 at bug.rs:22:11: 22:29
                                         // ty::Const
                                         // + ty: fn(T) -> u32 {<T as HasTrailingZeros>::trailing_zeros}
                                         // + val: Function(DefId(0/0:11 ~ bug[317d]::HasTrailingZeros[0]::trailing_zeros[0]), Slice([T]))
                                         // mir::Constant
                                         // + span: bug.rs:22:11: 22:29
                                         // + ty: fn(T) -> u32 {<T as HasTrailingZeros>::trailing_zeros}
                                         // + literal: const HasTrailingZeros::trailing_zeros
    }

    | Live variables on entry to bb1: []
    bb1: {                               // cleanup
                                         | Live variables on entry to bb1[0]: []
        resume;                          // bb1[0]: scope 0 at bug.rs:15:1: 25:2
    }

    | Live variables on entry to bb2: [_2, _4, _5]
    bb2: {                              
                                         | Live variables on entry to bb2[0]: [_2, _4, _5]
        StorageDead(_6);                 // bb2[0]: scope 0 at bug.rs:22:28: 22:29
                                         | Live variables on entry to bb2[1]: [_2, _4, _5]
        _3 = const std::ops::ShrAssign::shr_assign(move _4, move _5) -> [return: bb3, unwind: bb1]; // bb2[1]: scope 0 at bug.rs:22:5: 22:29
                                         // ty::Const
                                         // + ty: for<'r> fn(&'r mut T, u32) {<T as std::ops::ShrAssign<u32>>::shr_assign}
                                         // + val: Function(DefId(2/0:967 ~ core[d915]::ops[0]::bit[0]::ShrAssign[0]::shr_assign[0]), Slice([T, u32]))
                                         // mir::Constant
                                         // + span: bug.rs:22:5: 22:29
                                         // + ty: for<'r> fn(&'r mut T, u32) {<T as std::ops::ShrAssign<u32>>::shr_assign}
                                         // + literal: const std::ops::ShrAssign::shr_assign
    }

    | Live variables on entry to bb3: [_2]
    bb3: {                              
                                         | Live variables on entry to bb3[0]: [_2]
        nop;                             // bb3[0]: scope 0 at bug.rs:22:5: 22:29
                                         | Live variables on entry to bb3[1]: [_2]
        StorageDead(_5);                 // bb3[1]: scope 0 at bug.rs:22:28: 22:29
                                         | Live variables on entry to bb3[2]: [_2]
        StorageDead(_4);                 // bb3[2]: scope 0 at bug.rs:22:28: 22:29
                                         | Live variables on entry to bb3[3]: [_2]
        StorageLive(_7);                 // bb3[3]: scope 0 at bug.rs:24:5: 24:6
                                         | Live variables on entry to bb3[4]: [_2]
        _7 = _2;                         // bb3[4]: scope 0 at bug.rs:24:5: 24:6
                                         | Live variables on entry to bb3[5]: [_7]
        _0 = move _7;                    // bb3[5]: scope 0 at bug.rs:24:5: 24:6
                                         | Live variables on entry to bb3[6]: []
        StorageDead(_7);                 // bb3[6]: scope 0 at bug.rs:24:5: 24:6
                                         | Live variables on entry to bb3[7]: []
        return;                          // bb3[7]: scope 0 at bug.rs:25:2: 25:2
    }
}

