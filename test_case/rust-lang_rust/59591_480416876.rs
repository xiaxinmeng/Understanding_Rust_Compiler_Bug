plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:1b90501e
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[01:45:22]    Compiling bytes v0.4.11
[01:45:22]    Compiling clap v2.32.0
[01:45:26]    Compiling tar v0.4.20
[01:45:38]    Compiling url v1.7.2
[01:45:39] error[E0271]: type mismatch resolving `for<'r> <[closure@/cargo/registry/src/github.com-1ecc6299db9ec823/url-1.7.2/src/form_urlencoded.rs:99:51: 99:65] as std::ops::FnOnce<(&'r u8,)>>::Output == bool`
[01:45:39]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/url-1.7.2/src/form_urlencoded.rs:99:41
[01:45:39]    |
[01:45:39] 99 |             let mut split2 = self.input.splitn(2, |&b| b == b'&');
[01:45:39]    |                                         ^^^^^^ expected bound lifetime parameter, found concrete lifetime
[01:45:39]    |
[01:45:39]    = note: required because of the requirements on the impl of `std::needle::Needle<&[u8]>` for `[closure@/cargo/registry/src/github.com-1ecc6299db9ec823/url-1.7.2/src/form_urlencoded.rs:99:51: 99:65]`
[01:45:39] error[E0631]: type mismatch in closure arguments
[01:45:39] error[E0631]: type mismatch in closure arguments
[01:45:39]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/url-1.7.2/src/form_urlencoded.rs:99:41
[01:45:39]    |
[01:45:39] 99 |             let mut split2 = self.input.splitn(2, |&b| b == b'&');
[01:45:39]    |                                         ^^^^^^    -------------- found signature of `fn(&_) -> _`
[01:45:39]    |                                         |
[01:45:39]    |                                         expected signature of `for<'r> fn(&'r u8) -> _`
[01:45:39]    |
[01:45:39]    = note: required because of the requirements on the impl of `std::needle::Needle<&[u8]>` for `[closure@/cargo/registry/src/github.com-1ecc6299db9ec823/url-1.7.2/src/form_urlencoded.rs:99:51: 99:65]`
[01:45:39] 
[01:45:39] error[E0599]: no method named `next` found for type `std::slice::SplitN<'_, u8, [closure@/cargo/registry/src/github.com-1ecc6299db9ec823/url-1.7.2/src/form_urlencoded.rs:99:51: 99:65]>` in the current scope
[01:45:39]    --> /cargo/registry/src/github.com-1ecc6299db9ec823/url-1.7.2/src/form_urlencoded.rs:100:35
[01:45:39] 100 |             let sequence = split2.next().unwrap();
[01:45:39]     |                                   ^^^^
[01:45:39]     |
[01:45:39]     = note: the method `next` exists but the following trait bounds were not satisfied:
[01:45:39]     = note: the method `next` exists but the following trait bounds were not satisfied:
[01:45:39]             `std::slice::SplitN<'_, u8, [closure@/cargo/registry/src/github.com-1ecc6299db9ec823/url-1.7.2/src/form_urlencoded.rs:99:51: 99:65]> : std::iter::Iterator`
[01:45:39] 
[01:45:39] error[E0599]: no method named `next` found for type `std::slice::SplitN<'_, u8, [closure@/cargo/registry/src/github.com-1ecc6299db9ec823/url-1.7.2/src/form_urlencoded.rs:99:51: 99:65]>` in the current scope
[01:45:39]    --> /cargo/registry/src/github.com-1ecc6299db9ec823/url-1.7.2/src/form_urlencoded.rs:101:33
[01:45:39]     |
[01:45:39] 101 |             self.input = split2.next().unwrap_or(&[][..]);
[01:45:39]     |
[01:45:39]     = note: the method `next` exists but the following trait bounds were not satisfied:
[01:45:39]     = note: the method `next` exists but the following trait bounds were not satisfied:
[01:45:39]             `std::slice::SplitN<'_, u8, [closure@/cargo/registry/src/github.com-1ecc6299db9ec823/url-1.7.2/src/form_urlencoded.rs:99:51: 99:65]> : std::iter::Iterator`
[01:45:39] error: aborting due to 4 previous errors
[01:45:39] 
[01:45:39] Some errors occurred: E0271, E0599, E0631.
[01:45:39] For more information about an error, try `rustc --explain E0271`.
---
travis_time:end:1fbffff8:start=1554497224101826263,finish=1554497224132702171,duration=30875908
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00be85df
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:184e7428
travis_time:start:184e7428
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1ab39fbe
$ dmesg | grep -i kill
