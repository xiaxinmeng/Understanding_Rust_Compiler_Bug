plain
doc tests for: /checkout/src/doc/rustc/src/platform-support/armv7-unknown-linux-uclibceabihf.md
doc tests for: /checkout/src/doc/rustc/src/platform-support/fuchsia.md


command did not execute successfully: "/checkout/obj/build/bootstrap/debug/rustdoc" "-Wrustdoc::invalid_codeblock_attributes" "-Dwarnings" "-Znormalize-docs" "-Z" "unstable-options" "--test" "/checkout/src/doc/rustc/src/platform-support/fuchsia.md" "--test-args" ""

stdout ----

running 5 tests
running 5 tests
test /checkout/src/doc/rustc/src/platform-support/fuchsia.md - _and_::Testing (line 266) ... FAILED
test /checkout/src/doc/rustc/src/platform-support/fuchsia.md - _and_::Building_Rust_programs::Create_a_package (line 133) ... FAILED
test /checkout/src/doc/rustc/src/platform-support/fuchsia.md - _and_::Building_Rust_programs::Bulding_a_component (line 175) ... FAILED
test /checkout/src/doc/rustc/src/platform-support/fuchsia.md - _and_::Building_Rust_programs::Create_a_package (line 109) ... FAILED
test /checkout/src/doc/rustc/src/platform-support/fuchsia.md - _and_::Building_Rust_programs (line 89) ... ok
failures:


---- /checkout/src/doc/rustc/src/platform-support/fuchsia.md - _and_::Testing (line 266) stdout ----
error: struct literal body without path
 --> /checkout/src/doc/rustc/src/platform-support/fuchsia.md:269:14
