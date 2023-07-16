plain
    = note: the trait bound `dyn error::Error: Provider` is not satisfied
note: required because of the requirements on the impl of `Provider` for `dyn error::Error`
   --> library/core/src/error.rs:209:9
    |
209 | impl<E> Provider for E
note: required by a bound in `request_ref`
   --> library/core/src/any.rs:847:47
    |
    |
847 | pub fn request_ref<'a, T>(provider: &'a (impl Provider + ?Sized)) -> Option<&'a T>
    |                                               ^^^^^^^^ required by this bound in `request_ref`
    |
233 |         core::any::request_ref(&self)
    |                                +

---
    = note: the trait bound `dyn error::Error: Provider` is not satisfied
note: required because of the requirements on the impl of `Provider` for `dyn error::Error`
   --> library/core/src/error.rs:209:9
    |
209 | impl<E> Provider for E
note: required by a bound in `request_value`
   --> library/core/src/any.rs:825:49
    |
    |
825 | pub fn request_value<'a, T>(provider: &'a (impl Provider + ?Sized)) -> Option<T>
    |                                                 ^^^^^^^^ required by this bound in `request_value`
    |
239 |         core::any::request_value(&self)
    |                                  +

