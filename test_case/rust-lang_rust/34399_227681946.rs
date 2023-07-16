
src/libstd/os/linux/raw.rs:27:1: 27:30 error: An API can't be stabilized after it is deprecated

src/libstd/os/linux/raw.rs:27 pub type pthread_t = c_ulong;

                              ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~

src/libstd/sys/unix/ext/raw.rs:27:1: 27:39 error: An API can't be stabilized after it is deprecated

src/libstd/sys/unix/ext/raw.rs:27 pub use sys::platform::raw::pthread_t;

                                  ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

error: aborting due to 2 previous errors
