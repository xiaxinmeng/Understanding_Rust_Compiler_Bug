
error: use of deprecated item 'Foo::deprecated'
  --> src/main.rs:11:9
   |
11 |         deprecated: 1,
   |         ^^^^^^^^^^^^^
   |
note: lint level defined here
  --> src/main.rs:1:9
   |
1  | #![deny(warnings)]
   |         ^^^^^^^^
   = note: `#[deny(deprecated)]` implied by `#[deny(warnings)]`

error: use of deprecated item 'Foo::deprecated'
  --> src/main.rs:14:20
   |
14 |     println!("{}", foo.deprecated)
   |                    ^^^^^^^^^^^^^^

error: aborting due to 2 previous errors
