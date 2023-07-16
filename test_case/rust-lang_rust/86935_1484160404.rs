
error: ambiguous associated item
  --> src/main.rs:26:32
   |
26 |             Self::Ref { a } => Self::Ref::Ref { a },
   |                                ^^^^^^^^^ help: use fully-qualified syntax: `<Test as EnumRef>::Ref`
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, [see issue #57644 <https://github.com/rust-lang/rust/issues/57644>](https://github.com/rust-lang/rust/issues/57644)
note: `Ref` could refer to the variant defined here
  --> src/main.rs:12:5
   |
12 |     Ref { a: i32 },
   |     ^^^
note: `Ref` could also refer to the associated type defined here
  --> src/main.rs:3:5
   |
3  |     type Ref<'a>
   |     ^^^^^^^^^^^^
   = note: `#[deny(ambiguous_associated_items)]` on by default
