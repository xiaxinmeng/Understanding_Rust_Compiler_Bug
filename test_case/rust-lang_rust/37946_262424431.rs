
error: unused type parameter
   --> /checkout/src/libserialize/serialize.rs:677:18
    |
677 |     fn not_found<S, T: ?Sized>(trait_name: &'static str,
    |                  ^
    |
note: lint level defined here
   --> /checkout/src/libserialize/lib.rs:28:31
    |
28  | #![cfg_attr(not(stage0), deny(warnings))]
    |                               ^^^^^^^^

error: unused type parameter
   --> /checkout/src/libserialize/serialize.rs:677:21
    |
677 |     fn not_found<S, T: ?Sized>(trait_name: &'static str,
    |                     ^

error: aborting due to 2 previous errors
