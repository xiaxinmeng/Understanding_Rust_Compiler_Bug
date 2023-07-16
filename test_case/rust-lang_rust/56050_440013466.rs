plain
travis_time:end:34a3c89b:start=1542650783378931275,finish=1542650838005588580,duration=54626657305
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:43:17] 
[00:43:17] warning: `[is_unicast_link_local]` cannot be resolved, ignoring it...
[00:43:17]     --> libstd/net/ip.rs:1196:5
[00:43:17]      |
[00:43:17] 1196 | /     /// Returns [`true`] if the address is a unicast link-local address (`fe80::/64`).
[00:43:17] 1197 | |     ///
[00:43:17] 1198 | |     /// A common mis-conception is to think that "unicast link-local addresses start with
[00:43:17] 1199 | |     /// `fe80::`", but the [IETF RFC 4291] actually defines a stricter format for these addresses:
[00:43:17] ...    |
[00:43:17] 1245 | |     /// [`is_unicast_link_local()`](#method.is_unicast_link_local)
[00:43:17]      | |_______^
[00:43:17]      |
[00:43:17]      = note: the link appears in this line:
[00:43:17]              
[00:43:17]              
[00:43:17]               If you need a less strict validation use [`is_unicast_link_local()`] instead.
[00:43:17]      = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[00:43:17] 
[00:43:17] 
[00:43:17] warning: `[is_unicast_link_local_strict]` cannot be resolved, ignoring it...
[00:43:17]      |
[00:43:17]      |
[00:43:17] 1254 | /     /// Returns [`true`] if the address is a unicast link-local address (`fe80::/10`).
[00:43:17] 1255 | |     ///
[00:43:17] 1256 | |     /// This method returns [`true`] for addresses in the range reserved by [RFC 4291 section 2.4],
[00:43:17] 1257 | |     /// i.e. addresses with the following format:
[00:43:17] ...    |
[00:43:17] 1304 | |     /// [`is_unicast_link_local_strict()`](#method.is_unicast_link_local_strict)
[00:43:17]      | |_______^
[00:43:17]      |
[00:43:17]      = note: the link appears in this line:
[00:43:17]              
[00:43:17]              
[00:43:17]               unicast link-local addresses, whereas [`is_unicast_link_local_strict()`] does not. If you
[00:43:17]      = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[00:43:17] 
[00:43:17] 
[00:43:17] warning: `[is_unicast_link_local_strict]` cannot be resolved, ignoring it...
[00:43:17]      |
[00:43:17]      |
[00:43:17] 1254 | /     /// Returns [`true`] if the address is a unicast link-local address (`fe80::/10`).
[00:43:17] 1255 | |     ///
[00:43:17] 1256 | |     /// This method returns [`true`] for addresses in the range reserved by [RFC 4291 section 2.4],
[00:43:17] 1257 | |     /// i.e. addresses with the following format:
[00:43:17] ...    |
[00:43:17] 1304 | |     /// [`is_unicast_link_local_strict()`](#method.is_unicast_link_local_strict)
[00:43:17]      | |_______^
[00:43:17]      |
[00:43:17]      = note: the link appears in this line:
[00:43:17]              
[00:43:17]              
[00:43:17]               [`is_unicast_link_local_strict()`].
[00:43:17]      = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[00:43:17] 
[00:43:17] warning: `[chunks]` cannot be resolved, ignoring it...
[00:43:17]    --> /checkout/src/libcore/slice/mod.rs:860:52
---
[00:47:38] .................................................................................................... 100/5040
[00:47:41] .................................................................................................... 200/5040
[00:47:43] .............................ii............................................ii...................ii.. 300/5040
[00:47:46] ..............................................................................................iii... 400/5040
[00:47:49] .....iiiiiiii.iii............................iii...........................................i........ 500/5040
[00:47:55] .................................................................................................... 700/5040
[00:48:01] ..................................................................................i...........i..... 800/5040
[00:48:04] .................................................................................................... 900/5040
[00:48:07] .iiiii..................ii.iiii..................................................................... 1000/5040
---
[00:48:41] .................................................................................................... 2200/5040
[00:48:45] .................................................................................................... 2300/5040
[00:48:48] .................................................................................................... 2400/5040
[00:48:52] .................................................................................................... 2500/5040
[00:48:55] .....................................................................................iiiiiiiii...... 2600/5040
[00:49:02] ...................................................ii............................................... 2800/5040
[00:49:04] .................................................................................................... 2900/5040
[00:49:08] .................................................................................................... 3000/5040
[00:49:11] ..............................................i..................................................... 3100/5040
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:02:20] 
[01:02:20] running 117 tests
[01:02:23] i..ii...iii..iiii.....i...i.........i..iii...........i.....i.....ii...i..i.ii..............i...ii..i 100/117
[01:02:23] i.i.....iiii.....
[01:02:23] 
[01:02:23]  finished in 3.401
[01:02:23] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:02:37] 
[01:02:37] running 118 tests
[01:03:00] .iiiii...i.....i..i...i..i.i..i.i..i.....i..i....i..........iiii.........i.i....i...i.......ii.i.i.i 100/118
[01:03:04] ......iii.i.....ii
[01:03:04] 
[01:03:04]  finished in 27.119
[01:03:04] travis_fold:end:test_debuginfo

---
[01:21:43] thread '<unnamed>' panicked at 'explicit panic', libstd/io/buffered.rs:1297:17
[01:21:43] thread '<unnamed>' panicked at 'explicit panic', libstd/io/stdio.rs:758:13
[01:21:43] .................................................................................................... 300/769
[01:21:43] thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
[01:21:43]   left: `true`,
[01:21:43]  right: `false`', libstd/net/ip.rs:1946:13
[01:21:43] thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
[01:21:43]   left: `true`,
[01:21:43]  right: `false`', libstd/net/ip.rs:2041:13
[01:21:43] ....................................F........F...................................................... 400/769
[01:21:44] .................................................................................................... 500/769
[01:21:44] thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', libcore/result.rs:1009:5
[01:21:44] thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', libcore/result.rs:1009:5
[01:21:44] thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: "SendError(..)"', libcore/result.rs:1009:5
---
[01:21:46] dur: 107ns
_64-unknown-linux-gnu
136812 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release
134664 ./obj/build/bootstrap/debug/incremental/bootstrap-zemjd6kcyh2u
134660 ./obj/build/bootstrap/debug/incremental/bootstrap-zemjd6kcyh2u/s-f6to2up8ey-1mxn0m2-2q1xp1m83q6vz
130760 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu
130756 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release
127624 ./obj/build/x86_64-unknown-linux-gnu/test/mir-opt
123684 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps
---
67092 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu
67088 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release
62416 ./obj/build/x86_64-unknown-linux-gnu/test/run-pass
56436 ./src/llvm/test/MC
56044 E" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
