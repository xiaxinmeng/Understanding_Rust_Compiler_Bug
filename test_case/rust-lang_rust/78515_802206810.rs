plain
   Compiling hashbrown v0.11.0
   Compiling miniz_oxide v0.4.0
   Compiling object v0.22.0
   Compiling addr2line v0.14.0
error[E0599]: no method named `set_buffer_mode` found for struct `ReentrantMutexGuard<'_, RefCell<SwitchWriter<StdoutRaw>>>` in the current scope
    |
    |
561 |                 lock.set_buffer_mode(BufferMode::Immediate)
    |                      ^^^^^^^^^^^^^^^ method not found in `ReentrantMutexGuard<'_, RefCell<SwitchWriter<StdoutRaw>>>`
   ::: library/std/src/sys_common/remutex.rs:40:1
    |
    |
40  | pub struct ReentrantMutexGuard<'a, T: 'a> {
    | ----------------------------------------- method `set_buffer_mode` not found for this
error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
error: could not compile `std`
