
$ docker run --rm -it alpine /bin/sh
# apk update
# URL="https://www.dropbox.com/sh/t3vs8sjc3u8rzzr/AACjn6w1Dq75THcV62YGCQQ5a/2016-07-25.tar.gz?dl=1"
# apk add curl # for the command below
# curl -sL $URL | tar -xz
# apk add gcc # need libgcc_s for rustc to work
# rustc -V
# ldd $(which rustc)
        /lib/ld-musl-x86_64.so.1 (0x560e95330000)
        librustc_driver-3db96603e7de68a8.so => /bin/../lib/librustc_driver-3db96603e7de68a8.so (0x7fb4c1f44000)
        libstd-b357b9eff40a747f.so => /bin/../lib/libstd-b357b9eff40a747f.so (0x7fb4c19d9000)
        libc.so => /lib/ld-musl-x86_64.so.1 (0x560e95330000)
        (..)
        libgcc_s.so.1 => /usr/lib/libgcc_s.so.1 (0x7fb4ba14f000)
        (..)
# apk add musl-dev # need a bunch of libraries (rt, dl, c, m, etc) when linking a Rust binary
# echo 'fn main() { println!("Hello, world!") }' > hello.rs
# rustc hello.rs
# ./hello
Hello, world!
# echo 'fn main() { panic!("Hello, world!") }' > panic.rs
# rustc panic.rs
# RUST_BACKTRACE=1 ./panic # backtraces work
thread 'main' panicked at 'Hello, world!', panic.rs:1
stack backtrace:
   1:     0x558cae97656f - std::sys::backtrace::tracing::imp::write::h4ec66725a4c8d28d
   2:     0x558cae978c6b - std::panicking::default_hook::_{{closure}}::hcb8d8a999157470b
   3:     0x558cae978125 - std::panicking::default_hook::hc2e39a27ba5d2d45
   4:     0x558cae9786de - std::panicking::rust_panic_with_hook::h59941143cbee269d
   5:     0x558cae971c53 - std::panicking::begin_panic::hc74c3c2af73ca1a0
   6:     0x558cae971d99 - panic::main::h27a6e2b4553ca3bd
   7:     0x558cae9783a8 - std::panicking::try::call::h494c5dbd41ce3146
   8:     0x558cae97e936 - __rust_maybe_catch_panic
   9:     0x558cae977981 - std::rt::lang_start::hfb76de782d5db06c
  10:     0x558cae971dc9 - main
  11:     0x7fc9ee95772e - <unknown>
