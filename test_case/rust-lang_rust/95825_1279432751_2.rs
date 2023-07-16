shell
   Compiling autocfg v1.1.0
   Compiling proc-macro2 v1.0.37
   Compiling unicode-xid v0.2.2
   ...
     Compiling bin v0.1.0 (/home/dup2rt/Temp/rust-bug-95825/bin)
    Finished test [unoptimized + debuginfo] target(s) in 21.96s
     Running unittests src/main.rs (target/debug/deps/bin-d8e0732fa286bd7e)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/integration.rs (target/debug/deps/integration-e758c50b28aecb37)

running 1 test
test bin_errors_out ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 20.01s

     Running unittests src/lib.rs (target/debug/deps/lib-5e366658d769523f)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests lib
error[E0463]: can't find crate for `serde_derive` which `serde` depends on
 --> /home/dup2rt/Temp/rust-bug-95825/lib/src/cbor_types.rs:1:5
  |
1 | use serde::de::{Deserialize, Deserializer};
  |     ^^^^^ can't find crate

error[E0463]: can't find crate for `serde_derive` which `serde` depends on
 --> /home/dup2rt/Temp/rust-bug-95825/lib/src/cbor_types.rs:2:5
  |
2 | use serde::ser::{Serialize, Serializer};
  |     ^^^^^ can't find crate

error[E0463]: can't find crate for `serde_derive` which `serde_bytes` depends on
 --> /home/dup2rt/Temp/rust-bug-95825/lib/src/cbor_types.rs:3:5
  |
3 | use serde_bytes::ByteBuf;
  |     ^^^^^^^^^^^ can't find crate

error[E0463]: can't find crate for `serde_derive` which `serde_cbor` depends on
 --> /home/dup2rt/Temp/rust-bug-95825/lib/src/cbor_types.rs:4:5
  |
4 | use serde_cbor::tags::Tagged;
  |     ^^^^^^^^^^ can't find crate

error[E0463]: can't find crate for `serde_derive` which `serde` depends on
  --> /home/dup2rt/Temp/rust-bug-95825/lib/src/cbor_types.rs:21:28
   |
21 |             Some(_) => Err(serde::de::Error::custom("unexpected tag")),
   |                            ^^^^^ can't find crate

error[E0463]: can't find crate for `serde_derive` which `serde` depends on
  --> /home/dup2rt/Temp/rust-bug-95825/lib/src/cbor_types.rs:47:28
   |
47 |             Some(_) => Err(serde::de::Error::custom("unexpected tag")),
   |                            ^^^^^ can't find crate

error[E0463]: can't find crate for `serde_derive` which `serde` depends on
  --> /home/dup2rt/Temp/rust-bug-95825/lib/src/cbor_types.rs:72:28
   |
72 |             Some(_) => Err(serde::de::Error::custom("unexpected tag")),
   |                            ^^^^^ can't find crate

error: aborting due to 7 previous errors

For more information about this error, try `rustc --explain E0463`.
error: doctest failed, to rerun pass `-p lib --doc`

Caused by:
  process didn't exit successfully: `rustdoc --edition=2018 --crate-type cdylib --crate-type rlib --crate-name lib --test /home/dup2rt/Temp/rust-bug-95825/lib/src/lib.rs -L dependency=/home/dup2rt/Temp/rust-bug-95825/target/debug/deps -L dependency=/home/dup2rt/Temp/rust-bug-95825/target/debug/deps --extern lib=/home/dup2rt/Temp/rust-bug-95825/target/debug/deps/liblib.rlib --extern log=/home/dup2rt/Temp/rust-bug-95825/target/debug/deps/liblog-e2a90e882ca4d481.rlib --extern serde=/home/dup2rt/Temp/rust-bug-95825/target/debug/deps/libserde-2f2024fc03b2f3d2.rlib --extern serde_bytes=/home/dup2rt/Temp/rust-bug-95825/target/debug/deps/libserde_bytes-048ef4f616271d10.rlib --extern serde_cbor=/home/dup2rt/Temp/rust-bug-95825/target/debug/deps/libserde_cbor-c33d8a2e71989bfb.rlib -C embed-bitcode=no --error-format human` (exit status: 1)
make: *** [Makefile:2: red] Error 1
