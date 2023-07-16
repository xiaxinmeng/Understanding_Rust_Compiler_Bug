plain
    |
457 |       #[allow_internal_unstable(slice_ptr_len)]
    |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
...
461 | /     pub const fn len(self) -> usize {
462 | |         self.as_ptr().len()
    | |_____- not a macro

error: could not compile `core` due to previous error
warning: build failed, waiting for other jobs to finish...
