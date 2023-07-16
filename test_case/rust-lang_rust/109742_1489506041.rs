plain
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0618]: expected function, found `SimplifiedType`
    --> src/librustdoc/clean/types.rs:1748:33
     |
1748 |                 Isize => single(IntSimplifiedType(IntTy::Isize)),
     |                                 |
     |                                 call expression requires function

error[E0618]: expected function, found `SimplifiedType`
error[E0618]: expected function, found `SimplifiedType`
    --> src/librustdoc/clean/types.rs:1749:30
     |
1749 |                 I8 => single(IntSimplifiedType(IntTy::I8)),
     |                              |
     |                              call expression requires function

error[E0618]: expected function, found `SimplifiedType`
error[E0618]: expected function, found `SimplifiedType`
    --> src/librustdoc/clean/types.rs:1750:31
     |
1750 |                 I16 => single(IntSimplifiedType(IntTy::I16)),
     |                               |
     |                               call expression requires function

error[E0618]: expected function, found `SimplifiedType`
error[E0618]: expected function, found `SimplifiedType`
    --> src/librustdoc/clean/types.rs:1751:31
     |
1751 |                 I32 => single(IntSimplifiedType(IntTy::I32)),
     |                               |
     |                               call expression requires function

error[E0618]: expected function, found `SimplifiedType`
error[E0618]: expected function, found `SimplifiedType`
    --> src/librustdoc/clean/types.rs:1752:31
     |
1752 |                 I64 => single(IntSimplifiedType(IntTy::I64)),
     |                               |
     |                               call expression requires function

error[E0618]: expected function, found `SimplifiedType`
error[E0618]: expected function, found `SimplifiedType`
    --> src/librustdoc/clean/types.rs:1753:32
     |
1753 |                 I128 => single(IntSimplifiedType(IntTy::I128)),
     |                                |
     |                                call expression requires function


error[E0425]: cannot find function, tuple struct or tuple variant `UintSimplifiedType` in this scope
    --> src/librustdoc/clean/types.rs:1754:33
     |
1754 |                 Usize => single(UintSimplifiedType(UintTy::Usize)),
     |                                 ^^^^^^^^^^^^^^^^^^ help: a unit variant with a similar name exists: `IntSimplifiedType`
    ::: /checkout/compiler/rustc_middle/src/ty/fast_reject.rs:16:5
     |
16   |     IntSimplifiedType,
     |     ----------------- similarly named unit variant `IntSimplifiedType` defined here
     |     ----------------- similarly named unit variant `IntSimplifiedType` defined here

error[E0425]: cannot find function, tuple struct or tuple variant `UintSimplifiedType` in this scope
    --> src/librustdoc/clean/types.rs:1755:30
     |
1755 |                 U8 => single(UintSimplifiedType(UintTy::U8)),
     |                              ^^^^^^^^^^^^^^^^^^ help: a unit variant with a similar name exists: `IntSimplifiedType`
    ::: /checkout/compiler/rustc_middle/src/ty/fast_reject.rs:16:5
     |
16   |     IntSimplifiedType,
     |     ----------------- similarly named unit variant `IntSimplifiedType` defined here
     |     ----------------- similarly named unit variant `IntSimplifiedType` defined here

error[E0425]: cannot find function, tuple struct or tuple variant `UintSimplifiedType` in this scope
    --> src/librustdoc/clean/types.rs:1756:31
     |
1756 |                 U16 => single(UintSimplifiedType(UintTy::U16)),
     |                               ^^^^^^^^^^^^^^^^^^ help: a unit variant with a similar name exists: `IntSimplifiedType`
    ::: /checkout/compiler/rustc_middle/src/ty/fast_reject.rs:16:5
     |
16   |     IntSimplifiedType,
     |     ----------------- similarly named unit variant `IntSimplifiedType` defined here
     |     ----------------- similarly named unit variant `IntSimplifiedType` defined here

error[E0425]: cannot find function, tuple struct or tuple variant `UintSimplifiedType` in this scope
    --> src/librustdoc/clean/types.rs:1757:31
     |
1757 |                 U32 => single(UintSimplifiedType(UintTy::U32)),
     |                               ^^^^^^^^^^^^^^^^^^ help: a unit variant with a similar name exists: `IntSimplifiedType`
    ::: /checkout/compiler/rustc_middle/src/ty/fast_reject.rs:16:5
     |
16   |     IntSimplifiedType,
     |     ----------------- similarly named unit variant `IntSimplifiedType` defined here
     |     ----------------- similarly named unit variant `IntSimplifiedType` defined here

error[E0425]: cannot find function, tuple struct or tuple variant `UintSimplifiedType` in this scope
    --> src/librustdoc/clean/types.rs:1758:31
     |
1758 |                 U64 => single(UintSimplifiedType(UintTy::U64)),
     |                               ^^^^^^^^^^^^^^^^^^ help: a unit variant with a similar name exists: `IntSimplifiedType`
    ::: /checkout/compiler/rustc_middle/src/ty/fast_reject.rs:16:5
     |
16   |     IntSimplifiedType,
     |     ----------------- similarly named unit variant `IntSimplifiedType` defined here
     |     ----------------- similarly named unit variant `IntSimplifiedType` defined here

error[E0425]: cannot find function, tuple struct or tuple variant `UintSimplifiedType` in this scope
    --> src/librustdoc/clean/types.rs:1759:32
     |
1759 |                 U128 => single(UintSimplifiedType(UintTy::U128)),
     |                                ^^^^^^^^^^^^^^^^^^ help: a unit variant with a similar name exists: `IntSimplifiedType`
    ::: /checkout/compiler/rustc_middle/src/ty/fast_reject.rs:16:5
     |
16   |     IntSimplifiedType,
     |     ----------------- similarly named unit variant `IntSimplifiedType` defined here
     |     ----------------- similarly named unit variant `IntSimplifiedType` defined here

error[E0618]: expected function, found `SimplifiedType`
    --> src/librustdoc/clean/types.rs:1760:31
     |
1760 |                 F32 => single(FloatSimplifiedType(FloatTy::F32)),
     |                               |
     |                               call expression requires function

error[E0618]: expected function, found `SimplifiedType`
error[E0618]: expected function, found `SimplifiedType`
    --> src/librustdoc/clean/types.rs:1761:31
     |
1761 |                 F64 => single(FloatSimplifiedType(FloatTy::F64)),
     |                               |
     |                               call expression requires function

Some errors have detailed explanations: E0425, E0618.
