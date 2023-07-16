plain
   Compiling miniz_oxide v0.4.0
   Compiling hashbrown v0.11.0
   Compiling object v0.22.0
   Compiling addr2line v0.14.0
error[E0599]: no function or associated item named `with_capacity` found for struct `SwitchWriter<_>` in the current scope
    |
    |
563 |                 *lock.borrow_mut() = SwitchWriter::with_capacity(0, stdout_raw());
    |                                                    ^^^^^^^^^^^^^ function or associated item not found in `SwitchWriter<_>`
    | 
   ::: library/std/src/io/buffered/switchwriter.rs:27:1
    |
27  | pub struct SwitchWriter<W: Write> {
    | --------------------------------- function or associated item `with_capacity` not found for this
error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
error: could not compile `std`
