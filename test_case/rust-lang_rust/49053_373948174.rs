
[00:39:50]    Compiling error_index_generator v0.0.0 (file:///checkout/src/tools/error_index_generator)
[00:39:52]     Finished release [optimized] target(s) in 2.80 secs
[00:39:53] duplicate artfacts found when compiling a tool, this typically means that something was recompiled because a transitive dependency has different features activated than in a previous build:
[00:39:53] 
[00:39:53]   bitflags 0.9.1 (registry+https://github.com/rust-lang/crates.io-index)
[00:39:53]     `error_index_generator` enabled features ["default", "example_generated"] at "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libbitflags-3f1a1fadb9b1a8ec.rlib"
[00:39:53]     `rustbook` enabled features ["default", "example_generated"] at "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps/libbitflags-8b5679a6ec267b49.rlib"
[00:39:53]   winapi 0.2.8 (registry+https://github.com/rust-lang/crates.io-index)
[00:39:53]     `error_index_generator` enabled features [] at "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libwinapi-6efc14177cb3de4a.rlib"
[00:39:53]     `rustbook` enabled features [] at "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps/libwinapi-2332d26f2d360689.rlib"
[00:39:53]   libc 0.2.39 (registry+https://github.com/rust-lang/crates.io-index)
[00:39:53]     `error_index_generator` enabled features ["default", "use_std"] at "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/liblibc-fff2bd13c38c95a8.rlib"
[00:39:53]     `rustbook` enabled features ["default", "use_std"] at "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps/liblibc-170c926795128861.rlib"
[00:39:53]   rand 0.4.2 (registry+https://github.com/rust-lang/crates.io-index)
[00:39:53]     `error_index_generator` enabled features ["default", "libc", "std"] at "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librand-4291d425b15909cc.rlib"
[00:39:53]     `rustbook` enabled features ["default", "libc", "std"] at "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps/librand-705da6ff1eaaba3c.rlib"
[00:39:53]   pulldown-cmark 0.1.2 (registry+https://github.com/rust-lang/crates.io-index)
[00:39:53]     `error_index_generator` enabled features [] at "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libpulldown_cmark-b06a147dfe86b49e.rlib"
[00:39:53]     `rustbook` enabled features ["default", "getopts"] at "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps/libpulldown_cmark-7bdb88b3c356cbc9.rlib"
[00:39:53]   kernel32-sys 0.2.2 (registry+https://github.com/rust-lang/crates.io-index)
[00:39:53]     `error_index_generator` enabled features [] at "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libkernel32-098f6019eeaaaf39.rlib"
[00:39:53]     `rustbook` enabled features [] at "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps/libkernel32-4a53592702dd8140.rlib"
[00:39:53]   remove_dir_all 0.3.0 (registry+https://github.com/rust-lang/crates.io-index)
[00:39:53]     `error_index_generator` enabled features [] at "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libremove_dir_all-4f18feda118fd3e9.rlib"
[00:39:53] thread 'main' panicked at 'tools should not compile multiple copies of the same crate', bootstrap/tool.rs:188:13
[00:39:53]     `rustbook` enabled features [] at "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps/libremove_dir_all-7077b3afe6766cb9.rlib"
[00:39:53]   tempdir 0.3.6 (registry+https://github.com/rust-lang/crates.io-index)
[00:39:53]     `error_index_generator` enabled features [] at "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtempdir-89fd2625a654b5c7.rlib"
[00:39:53]     `rustbook` enabled features [] at "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps/libtempdir-68fa061d83ba4e87.rlib"
[00:39:53] 
[00:39:53] note: Run with `RUST_BACKTRACE=1` for a backtrace.
