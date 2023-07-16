
Future incompatibility report: Future breakage diagnostic:
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/issue-50814.rs:15:21
   |
LL |     const MAX: u8 = A::MAX + B::MAX;
   |     ----------------^^^^^^^^^^^^^^^-
   |                     |
   |                     attempt to compute `u8::MAX + u8::MAX`, which would overflow
   |
   = note: `#[deny(const_err)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

Future breakage diagnostic:
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/issue-50814.rs:15:21
   |
LL |     const MAX: u8 = A::MAX + B::MAX;
   |     ----------------^^^^^^^^^^^^^^^-
   |                     |
   |                     attempt to compute `u8::MAX + u8::MAX`, which would overflow
   |
   = note: `#[deny(const_err)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
