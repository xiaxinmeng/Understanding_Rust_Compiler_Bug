
murphyj@murphyj-AX370:~/src/spacebox-org/spacebox$ cargo doc --open
    Checking void v1.0.2
   Compiling version_check v0.1.4
 Documenting void v1.0.2
    Checking libc v0.2.43
    Checking rand_core v0.2.1
 Documenting stable_deref_trait v1.1.1
    Checking stable_deref_trait v1.1.1
 Documenting rand_core v0.2.1
 Documenting libc v0.2.43
 Documenting cfg-if v0.1.5
 Documenting scopeguard v0.3.3
   Compiling typenum v1.10.0
   Compiling proc-macro2 v0.4.17
    Checking byteorder v1.2.6
 Documenting byteorder v1.2.6
    Checking scopeguard v0.3.3
    Checking cfg-if v0.1.5
   Compiling unicode-xid v0.1.0
 Documenting futures v0.1.23
   Compiling regex v1.0.4
 Documenting unicode-xid v0.1.0
    Checking ucd-util v0.1.1
 Documenting ucd-util v0.1.1
    Checking lazycell v0.6.0
 Documenting slab v0.4.1
 Documenting lazycell v0.6.0
    Checking slab v0.4.1
    Checking futures v0.1.23
   Compiling ryu v0.2.6
 Documenting utf8-ranges v1.0.1
 Documenting crossbeam-utils v0.5.0
    Checking utf8-ranges v1.0.1
   Compiling serde v1.0.76
    Checking crossbeam-utils v0.5.0
 Documenting lazycell v1.0.0
 Documenting arrayref v0.3.4
    Checking bitflags v1.0.4
 Documenting same-file v1.0.3
    Checking same-file v1.0.3
    Checking arrayref v0.3.4
    Checking byte-tools v0.2.0
 Documenting fnv v1.0.6
    Checking fnv v1.0.6
 Documenting byte-tools v0.2.0
 Documenting bitflags v1.0.4
    Checking lazycell v1.0.0
 Documenting itoa v0.4.2
 Documenting keccak v0.1.0
 Documenting crossbeam v0.3.2
    Checking keccak v0.1.0
    Checking crossbeam v0.3.2
    Checking itoa v0.4.2
    Checking unreachable v1.0.0
    Checking owning_ref v0.3.3
    Checking log v0.4.5
 Documenting log v0.4.5
 Documenting owning_ref v0.3.3
   Compiling lazy_static v1.1.0
    Checking regex-syntax v0.6.2
 Documenting regex-syntax v0.6.2
    Checking rand v0.5.5
    Checking iovec v0.1.2
    Checking net2 v0.2.33
    Checking memchr v2.0.2
    Checking num_cpus v1.8.0
    Checking inotify-sys v0.1.3
    Checking filetime v0.2.1
    Checking block-buffer v0.3.3
    Checking walkdir v2.2.5
    Checking smallvec v0.6.5
 Documenting unreachable v1.0.0
    Checking lock_api v0.1.3
 Documenting proc-macro2 v0.4.17
 Documenting walkdir v2.2.5
 Documenting typenum v1.10.0
    Checking bytes v0.4.10
    Checking aho-corasick v0.6.8
 Documenting ryu v0.2.6
 Documenting serde v1.0.76
    Checking mio v0.6.15
 Documenting block-buffer v0.3.3
 Documenting smallvec v0.6.5
 Documenting lazy_static v1.1.0
    Checking quote v0.6.8
error: internal compiler error: librustc/traits/structural_impls.rs:178: impossible case reached

thread '<unnamed>' panicked at 'Box<Any>', librustc_errors/lib.rs:578:9
note: Run with `RUST_BACKTRACE=1` for a backtrace.

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.29.0-beta.12 (4dcf42f98 2018-09-04) running on x86_64-unknown-linux-gnu

 Documenting tokio-executor v0.1.4
error: Could not document `typenum`.

Caused by:
  process didn't exit successfully: `rustdoc --crate-name typenum /home/murphyj/.cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.10.0/src/lib.rs --cap-lints allow -o /home/murphyj/src/spacebox-org/spacebox/target/doc -L dependency=/home/murphyj/src/spacebox-org/spacebox/target/debug/deps` (exit code: 1)
warning: build failed, waiting for other jobs to finish...
error: build failed
