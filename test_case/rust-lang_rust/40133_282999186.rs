
/checkout/src/test/codegen/lifetime_start_end.rs:39:11: error: expected string not found in input
// CHECK: [[E_b:%[0-9]+]] = bitcast %"core::option::Option<i32>"** %b to i8*
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/lifetime_start_end.ll:37:2: note: scanning from here
 %9 = bitcast i32* %c to i8*
 ^
