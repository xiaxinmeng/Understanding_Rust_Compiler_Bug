rust
// MIR for `main::{{closure}}#0`
// source = MirSource { instance: Item(DefId(0:11 ~ main[317d]::main[0]::{{closure}}[0])), promoted: None }
// pass_name = generator_resume
// disambiguator = 0
// generator_layout = GeneratorLayout { field_tys: [Dropper], variant_fields: [[], [], [], [_0]], storage_conflicts: BitMatrix { num_rows: 1, num_columns: 1, words: [1], marker: PhantomData } }

fn  main::{{closure}}#0(_1: std::pin::Pin<&mut [generator@main.rs:15:17: 17:6 {Dropper, (), Dropper}]>, _2: Dropper) -> std::ops::GeneratorState<(), ()> {
    debug d => (((*(_1.0: &mut [generator@main.rs:15:17: 17:6 {Dropper, (), Dropper}])) as variant#3).0: Dropper); // in scope 0 at main.rs:15:18: 15:23
    let mut _0: std::ops::GeneratorState<(), ()>; // return place in scope 0 at main.rs:15:17: 17:6
    let mut _3: Dropper;                 // in scope 0 at main.rs:16:13: 16:18
    let mut _4: ();                      // in scope 0 at main.rs:16:13: 16:18
    let mut _5: ();                      // in scope 0 at main.rs:15:25: 15:25
    let mut _6: isize;                   // in scope 0 at main.rs:15:17: 17:6

    bb0: {
        _6 = discriminant((*(_1.0: &mut [generator@main.rs:15:17: 17:6 {Dropper, (), Dropper}]))); // bb0[0]: scope 0 at main.rs:15:17: 17:6
        switchInt(move _6) -> [0u32: bb1, 1u32: bb12, 2u32: bb13, 3u32: bb11, otherwise: bb14]; // bb0[1]: scope 0 at main.rs:15:17: 17:6
    }

    bb1: {
        StorageLive(_3);                 // bb1[0]: scope 0 at main.rs:16:13: 16:18
        StorageLive(_4);                 // bb1[1]: scope 0 at main.rs:16:13: 16:18
        _4 = ();                         // bb1[2]: scope 0 at main.rs:16:13: 16:18
        _0 = std::ops::GeneratorState::<(), ()>::Yielded(move _4,); // bb1[3]: scope 0 at main.rs:16:13: 16:18
        discriminant((*(_1.0: &mut [generator@main.rs:15:17: 17:6 {Dropper, (), Dropper}]))) = 3; // bb1[4]: scope 0 at main.rs:16:13: 16:18
        return;                          // bb1[5]: scope 0 at main.rs:16:13: 16:18
    }

    bb2 (cleanup): {
        discriminant((*(_1.0: &mut [generator@main.rs:15:17: 17:6 {Dropper, (), Dropper}]))) = 2; // bb2[0]: scope 0 at main.rs:15:17: 17:6
        resume;                          // bb2[1]: scope 0 at main.rs:15:17: 17:6
    }

    bb3: {
        StorageDead(_4);                 // bb3[0]: scope 0 at main.rs:16:17: 16:18
        drop((((*(_1.0: &mut [generator@main.rs:15:17: 17:6 {Dropper, (), Dropper}])) as variant#3).0: Dropper)) -> [return: bb8, unwind: bb7]; // bb3[1]: scope 0 at main.rs:16:9: 16:10
    }

    bb4 (cleanup): {
        goto -> bb9;                     // bb4[0]: scope 0 at main.rs:17:5: 17:6
    }

    bb5: {
        goto -> bb10;                    // bb5[0]: scope 0 at main.rs:17:5: 17:6
    }

    bb6: {
        _0 = std::ops::GeneratorState::<(), ()>::Complete(move _5,); // bb6[0]: scope 0 at main.rs:17:6: 17:6
        discriminant((*(_1.0: &mut [generator@main.rs:15:17: 17:6 {Dropper, (), Dropper}]))) = 1; // bb6[1]: scope 0 at main.rs:17:6: 17:6
        return;                          // bb6[2]: scope 0 at main.rs:17:6: 17:6
    }

    bb7 (cleanup): {
        (((*(_1.0: &mut [generator@main.rs:15:17: 17:6 {Dropper, (), Dropper}])) as variant#3).0: Dropper) = move _3; // bb7[0]: scope 0 at main.rs:16:9: 16:10
        StorageDead(_3);                 // bb7[1]: scope 0 at main.rs:16:17: 16:18
        drop((((*(_1.0: &mut [generator@main.rs:15:17: 17:6 {Dropper, (), Dropper}])) as variant#3).0: Dropper)) -> bb4; // bb7[2]: scope 0 at main.rs:17:5: 17:6
    }

    bb8: {
        (((*(_1.0: &mut [generator@main.rs:15:17: 17:6 {Dropper, (), Dropper}])) as variant#3).0: Dropper) = move _3; // bb8[0]: scope 0 at main.rs:16:9: 16:10
        StorageDead(_3);                 // bb8[1]: scope 0 at main.rs:16:17: 16:18
        _5 = ();                         // bb8[2]: scope 0 at main.rs:15:25: 17:6
        drop((((*(_1.0: &mut [generator@main.rs:15:17: 17:6 {Dropper, (), Dropper}])) as variant#3).0: Dropper)) -> [return: bb5, unwind: bb2]; // bb8[3]: scope 0 at main.rs:17:5: 17:6
    }

    bb9 (cleanup): {
        goto -> bb2;                     // bb9[0]: scope 0 at main.rs:17:5: 17:6
    }

    bb10: {
        goto -> bb6;                     // bb10[0]: scope 0 at main.rs:17:5: 17:6
    }

    bb11: {
        StorageLive(_3);                 // bb11[0]: scope 0 at main.rs:15:17: 17:6
        StorageLive(_4);                 // bb11[1]: scope 0 at main.rs:15:17: 17:6
        _3 = move _2;                    // bb11[2]: scope 0 at main.rs:15:17: 17:6
        goto -> bb3;                     // bb11[3]: scope 0 at main.rs:15:17: 17:6
    }

    bb12: {
        assert(const false, "generator resumed after completion") -> bb12; // bb12[0]: scope 0 at main.rs:15:17: 17:6
                                         // ty::Const
                                         // + ty: bool
                                         // + val: Value(Scalar(0x00))
                                         // mir::Constant
                                         // + span: main.rs:15:17: 17:6
                                         // + literal: Const { ty: bool, val: Value(Scalar(0x00)) }
    }

    bb13: {
        assert(const false, "generator resumed after panicking") -> bb13; // bb13[0]: scope 0 at main.rs:15:17: 17:6
                                         // ty::Const
                                         // + ty: bool
                                         // + val: Value(Scalar(0x00))
                                         // mir::Constant
                                         // + span: main.rs:15:17: 17:6
                                         // + literal: Const { ty: bool, val: Value(Scalar(0x00)) }
    }

    bb14: {
        unreachable;                     // bb14[0]: scope 0 at main.rs:15:17: 17:6
    }
}
