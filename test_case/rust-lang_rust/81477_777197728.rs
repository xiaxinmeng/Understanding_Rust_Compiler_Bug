plain

################################################                          67.8%
######################################################################## 100.0%
extracting /checkout/obj/build/cache/2021-01-28/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.gz
    Updating git repository `https://github.com/webdesus/fs_extra`
---
   Compiling unicode-script v0.5.2
   Compiling punycode v0.4.1
   Compiling rustc-demangle v0.1.18
   Compiling pathdiff v0.2.0
   Compiling fs_extra v1.2.0 (https://github.com/webdesus/fs_extra#752fc4d1)
   Compiling tracing-core v0.1.17
   Compiling crossbeam-utils v0.6.6
   Compiling thread_local v1.0.1
   Compiling sharded-slab v0.0.9
---
   Compiling punycode v0.4.1
   Compiling rustc-demangle v0.1.18
   Compiling unicode-script v0.5.2
   Compiling pathdiff v0.2.0
   Compiling fs_extra v1.2.0 (https://github.com/webdesus/fs_extra#752fc4d1)
   Compiling lock_api v0.4.1
   Compiling tracing-core v0.1.17
   Compiling crossbeam-utils v0.6.6
   Compiling thread_local v1.0.1
---
    Checking cargo-platform v0.1.1 (/tmp/cargo/crates/cargo-platform)
    Checking serde_ignored v0.1.2
    Checking crates-io v0.33.0 (/tmp/cargo/crates/crates-io)
    Checking rustfix v0.5.1
error: Trailing semicolon in macro
   --> src/cargo/ops/cargo_new.rs:792:27
    |
792 |                 Err(e) => log::warn!("failed to call rustfmt: {}", e),
    |
    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: Trailing semicolon in macro
error: Trailing semicolon in macro
   --> src/cargo/ops/fix.rs:435:22
    |
435 |         .inspect(|y| trace!("line: {}", y))
    |
    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: Trailing semicolon in macro
error: Trailing semicolon in macro
   --> src/cargo/sources/git/utils.rs:170:31
    |
170 |                     Err(e) => debug!("failed reset after fetch {:?}", e),
    |
    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: Trailing semicolon in macro
error: Trailing semicolon in macro
   --> src/cargo/sources/git/utils.rs:738:19
    |
738 |         Err(e) => debug!("failed to check github {:?}", e),
    |
    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: Trailing semicolon in macro
error: Trailing semicolon in macro
   --> src/cargo/sources/git/utils.rs:920:19
    |
920 |         Err(e) => debug!("git-gc failed to spawn: {}", e),
    |
    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: Trailing semicolon in macro
error: Trailing semicolon in macro
   --> src/cargo/sources/registry/index.rs:529:27
    |
529 |                 Err(e) => log::debug!("cache missing for {:?} error: {}", relative, e),
    |
    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: Trailing semicolon in macro
error: Trailing semicolon in macro
   --> src/cargo/util/diagnostic_server.rs:272:31
    |
272 |                     Err(e) => warn!("invalid diagnostics message: {}", e),
    |
    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: Trailing semicolon in macro
error: Trailing semicolon in macro
   --> src/cargo/util/paths.rs:511:19
    |
511 |         Ok(()) => log::debug!("set file mtime {} to {}", path.display(), time),
    |
    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: Trailing semicolon in macro
error: Trailing semicolon in macro
   --> src/cargo/util/paths.rs:512:19
    |
512 |           Err(e) => log::warn!(
    |  ___________________^
513 | |             "could not set mtime of {} to {}: {:?}",
514 | |             path.display(),
516 | |             e
517 | |         ),
    | |_________^
    |
    |
    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: Trailing semicolon in macro
   --> src/cargo/util/rustc.rs:247:27
    |
247 |                 Ok(()) => info!("updated rustc info cache"),
    |
    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: Trailing semicolon in macro
error: Trailing semicolon in macro
   --> src/cargo/util/rustc.rs:248:27
    |
248 |                 Err(e) => warn!("failed to update rustc info cache: {}", e),
    |
    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to 11 previous errors
