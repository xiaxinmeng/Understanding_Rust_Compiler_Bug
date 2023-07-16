
// MIR for `add_self`
// source = MirSource { def_id: DefId(0/0:11 ~ bug[317d]::add_self[0]), promoted: None }
// pass_name = nll
// disambiguator = 0

| Free Region Mapping
| '_#0r    | Global   | ['_#0r, '_#1r]
| '_#1r    | Local    | ['_#1r]
|
| Inferred Region Values
| '_#0r    | {'_#0r, bb0[0..=4], bb1[0], bb2[0..=7]}
| '_#1r    | {'_#1r, bb0[0..=4], bb1[0], bb2[0..=7]}
| '_#2r    | {bb0[1..=4]}
| '_#3r    | {bb0[2..=4]}
| '_#4r    | {bb0[4]}
|
| Inference Constraints
| '_#0r live at {'_#0r, bb0[0..=4], bb1[0], bb2[0..=7]}
| '_#1r live at {'_#1r, bb0[0..=4], bb1[0], bb2[0..=7]}
| '_#2r live at {bb0[1]}
| '_#3r live at {bb0[2..=4]}
| '_#4r live at {bb0[4]}
| '_#2r: '_#3r @ bb0[2] due to bug.rs:24:5: 24:6
| '_#3r: '_#4r @ bb0[4] due to bug.rs:24:5: 24:11
fn add_self(_1: T) -> T{
    let mut _0: T;                       // return place
    let mut _2: ();
    let mut _3: &mut T;
    let mut _4: T;
    let mut _5: T;

    | Live variables on entry to bb0: [_1]
    bb0: {                              
                                         | Live variables on entry to bb0[0]: [_1]
        StorageLive(_3);                 // bb0[0]: scope 0 at bug.rs:24:5: 24:6
                                         | Live variables on entry to bb0[1]: [_1]
        _3 = &mut _1;                    // bb0[1]: scope 0 at bug.rs:24:5: 24:6
                                         | Live variables on entry to bb0[2]: [_1, _3]
        StorageLive(_4);                 // bb0[2]: scope 0 at bug.rs:24:10: 24:11
                                         | Live variables on entry to bb0[3]: [_1, _3]
        _4 = _1;                         // bb0[3]: scope 0 at bug.rs:24:10: 24:11
                                         | Live variables on entry to bb0[4]: [_1, _3, _4]
        _2 = const std::ops::AddAssign::add_assign(move _3, move _4) -> [return: bb2, unwind: bb1]; // bb0[4]: scope 0 at bug.rs:24:5: 24:11
                                         // ty::Const
                                         // + ty: for<'r> fn(&'r mut T, T) {<T as std::ops::AddAssign>::add_assign}
                                         // + val: Function(DefId(2/0:912 ~ core[d915]::ops[0]::arith[0]::AddAssign[0]::add_assign[0]), Slice([T, T]))
                                         // mir::Constant
                                         // + span: bug.rs:24:5: 24:11
                                         // + ty: for<'r> fn(&'r mut T, T) {<T as std::ops::AddAssign>::add_assign}
                                         // + literal: const std::ops::AddAssign::add_assign
    }

    | Live variables on entry to bb1: []
    bb1: {                               // cleanup
                                         | Live variables on entry to bb1[0]: []
        resume;                          // bb1[0]: scope 0 at bug.rs:22:1: 26:2
    }

    | Live variables on entry to bb2: [_1]
    bb2: {                              
                                         | Live variables on entry to bb2[0]: [_1]
        nop;                             // bb2[0]: scope 0 at bug.rs:24:5: 24:11
                                         | Live variables on entry to bb2[1]: [_1]
        StorageDead(_4);                 // bb2[1]: scope 0 at bug.rs:24:10: 24:11
                                         | Live variables on entry to bb2[2]: [_1]
        StorageDead(_3);                 // bb2[2]: scope 0 at bug.rs:24:10: 24:11
                                         | Live variables on entry to bb2[3]: [_1]
        StorageLive(_5);                 // bb2[3]: scope 0 at bug.rs:25:5: 25:6
                                         | Live variables on entry to bb2[4]: [_1]
        _5 = _1;                         // bb2[4]: scope 0 at bug.rs:25:5: 25:6
                                         | Live variables on entry to bb2[5]: [_5]
        _0 = move _5;                    // bb2[5]: scope 0 at bug.rs:25:5: 25:6
                                         | Live variables on entry to bb2[6]: []
        StorageDead(_5);                 // bb2[6]: scope 0 at bug.rs:25:5: 25:6
                                         | Live variables on entry to bb2[7]: []
        return;                          // bb2[7]: scope 0 at bug.rs:26:2: 26:2
    }
}
