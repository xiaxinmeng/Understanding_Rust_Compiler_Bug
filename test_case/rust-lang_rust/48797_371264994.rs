
LD_LIBRARY_PATH=path/rust_linux_x86_64/rustc/lib DYLD_LIBRARY_PATH=path/rust_linux_x86_64/rustc/lib HOST=x86_64-unknown-linux-gnu TARGET=x86_64-unknown-linux-gnu path/rust_linux_x86_64/rustc/bin/rustc src/lib.rs --crate-name crate_name --crate-type lib -L all=path/rust_linux_x86_64/rust-std-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib -C panic=abort

