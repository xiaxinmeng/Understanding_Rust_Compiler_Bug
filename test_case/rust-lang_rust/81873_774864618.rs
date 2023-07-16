plain
   Compiling object v0.22.0
   Compiling miniz_oxide v0.4.0
   Compiling hashbrown v0.9.0
   Compiling addr2line v0.14.0
error: hidden lifetime parameters in types are deprecated
    |
    |
236 |     pub fn unlock(guard: MutexGuard<T>) {
    |                          ^^^^^^^^^^^^^ help: indicate the anonymous lifetime: `MutexGuard<'_, T>`
    |
    = note: `-D elided-lifetimes-in-paths` implied by `-D warnings`
error: aborting due to previous error

error: could not compile `std`

