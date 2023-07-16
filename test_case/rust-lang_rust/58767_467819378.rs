diff
4c4
< // disambiguator = before
---
> // disambiguator = after
14,24c14
<         StorageLive(_3);                 // bb0[1]: scope 0 at src/lib.rs:12:9: 12:26
<         StorageLive(_4);                 // bb0[2]: scope 0 at src/lib.rs:12:9: 12:13
<         _4 = &(*_1);                     // bb0[3]: scope 0 at src/lib.rs:12:9: 12:13
<         _3 = const Hz::normalized(move _4) -> [return: bb2, unwind: bb1]; // bb0[4]: scope 0 at src/lib.rs:12:9: 12:26
<                                          // ty::Const
<                                          // + ty: for<'r> fn(&'r Hz) -> Hz {Hz::normalized}
<                                          // + val: Scalar(Bits { size: 0, bits: 0 })
<                                          // mir::Constant
<                                          // + span: src/lib.rs:12:14: 12:24
<                                          // + ty: for<'r> fn(&'r Hz) -> Hz {Hz::normalized}
<                                          // + literal: Evaluated(Const { ty: for<'r> fn(&'r Hz) -> Hz {Hz::normalized}, val: Scalar(Bits { size: 0, bits: 0 }) })
---
>         goto -> bb2;                     // bb0[1]: scope 0 at src/lib.rs:12:9: 12:26
32,34c22,23
<         _2 = &_3;                        // bb2[0]: scope 0 at src/lib.rs:12:9: 12:26
<         StorageDead(_4);                 // bb2[1]: scope 0 at src/lib.rs:12:25: 12:26
<         _0 = const Hz::num(move _2) -> [return: bb3, unwind: bb1]; // bb2[2]: scope 0 at src/lib.rs:12:9: 12:32
---
>         _2 = &(promoted[0]: Hz);         // bb2[0]: scope 0 at src/lib.rs:12:9: 12:26
>         _0 = const Hz::num(move _2) -> [return: bb3, unwind: bb1]; // bb2[1]: scope 0 at src/lib.rs:12:9: 12:32
46,47c35
<         StorageDead(_3);                 // bb3[1]: scope 0 at src/lib.rs:13:5: 13:6
<         return;                          // bb3[2]: scope 0 at src/lib.rs:13:6: 13:6
---
>         return;                          // bb3[1]: scope 0 at src/lib.rs:13:6: 13:6

