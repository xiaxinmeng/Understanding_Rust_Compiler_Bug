
error[E0609]: no field `red` on type `*mut Foo`
 --> src/lib.rs:4:7
  |
4 |     x.red = 4;
  |     --^^^
  |     |
  |     help: `x` is a raw pointer; try dereferencing it: `(*x).red`
  