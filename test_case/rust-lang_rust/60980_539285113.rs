
error[E0277]: the trait bound `A: IntoWasmAbi` is not satisfied
 --> file.rs:3:1
  |
3 | / trait IntoWasmAbi {
4 | |     type Abi;
5 | |     fn into_abi(self) -> Self::Abi;
6 | | }
  | |_^ the trait `IntoWasmAbi` is not implemented for `A`
