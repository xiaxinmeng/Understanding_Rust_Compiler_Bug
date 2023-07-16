
error[E0277]: the trait bound `A: IntoWasmAbi` is not satisfied
 --> file2.rs:8:10
  |
8 | type T = <A as IntoWasmAbi>::Abi;
  |          ^^^^^^^^^^^^^^^^^^^^^^^ the trait `IntoWasmAbi` is not implemented for `A`

error[E0277]: the trait bound `A: IntoWasmAbi` is not satisfied
  --> file2.rs:10:1
   |
10 | fn foo(a: T) {}
   | ^^^^^^^^^^^^^^^ the trait `IntoWasmAbi` is not implemented for `A`
