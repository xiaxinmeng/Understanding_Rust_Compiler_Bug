plain
[01:05:52]    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[01:05:53] [RUSTC-TIMING] panic_unwind test:false 0.304
[01:05:53] warning: dropping unsupported crate type `dylib` for target `x86_64-unknown-cloudabi`
[01:05:53] 
[01:05:56] error[E0599]: no method named `read_vectored` found for type `sys::cloudabi::shims::net::TcpStream` in the current scope
[01:05:56]     |
[01:05:56]     |
[01:05:56] 573 |         self.0.read_vectored(bufs)
[01:05:56]     | 
[01:05:56]     | 
[01:05:56]    ::: src/libstd/sys/cloudabi/shims/net.rs:10:1
[01:05:56]     |
[01:05:56] 10  | pub struct TcpStream(Void);
[01:05:56]     | --------------------------- method `read_vectored` not found for this
[01:05:56]     = help: items from traits can only be used if the trait is implemented and in scope
[01:05:56]     = note: the following trait defines an item `read_vectored`, perhaps you need to implement it:
[01:05:56]             candidate #1: `io::Read`
[01:05:56] 
[01:05:56] 
[01:05:56] error[E0599]: no method named `write_vectored` found for type `sys::cloudabi::shims::net::TcpStream` in the current scope
[01:05:56]     |
[01:05:56]     |
[01:05:56] 586 |         self.0.write_vectored(bufs)
[01:05:56]     | 
[01:05:56]     | 
[01:05:56]    ::: src/libstd/sys/cloudabi/shims/net.rs:10:1
[01:05:56]     |
[01:05:56] 10  | pub struct TcpStream(Void);
[01:05:56]     | --------------------------- method `write_vectored` not found for this
[01:05:56]     = help: items from traits can only be used if the trait is implemented and in scope
[01:05:56]     = note: the following trait defines an item `write_vectored`, perhaps you need to implement it:
[01:05:56]             candidate #1: `io::Write`
[01:05:56] 
[01:05:56] 
[01:05:56] error[E0599]: no method named `read_vectored` found for type `sys::cloudabi::shims::net::TcpStream` in the current scope
[01:05:56]     |
[01:05:56]     |
[01:05:56] 596 |         self.0.read_vectored(bufs)
[01:05:56]     | 
[01:05:56]     | 
[01:05:56]    ::: src/libstd/sys/cloudabi/shims/net.rs:10:1
[01:05:56]     |
[01:05:56] 10  | pub struct TcpStream(Void);
[01:05:56]     | --------------------------- method `read_vectored` not found for this
[01:05:56]     = help: items from traits can only be used if the trait is implemented and in scope
[01:05:56]     = note: the following trait defines an item `read_vectored`, perhaps you need to implement it:
[01:05:56]             candidate #1: `io::Read`
[01:05:56] 
[01:05:56] 
[01:05:56] error[E0599]: no method named `write_vectored` found for type `sys::cloudabi::shims::net::TcpStream` in the current scope
[01:05:56]     |
[01:05:56]     |
[01:05:56] 609 |         self.0.write_vectored(bufs)
[01:05:56]     | 
[01:05:56]     | 
[01:05:56]    ::: src/libstd/sys/cloudabi/shims/net.rs:10:1
[01:05:56]     |
[01:05:56] 10  | pub struct TcpStream(Void);
[01:05:56]     | --------------------------- method `write_vectored` not found for this
[01:05:56]     = help: items from traits can only be used if the trait is implemented and in scope
[01:05:56]     = note: the following trait defines an item `write_vectored`, perhaps you need to implement it:
[01:05:56]             candidate #1: `io::Write`
[01:05:56] 
---
travis_time:end:0185cfdc:start=1551005519727658129,finish=1551005519748345908,duration=20687779
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0a92f516
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0abf0c7c
travis_time:start:0abf0c7c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0cc0bc70
$ dmesg | grep -i kill
