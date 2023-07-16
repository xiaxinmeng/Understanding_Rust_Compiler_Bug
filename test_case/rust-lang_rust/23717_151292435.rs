
main.0.rs:(.text.__rust_allocate+0x10): undefined reference to `malloc'
main.0.rs:(.text.__rust_allocate+0x30): undefined reference to `posix_memalign'
dtls/build/main.o: In function `__rust_deallocate':
main.0.rs:(.text.__rust_deallocate+0x1): undefined reference to `free'
dtls/build/main.o: In function `__rust_reallocate':
main.0.rs:(.text.__rust_reallocate+0x20): undefined reference to `realloc'
main.0.rs:(.text.__rust_reallocate+0x42): undefined reference to `posix_memalign'
main.0.rs:(.text.__rust_reallocate+0x6c): undefined reference to `free'
dtls/build/main.o: In function `funcs::posix88::fcntl::open::h9ce22954c6c581ea6ab':
main.0.rs:(.text._ZN5funcs7posix885fcntl4open20h9ce22954c6c581ea6abE+0x1): undefined reference to `open'
