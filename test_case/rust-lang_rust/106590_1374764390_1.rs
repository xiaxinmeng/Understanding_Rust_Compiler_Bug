
error[[E0308]](https://doc.rust-lang.org/nightly/error-index.html#E0308): mismatched types
 --> src/main.rs:5:14
  |
5 |     x.insert(1i32, Default::default());
  |       ------ ^^^^  ------------------ this is of type `_`, which causes `x` to be inferred as `HashMap<u32, _>`
  |       |      |
  |       |      expected `u32`, found `i32`
  |       |      this is of type `i32`, which causes `x` to be inferred as `HashMap<u32, _>`
  |       arguments to this method are incorrect
  |
note: associated function defined here
 --> /rustc/ee0412d1ef81efcfabe7f66cd21476ca85d618b1/library/std/src/collections/hash/map.rs:1103:12
help: change the type of the numeric literal from `i32` to `u32`
  |
5 |     x.insert(1u32, Default::default());
  |               ~~~
