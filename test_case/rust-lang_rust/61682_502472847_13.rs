rust
error: ambiguous associated item
  --> src/lib.rs:10:14
   |
10 |     const C: Self::V = 0;
   |              ^^^^^^^ help: use fully-qualified syntax: `<E as Trait>::V`
   |
   = note: #[deny(ambiguous_associated_items)] on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #57644 <https://github.com/rust-lang/rust/issues/57644>
note: `V` could refer to variant defined here
  --> src/lib.rs:1:10
   |
1  | enum E { V }
   |          ^
note: `V` could also refer to associated type defined here
  --> src/lib.rs:4:5
   |
4  |     type V;
   |     ^^^^^^^
