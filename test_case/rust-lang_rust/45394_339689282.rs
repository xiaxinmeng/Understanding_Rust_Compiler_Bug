
[01:07:10] error: expected expression, found `}`
[01:07:10]   --> /checkout/src/test/run-pass/rfc-2008-non-exhaustive/enums.rs:17:67
[01:07:10]    |
[01:07:10] 17 |     let variant_tuple = NonExhaustiveVariants::Tuple { 0: 340, .. };
[01:07:10]    |                                                                   ^
[01:07:10] 
[01:07:10] error: expected expression, found `}`
[01:07:10]   --> /checkout/src/test/run-pass/rfc-2008-non-exhaustive/enums.rs:18:73
[01:07:10]    |
[01:07:10] 18 |     let variant_struct = NonExhaustiveVariants::Struct { field: 340, .. };
[01:07:10]    |                                                                         ^

[01:07:10]   --> /checkout/src/test/run-pass/rfc-2008-non-exhaustive/structs.rs:17:69
[01:07:10]    |
[01:07:10] 17 |     let ns = NormalStruct { first_field: 640, second_field: 480, .. };
[01:07:10]    |                                                                     ^
[01:07:10] 
[01:07:10] error: expected expression, found `}`
[01:07:10]   --> /checkout/src/test/run-pass/rfc-2008-non-exhaustive/structs.rs:21:47
[01:07:10]    |
[01:07:10] 21 |     let ts = TupleStruct { 0: 340, 1: 480, .. };
[01:07:10]    |                                               ^
[01:07:10] 
[01:07:10] error: expected expression, found `}`
[01:07:10]   --> /checkout/src/test/run-pass/rfc-2008-non-exhaustive/structs.rs:25:30
[01:07:10]    |
[01:07:10] 25 |     let us = UnitStruct { .. };
