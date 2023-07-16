
error[E0658]: generic associated types are unstable
 --> src/lib.rs:2:5
  |
2 |     type Member<T>;
  |     ^^^^^^^^^^^^^^^
  |
  = note: see issue #44265 <https://github.com/rust-lang/rust/issues/44265> for more information

error[E0107]: missing generics for associated type `CollectionFamily::Member`
 --> src/lib.rs:2:10
  |
2 |     type Member<T>;
  |          ^^^^^^ expected 1 type argument
  |
note: associated type defined here, with 1 type parameter: `T`
 --> src/lib.rs:2:10
  |
2 |     type Member<T>;
  |          ^^^^^^ -
help: use angle brackets to add missing type argument
  |
2 |     type Member<T><T>;
  |                ^^^
