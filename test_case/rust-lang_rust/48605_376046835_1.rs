rust
fn main::{{closure}}(_1: [closure@.\src\test\run-pass\issue-16671.rs:19:9: 21:6 var:std::vec::Vec<i32>]) -> (){
    let mut _0: ();                      // return place
    let mut _2: ();
    let mut _3: &mut std::vec::Vec<i32>;

    bb0: {                              
        StorageLive(_3);                 // bb0[0]: scope 0 at .\src\test\run-pass\issue-16671.rs:20:9: 20:12
        _3 = &mut (_1.0: std::vec::Vec<i32>); // bb0[1]: scope 0 at .\src\test\run-pass\issue-16671.rs:20:9: 20:12
        _2 = const <std::vec::Vec<T>>::push(move _3, const 1i32) -> [return: bb2, unwind: bb3]; // bb0[2]: scope 0 at .\src\test\run-pass\issue-16671.rs:20:9: 20:20
                                         // ty::Const
                                         // + ty: for<'r> fn(&'r mut std::vec::Vec<i32>, i32) {<std::vec::Vec<T>><i32>::push}
                                         // + val: Value(ByVal(Undef))
                                         // mir::Constant
                                         // + span: .\src\test\run-pass\issue-16671.rs:20:9: 20:20
                                         // + ty: for<'r> fn(&'r mut std::vec::Vec<i32>, i32) {<std::vec::Vec<T>><i32>::push}
                                         // + literal: const <std::vec::Vec<T>>::push
                                         // ty::Const
                                         // + ty: i32
                                         // + val: Value(ByVal(Bytes(1)))
                                         // mir::Constant
                                         // + span: .\src\test\run-pass\issue-16671.rs:20:18: 20:19
                                         // + ty: i32
                                         // + literal: const 1i32
    }

    bb1: {                               // cleanup
        resume;                          // bb1[0]: scope 0 at .\src\test\run-pass\issue-16671.rs:19:9: 21:6
    }

    bb2: {                              
        StorageDead(_3);                 // bb2[0]: scope 0 at .\src\test\run-pass\issue-16671.rs:20:19: 20:20
        _0 = ();                         // bb2[1]: scope 0 at .\src\test\run-pass\issue-16671.rs:19:16: 21:6
        drop(_1) -> [return: bb4, unwind: bb1]; // bb2[2]: scope 0 at .\src\test\run-pass\issue-16671.rs:21:5: 21:6
    }

    bb3: {                               // cleanup
        drop(_1) -> bb1;                 // bb3[0]: scope 0 at .\src\test\run-pass\issue-16671.rs:21:5: 21:6
    }

    bb4: {                              
        return;                          // bb4[0]: scope 0 at .\src\test\run-pass\issue-16671.rs:21:6: 21:6
    }
}
