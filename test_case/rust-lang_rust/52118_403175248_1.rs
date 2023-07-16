
error[E0599]: no variant named `Init` found for type `State` in the current scope
  --> src/main.rs:11:13
   |
11 |             Self::Init => 1,
   |             ^^^^^^^^^^ can't access variant `State::Init` through `Self`
   |
   = note: until <https://github.com/rust-lang/rust/issues/49683> is implemented, you can't refer to enum variants using the local alias `Self`
help: instead of using `Self`, use the canonical path for the variant instead:
   |
11 |             State::Init => 1,
   |             ^^^^^^^^^^^

