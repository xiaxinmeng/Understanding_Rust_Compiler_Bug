
error[E0277]: the trait bound `A: IntoWasmAbi` is not satisfied
 --> file.rs:9:1
  |
1 |   struct A;
  |   -------- this struct doesn't implement `IntoWasmAbi`
2 | 
3 | / trait IntoWasmAbi {
4 | |     type Abi;
5 | |     fn into_abi(self) -> Self::Abi;
6 | | }
  | |_^ this trait is not implemented for `A`
7 | 
8 |   type T = <A as IntoWasmAbi>::Abi;
  |   -------------------------------- `IntoWasmAbi` expected to be implemented for `A` because of this
  | 
9 |   fn foo(a: T) {}
  |             - `IntoWasmAbi` is not implemented for `A`
