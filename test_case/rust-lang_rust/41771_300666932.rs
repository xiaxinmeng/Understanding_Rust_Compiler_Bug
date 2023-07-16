
[00:01:38] error[E0277]: the trait bound `T: core::clone::Clone` is not satisfied
[00:01:38]     --> /checkout/src/libcollections/vec.rs:1324:9
[00:01:38]      |
[00:01:38] 1324 | impl<T> ExtendWith<T> for ExtendElement<T> {
[00:01:38]      |         ^^^^^^^^^^^^^ the trait `core::clone::Clone` is not implemented for `T`
[00:01:38]      |
[00:01:38]      = help: consider adding a `where T: core::clone::Clone` bound
[00:01:38]      = note: required by `vec::ExtendElement`
[00:01:38] 
[00:01:38] error[E0277]: the trait bound `T: core::clone::Clone` is not satisfied
[00:01:38]     --> /checkout/src/libcollections/vec.rs:1325:5
[00:01:38]      |
[00:01:38] 1325 |     fn next(&self) -> T { self.0.clone() }
[00:01:38]      |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `core::clone::Clone` is not implemented for `T`
[00:01:38]      |
[00:01:38]      = help: consider adding a `where T: core::clone::Clone` bound
[00:01:38]      = note: required by `vec::ExtendElement`
[00:01:38] 
[00:01:38] error[E0277]: the trait bound `T: core::clone::Clone` is not satisfied
[00:01:38]     --> /checkout/src/libcollections/vec.rs:1326:5
[00:01:38]      |
[00:01:38] 1326 |     fn last(self) -> T { self.0 }
[00:01:38]      |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `core::clone::Clone` is not implemented for `T`
[00:01:38]      |
[00:01:38]      = help: consider adding a `where T: core::clone::Clone` bound
[00:01:38]      = note: required by `vec::ExtendElement`
[00:01:38] 
[00:01:38] error: aborting due to 3 previous errors
