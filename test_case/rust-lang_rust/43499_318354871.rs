
error[E0308]: mismatched types
   --> src/lib.rs:587:40
    |
587 |                     libc::pthread_kill(self.thread.as_pthread_t(), libc::SIGUSR1);
    |                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^ expected u32, found usize
    |
    = help: here are some functions which might fulfill your needs:
            - .count_ones()
            - .count_zeros()
            - .leading_zeros()
            - .trailing_zeros()

error: aborting due to previous error

error: Could not compile `jobserver`.
