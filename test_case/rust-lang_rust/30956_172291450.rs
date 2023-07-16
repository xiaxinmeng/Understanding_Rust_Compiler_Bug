
test num::tests::test_i32f64 ... ok

test num::tests::test_i32i64 ... ok

<test>:1:62: 1:63 error: macro undefined: 'z!'

<test>:1 fn bogus() {macro_rules! z (() => (3+4));}fn inty() -> i32 { z!() }

                                                                      ^

<test>:1:62: 1:63 help: did you mean `z!`?

error: aborting due to previous error

test ext::expand::tests::macros_cant_escape_fns_test ... ok

test ext::expand::tests::method_arg_hygiene ... ok

test num::tests::test_i8f64 ... ok

test num::tests::test_i8f32 ... ok

test ext::expand::tests::method_arg_hygiene_2 ... ok

<test>:1:59: 1:60 error: macro undefined: 'z!'

<test>:1 mod foo {macro_rules! z (() => (3+4));}fn inty() -> i32 { z!() }

                                                                   ^

<test>:1:59: 1:60 help: did you mean `z!`?

error: aborting due to previous error

test ext::expand::tests::macros_cant_escape_mods_test ... ok
