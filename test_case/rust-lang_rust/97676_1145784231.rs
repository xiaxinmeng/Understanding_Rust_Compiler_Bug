plain
    Checking rustc-demangle v0.1.21
error[E0277]: the size for values of type `T` cannot be known at compilation time
    --> library/alloc/src/rc.rs:1538:34
     |
1504 | unsafe impl<#[may_dangle] T: ?Sized> Drop for Rc<T> {
     |                           - this type parameter needs to be `core::marker::Sized`
1538 |             if mem::needs_drop::<T>() {
     |                                  ^ doesn't have a size known at compile-time
     |
note: required by a bound in `core::mem::needs_drop`
note: required by a bound in `core::mem::needs_drop`
    --> /checkout/library/core/src/mem/mod.rs:595:25
     |
595  | pub const fn needs_drop<T>() -> bool {
     |                         ^ required by this bound in `core::mem::needs_drop`
help: consider removing the `?Sized` bound to make the type parameter `Sized`
     |
1504 - unsafe impl<#[may_dangle] T: ?Sized> Drop for Rc<T> {
1504 + unsafe impl<#[may_dangle] T> Drop for Rc<T> {

error[E0277]: the size for values of type `T` cannot be known at compilation time
    --> library/alloc/src/sync.rs:1702:34
     |
     |
1636 | unsafe impl<#[may_dangle] T: ?Sized> Drop for Arc<T> {
     |                           - this type parameter needs to be `core::marker::Sized`
1702 |             if mem::needs_drop::<T>() {
     |                                  ^ doesn't have a size known at compile-time
     |
note: required by a bound in `core::mem::needs_drop`
note: required by a bound in `core::mem::needs_drop`
    --> /checkout/library/core/src/mem/mod.rs:595:25
     |
595  | pub const fn needs_drop<T>() -> bool {
     |                         ^ required by this bound in `core::mem::needs_drop`
help: consider removing the `?Sized` bound to make the type parameter `Sized`
     |
1636 - unsafe impl<#[may_dangle] T: ?Sized> Drop for Arc<T> {
1636 + unsafe impl<#[may_dangle] T> Drop for Arc<T> {

For more information about this error, try `rustc --explain E0277`.
error: could not compile `alloc` due to 2 previous errors
Build completed unsuccessfully in 0:01:39
