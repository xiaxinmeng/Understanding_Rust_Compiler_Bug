
$ cargo new --bin app && cd $_

$ edit Cargo.toml && tail -n2 $_
[profile.dev]
panic = "abort"

$ edit src/main.rs && cat $_
#![feature(lang_items)]
#![no_main]
#![no_std]

#[no_mangle]
pub fn _start() -> ! {
    panic!()
}

#[lang = "panic_fmt"]
extern "C" fn panic_fmt() -> ! {
    loop {}
}

$ cargo rustc -- -C link-arg=-nostartfiles
error: linking with `cc` failed: exit code: 1
  |
  = note: "cc" "-Wl,--as-needed" "-Wl,-z,noexecstack" "-m64" "-L" "/home/japaric/.multirust/toolchains/nightly-2016-12-06-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib" "/home/japaric/tmp/app/target/debug/deps/app-55df21ded2024f97.0.o" "-o" "/home/japaric/tmp/app/target/debug/deps/app-55df21ded2024f97" "-Wl,--gc-sections" "-pie" "-nodefaultlibs" "-L" "/home/japaric/tmp/app/target/debug/deps" "-L" "/home/japaric/.multirust/toolchains/nightly-2016-12-06-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-Bstatic" "-Wl,-Bdynamic" "/home/japaric/.multirust/toolchains/nightly-2016-12-06-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-1357b93f.rlib" "-nostartfiles"
  = note: /home/japaric/.multirust/toolchains/nightly-2016-12-06-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-1357b93f.rlib(core-1357b93f.0.o): In function `core::panicking::panic_fmt':
/buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/libcore/num/bignum.rs:489: undefined reference to `rust_begin_unwind'
collect2: error: ld returned 1 exit status


error: aborting due to previous error

error: Could not compile `app`.

$ nm -C target/debug/deps/app*.0.o
0000000000000000 t rust_begin_unwind
0000000000000000 N __rustc_debug_gdb_scripts_section__
0000000000000000 T _start
0000000000000000 r str.0
0000000000000000 r str.1
0000000000000000 d app::_start::_MSG_FILE_LINE::h21fcbd8366a7e1b6
                 U core::panicking::panic::h194ce5d68a8f28a1
0000000000000000 t drop::h8b70c8901008835d
