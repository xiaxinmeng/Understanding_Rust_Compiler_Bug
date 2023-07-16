text

===================================================================================================

Failure due to:
   stderr:
   ------------------------------------------
   error: this operation will panic at runtime
     --> /usr/local/google/home/richkadel/rust/src/test/ui/issues/issue-33287.rs:7:17
      |
   LL |     let range = A[1]..;
      |                 ^^^^ index out of bounds: the length is 1 but the index is 1
      |
      = note: `#[deny(unconditional_panic)]` on by default

src/test/ui/issues/issue-33287.rs

