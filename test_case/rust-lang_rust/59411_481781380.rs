
$ llvm-dwarfdump `rustc +nightly-2019-03-15 --print sysroot`/lib/rustlib/x86_64-unknown-linux-musl/lib/libbacktrace_sys-* > /dev/null
error: /home/alex/.rustup/toolchains/nightly-2019-03-15-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-musl/lib/libbacktrace_sys-c3c54ad8d35f7169.rlib(rust.metadata.bin): The end of the file was unexpectedly encountered

$ llvm-dwarfdump /home/alex/.rustup/toolchains/nightly-2019-03-16-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-musl/lib/libbacktrace_sys-76fed535951a06b4.rlib > /dev/null
...
error: failed to decompress '.debug_ranges', zlib is not available
error: failed to decompress '.debug_line', zlib is not available
error: failed to decompress '.debug_str', zlib is not available
error: failed to decompress '.debug_info', zlib is not available
error: failed to decompress '.debug_abbrev', zlib is not available
error: failed to decompress '.debug_loc', zlib is not available
error: failed to decompress '.debug_aranges', zlib is not available
error: failed to decompress '.debug_ranges', zlib is not available
error: failed to decompress '.debug_line', zlib is not available
error: failed to decompress '.debug_str', zlib is not available
error: failed to decompress '.debug_info', zlib is not available
error: failed to decompress '.debug_abbrev', zlib is not available
error: failed to decompress '.debug_loc', zlib is not available
error: failed to decompress '.debug_ranges', zlib is not available
error: failed to decompress '.debug_line', zlib is not available
error: failed to decompress '.debug_str', zlib is not available
error: failed to decompress '.debug_info', zlib is not available
error: failed to decompress '.debug_abbrev', zlib is not available
error: failed to decompress '.debug_loc', zlib is not available
error: failed to decompress '.debug_aranges', zlib is not available
error: failed to decompress '.debug_line', zlib is not available
error: failed to decompress '.debug_str', zlib is not available
error: failed to decompress '.debug_info', zlib is not available
error: failed to decompress '.debug_abbrev', zlib is not available
error: failed to decompress '.debug_loc', zlib is not available
error: failed to decompress '.debug_aranges', zlib is not available
error: failed to decompress '.debug_ranges', zlib is not available
error: failed to decompress '.debug_line', zlib is not available
error: failed to decompress '.debug_str', zlib is not available
...
