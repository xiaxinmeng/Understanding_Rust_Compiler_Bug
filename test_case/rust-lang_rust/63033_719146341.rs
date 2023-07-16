
error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
  --> src/profile.rs:91:10
   |
91 |     ) -> Result<(), BoxError> {
   |          ^^^^^^^^^^^^^^^^^^^^
   |
note: hidden type `impl futures::Future` captures lifetime smaller than the function body
  --> src/profile.rs:91:10
   |
91 |     ) -> Result<(), BoxError> {
   |       
