
fn  main() -> () {
    let mut _0: ();                      // return place
    scope 1 {
    }
    scope 2 {
        let _1: i32;                     // "_val" in scope 2 at src/main.rs:14:9: 14:13
    }
    let mut _2: isize;
    let mut _3: (isize, bool);

    bb0: {
        StorageLive(_1);                 // bb0[0]: scope 0 at src/main.rs:14:9: 14:13
        StorageLive(_2);                 // bb0[1]: scope 0 at src/main.rs:14:16: 14:47
        _3 = CheckedAdd(const Unevaluated(DefId(2/1:7942 ~ core[e194]::cmp[0]::Ordering[0]::Less[0]::{{constant}}[0]), []) : isize, const 0isize); // bb0[2]: scope 0 at src/main.rs:14:16: 14:47
                                         // ty::Const
                                         // + ty: isize
                                         // + val: Unevaluated(DefId(2/1:7942 ~ core[e194]::cmp[0]::Ordering[0]::Less[0]::{{constant}}[0]), [])
                                         // mir::Constant
                                         // + span: src/main.rs:14:16: 14:47
                                         // + ty: isize
                                         // + literal: Const { ty: isize, val: Unevaluated(DefId(2/1:7942 ~ core[e194]::cmp[0]::Ordering[0]::Less[0]::{{constant}}[0]), []) }
                                         // ty::Const
                                         // + ty: isize
                                         // + val: Scalar(Bits { size: 8, bits: 0 })
                                         // mir::Constant
                                         // + span: src/main.rs:14:16: 14:47
                                         // + ty: isize
                                         // + literal: Const { ty: isize, val: Scalar(Bits { size: 8, bits: 0 }) }
        assert(!move (_3.1: bool), "attempt to add with overflow") -> bb1; // bb0[3]: scope 0 at src/main.rs:14:16: 14:47
    }

    bb1: {
        _2 = move (_3.0: isize);         // bb1[0]: scope 0 at src/main.rs:14:16: 14:47
        _1 = move _2 as i32 (Misc);      // bb1[1]: scope 0 at src/main.rs:14:16: 14:47
        StorageDead(_2);                 // bb1[2]: scope 0 at src/main.rs:14:46: 14:47
        StorageDead(_1);                 // bb1[3]: scope 0 at src/main.rs:15:1: 15:2
        return;                          // bb1[4]: scope 0 at src/main.rs:15:2: 15:2
    }
}