5 |       program: {
  |  ______________^
6 | |         runner: "elf",
6 | |         runner: "elf",
7 | |         binary: "bin/hello_fuchsia",
8 | |         args: ["it_works"],
  | |_____^
  |
  |
help: you might have forgotten to add the struct literal inside the block
  |
5 ~     program: { SomeStruct {
6 |         runner: "elf",
7 |         binary: "bin/hello_fuchsia",
8 |         args: ["it_works"],
  |


error: expected type, found `"syslog/client.shard.cml"`
 --> /checkout/src/doc/rustc/src/platform-support/fuchsia.md:268:16
  |
4 |     include: [ "syslog/client.shard.cml" ],
  |            -   ^^^^^^^^^^^^^^^^^^^^^^^^^ expected type
  |            |
  |            tried to parse a type due to this type ascription
  |
  = note: `#![feature(type_ascription)]` lets you annotate an expression with a type: `<expr>: <type>`

error: struct literal body without path
error: struct literal body without path
  --> /checkout/src/doc/rustc/src/platform-support/fuchsia.md:267:1
3  | / {
3  | / {
4  | |     include: [ "syslog/client.shard.cml" ],
5  | |     program: {
6  | |         runner: "elf",
9  | |     },
10 | | }
   | |_^
   |
   |
help: you might have forgotten to add the struct literal inside the block
   |
3  ~ { SomeStruct {
4  |     include: [ "syslog/client.shard.cml" ],
9  |     },
10 ~ } }
   |


error: aborting due to 3 previous errors

Couldn't compile the test.
---- /checkout/src/doc/rustc/src/platform-support/fuchsia.md - _and_::Building_Rust_programs::Create_a_package (line 133) stdout ----
error: expected expression, found `$`
 --> /checkout/src/doc/rustc/src/platform-support/fuchsia.md:134:19
  |
3 | bin/hello_fuchsia=${WORK_DIR}/bin/hello_fuchsia

error: aborting due to previous error

Couldn't compile the test.
Couldn't compile the test.
---- /checkout/src/doc/rustc/src/platform-support/fuchsia.md - _and_::Building_Rust_programs::Bulding_a_component (line 175) stdout ----
error: struct literal body without path
 --> /checkout/src/doc/rustc/src/platform-support/fuchsia.md:178:14
5 |       program: {
  |  ______________^
6 | |         runner: "elf",
6 | |         runner: "elf",
7 | |         binary: "bin/hello_fuchsia",
  | |_____^
  |
  |
help: you might have forgotten to add the struct literal inside the block
  |
5 ~     program: { SomeStruct {
6 |         runner: "elf",
7 |         binary: "bin/hello_fuchsia",
  |


error: expected type, found `"syslog/client.shard.cml"`
 --> /checkout/src/doc/rustc/src/platform-support/fuchsia.md:177:16
  |
4 |     include: [ "syslog/client.shard.cml" ],
  |            -   ^^^^^^^^^^^^^^^^^^^^^^^^^ expected type
  |            |
  |            tried to parse a type due to this type ascription
  |
  = note: `#![feature(type_ascription)]` lets you annotate an expression with a type: `<expr>: <type>`

error: struct literal body without path
error: struct literal body without path
 --> /checkout/src/doc/rustc/src/platform-support/fuchsia.md:176:1
3 | / {
3 | / {
4 | |     include: [ "syslog/client.shard.cml" ],
5 | |     program: {
6 | |         runner: "elf",
7 | |         binary: "bin/hello_fuchsia",
9 | | }
  | |_^
  |
  |
help: you might have forgotten to add the struct literal inside the block
  |
3 ~ { SomeStruct {
4 |     include: [ "syslog/client.shard.cml" ],
8 |     },
9 ~ } }
  |


error: aborting due to 3 previous errors

Couldn't compile the test.
---- /checkout/src/doc/rustc/src/platform-support/fuchsia.md - _and_::Building_Rust_programs::Create_a_package (line 109) stdout ----
error: unknown start of token: \u{2523}
 --> /checkout/src/doc/rustc/src/platform-support/fuchsia.md:113:1
6 | ┣━ bin
  | ^

error: unknown start of token: \u{2501}
error: unknown start of token: \u{2501}
 --> /checkout/src/doc/rustc/src/platform-support/fuchsia.md:113:2
6 | ┣━ bin
  |  ^
  |
  |
help: Unicode character '━' (Box Drawings Heavy Horizontal) looks like '-' (Minus/Hyphen), but it is not
6 | ┣- bin
  |  ~

error: unknown start of token: \u{2503}
error: unknown start of token: \u{2503}
 --> /checkout/src/doc/rustc/src/platform-support/fuchsia.md:114:1
7 | ┃  ┗━ hello_fuchsia
  | ^

error: unknown start of token: \u{2517}
error: unknown start of token: \u{2517}
 --> /checkout/src/doc/rustc/src/platform-support/fuchsia.md:114:4
7 | ┃  ┗━ hello_fuchsia
  |    ^

error: unknown start of token: \u{2501}
error: unknown start of token: \u{2501}
 --> /checkout/src/doc/rustc/src/platform-support/fuchsia.md:114:5
7 | ┃  ┗━ hello_fuchsia
  |     ^
  |
  |
help: Unicode character '━' (Box Drawings Heavy Horizontal) looks like '-' (Minus/Hyphen), but it is not
7 | ┃  ┗- hello_fuchsia
  |     ~

error: unknown start of token: \u{2523}
error: unknown start of token: \u{2523}
 --> /checkout/src/doc/rustc/src/platform-support/fuchsia.md:115:1
8 | ┣━ meta
  | ^

error: unknown start of token: \u{2501}
error: unknown start of token: \u{2501}
 --> /checkout/src/doc/rustc/src/platform-support/fuchsia.md:115:2
8 | ┣━ meta
  |  ^
  |
  |
help: Unicode character '━' (Box Drawings Heavy Horizontal) looks like '-' (Minus/Hyphen), but it is not
8 | ┣- meta
  |  ~

error: unknown start of token: \u{2503}
error: unknown start of token: \u{2503}
 --> /checkout/src/doc/rustc/src/platform-support/fuchsia.md:116:1
9 | ┃  ┣━ package
  | ^

error: unknown start of token: \u{2523}
error: unknown start of token: \u{2523}
 --> /checkout/src/doc/rustc/src/platform-support/fuchsia.md:116:4
9 | ┃  ┣━ package
  |    ^

error: unknown start of token: \u{2501}
error: unknown start of token: \u{2501}
 --> /checkout/src/doc/rustc/src/platform-support/fuchsia.md:116:5
9 | ┃  ┣━ package
  |     ^
  |
  |
help: Unicode character '━' (Box Drawings Heavy Horizontal) looks like '-' (Minus/Hyphen), but it is not
9 | ┃  ┣- package
  |     ~

error: unknown start of token: \u{2503}
error: unknown start of token: \u{2503}
  --> /checkout/src/doc/rustc/src/platform-support/fuchsia.md:117:1
10 | ┃  ┗━ hello_fuchsia.cm
   | ^

error: unknown start of token: \u{2517}
error: unknown start of token: \u{2517}
  --> /checkout/src/doc/rustc/src/platform-support/fuchsia.md:117:4
10 | ┃  ┗━ hello_fuchsia.cm
   |    ^

error: unknown start of token: \u{2501}
error: unknown start of token: \u{2501}
  --> /checkout/src/doc/rustc/src/platform-support/fuchsia.md:117:5
10 | ┃  ┗━ hello_fuchsia.cm
   |     ^
   |
   |
help: Unicode character '━' (Box Drawings Heavy Horizontal) looks like '-' (Minus/Hyphen), but it is not
10 | ┃  ┗- hello_fuchsia.cm
   |     ~

error: unknown start of token: \u{2517}
error: unknown start of token: \u{2517}
  --> /checkout/src/doc/rustc/src/platform-support/fuchsia.md:118:1
   |
11 | ┗━ hello_fuchsia.manifest

error: unknown start of token: \u{2501}
error: unknown start of token: \u{2501}
  --> /checkout/src/doc/rustc/src/platform-support/fuchsia.md:118:2
   |
11 | ┗━ hello_fuchsia.manifest
   |
   |
help: Unicode character '━' (Box Drawings Heavy Horizontal) looks like '-' (Minus/Hyphen), but it is not
   |
11 | ┗- hello_fuchsia.manifest
   |  ~

error: expected `;`, found `hello_fuchsia`
 --> /checkout/src/doc/rustc/src/platform-support/fuchsia.md:110:17
  |
3 | hello_fuchsia.rs
  |                 ^ help: add `;` here
4 | hello_fuchsia.cml
  | ------------- unexpected token

error: expected `;`, found `package`
 --> /checkout/src/doc/rustc/src/platform-support/fuchsia.md:111:18
  |
4 | hello_fuchsia.cml
  |                  ^ help: add `;` here
5 | package
  | ------- unexpected token

error[E0425]: cannot find value `hello_fuchsia` in this scope
 --> /checkout/src/doc/rustc/src/platform-support/fuchsia.md:110:1
  |
3 | hello_fuchsia.rs


error[E0425]: cannot find value `hello_fuchsia` in this scope
 --> /checkout/src/doc/rustc/src/platform-support/fuchsia.md:111:1
  |
4 | hello_fuchsia.cml

error[E0425]: cannot find value `package` in this scope
error[E0425]: cannot find value `package` in this scope
 --> /checkout/src/doc/rustc/src/platform-support/fuchsia.md:112:1
5 | package
  | ^^^^^^^ not found in this scope

error[E0425]: cannot find value `bin` in this scope
error[E0425]: cannot find value `bin` in this scope
 --> /checkout/src/doc/rustc/src/platform-support/fuchsia.md:113:4
6 | ┣━ bin
  |    ^^^ not found in this scope


error[E0425]: cannot find value `hello_fuchsia` in this scope
 --> /checkout/src/doc/rustc/src/platform-support/fuchsia.md:114:7
7 | ┃  ┗━ hello_fuchsia
  |       ^^^^^^^^^^^^^ not found in this scope

error[E0425]: cannot find value `meta` in this scope
error[E0425]: cannot find value `meta` in this scope
 --> /checkout/src/doc/rustc/src/platform-support/fuchsia.md:115:4
8 | ┣━ meta
  |    ^^^^ not found in this scope

error[E0425]: cannot find value `package` in this scope
error[E0425]: cannot find value `package` in this scope
 --> /checkout/src/doc/rustc/src/platform-support/fuchsia.md:116:7
9 | ┃  ┣━ package
  |       ^^^^^^^ not found in this scope


error[E0425]: cannot find value `hello_fuchsia` in this scope
  --> /checkout/src/doc/rustc/src/platform-support/fuchsia.md:117:7
10 | ┃  ┗━ hello_fuchsia.cm
   |       ^^^^^^^^^^^^^ not found in this scope


error[E0425]: cannot find value `hello_fuchsia` in this scope
  --> /checkout/src/doc/rustc/src/platform-support/fuchsia.md:118:4
   |
11 | ┗━ hello_fuchsia.manifest

error: aborting due to 26 previous errors

For more information about this error, try `rustc --explain E0425`.
For more information about this error, try `rustc --explain E0425`.
Couldn't compile the test.

failures:
    /checkout/src/doc/rustc/src/platform-support/fuchsia.md - _and_::Building_Rust_programs::Bulding_a_component (line 175)
    /checkout/src/doc/rustc/src/platform-support/fuchsia.md - _and_::Building_Rust_programs::Create_a_package (line 109)
    /checkout/src/doc/rustc/src/platform-support/fuchsia.md - _and_::Building_Rust_programs::Create_a_package (line 133)
    /checkout/src/doc/rustc/src/platform-support/fuchsia.md - _and_::Testing (line 266)
test result: FAILED. 1 passed; 4 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.17s


stderr ----
