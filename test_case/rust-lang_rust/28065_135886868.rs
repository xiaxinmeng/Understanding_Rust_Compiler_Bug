 Rust
// FIXME: switch this to use atexit. Currently this
// segfaults (the queue's memory is mysteriously gone), so
// instead the cleanup is tied to the `std::rt` entry point.
