
fn  main() -> () {
    let mut _0: ();                      // return place in scope 0 at test.rs:6:11: 6:11
    let _1: ();                          // in scope 0 at test.rs:7:5: 7:23
    let mut _2: ((), u8, u8);            // in scope 0 at test.rs:7:12: 7:22

    bb0: {
        StorageLive(_1);                 // bb0[0]: scope 0 at test.rs:7:5: 7:23
        StorageLive(_2);                 // bb0[1]: scope 0 at test.rs:7:12: 7:22
        (_2.0: ()) = const Scalar(0x00) : (); // bb0[2]: scope 0 at test.rs:7:12: 7:22
                                         // ty::Const
                                         // + ty: ()
                                         // + val: Value(Scalar(0x00))
                                         // mir::Constant
                                         // + span: test.rs:7:12: 7:22
                                         // + literal: Const { ty: (), val: Value(Scalar(0x00)) }
        (_2.1: u8) = const 0u8;          // bb0[3]: scope 0 at test.rs:7:12: 7:22
                                         // ty::Const
                                         // + ty: u8
                                         // + val: Value(Scalar(0x00))
                                         // mir::Constant
                                         // + span: test.rs:7:12: 7:22
                                         // + literal: Const { ty: u8, val: Value(Scalar(0x00)) }
        _1 = const encode(move _2) -> bb1; // bb0[4]: scope 0 at test.rs:7:5: 7:23
                                         // ty::Const
                                         // + ty: fn(((), u8, u8)) {encode}
                                         // + val: Value(Scalar(<ZST>))
                                         // mir::Constant
                                         // + span: test.rs:7:5: 7:11
                                         // + literal: Const { ty: fn(((), u8, u8)) {encode}, val: Value(Scalar(<ZST>)) }
    }

    bb1: {
        StorageDead(_2);                 // bb1[0]: scope 0 at test.rs:7:22: 7:23
        StorageDead(_1);                 // bb1[1]: scope 0 at test.rs:7:23: 7:24
        return;                          // bb1[2]: scope 0 at test.rs:8:2: 8:2
    }
}
