
error[E0599]: the method `get_pin_mut` exists for struct `Pin<&mut Exclusive<T>>`, but its trait bounds were not satisfied
   --> library/core/src/sync/exclusive.rs:163:14
    |
163 |         self.get_pin_mut().poll(cx)
    |              ^^^^^^^^^^^
    |
   ::: library/core/src/pin.rs:408:1
    |
408 | pub struct Pin<P> {
    | ----------------- method `get_pin_mut` not found for this
    |
    = note: the following trait bounds were not satisfied:
            `T: Sized`
