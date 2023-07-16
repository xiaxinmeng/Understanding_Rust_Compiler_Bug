plain
[TIMING] Std { target: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None }, compiler: Compiler { stage: 2, host: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None } } } -- 0.000
Build completed successfully in 0:16:44
+ RUSTC_BOOTSTRAP=1
+ ./build/x86_64-unknown-linux-gnu/stage2/bin/rustc --edition=2018 --crate-type=lib ../library/core/src/lib.rs
error: environment variable `CARGO_PKG_NAME` not defined
   --> ../library/core/src/../../std/src/primitive_docs.rs:103:62
    |
103 | #[doc = concat!("[`exit`]: ", include_str!(concat!("../../", env!("CARGO_PKG_NAME"), "/primitive_docs/process_exit.md")))]
    |
    = note: this error originates in the macro `env` (in Nightly builds, run with -Z macro-backtrace for more info)


error: environment variable `CARGO_PKG_NAME` not defined
   --> ../library/core/src/../../std/src/primitive_docs.rs:188:64
    |
188 | #[doc = concat!("[`String`]: ", include_str!(concat!("../../", env!("CARGO_PKG_NAME"), "/primitive_docs/string_string.md")))]
    |
    = note: this error originates in the macro `env` (in Nightly builds, run with -Z macro-backtrace for more info)


error: environment variable `CARGO_PKG_NAME` not defined
   --> ../library/core/src/../../std/src/primitive_docs.rs:264:62
    |
264 | #[doc = concat!("[`File`]: ", include_str!(concat!("../../", env!("CARGO_PKG_NAME"), "/primitive_docs/fs_file.md")))]
    |
    = note: this error originates in the macro `env` (in Nightly builds, run with -Z macro-backtrace for more info)


error: environment variable `CARGO_PKG_NAME` not defined
   --> ../library/core/src/../../std/src/primitive_docs.rs:303:64
    |
303 | #[doc = concat!("[`String`]: ", include_str!(concat!("../../", env!("CARGO_PKG_NAME"), "/primitive_docs/string_string.md")))]
    |
    = note: this error originates in the macro `env` (in Nightly builds, run with -Z macro-backtrace for more info)


error: environment variable `CARGO_PKG_NAME` not defined
   --> ../library/core/src/../../std/src/primitive_docs.rs:495:66
    |
495 | #[doc = concat!("[`into_raw`]: ", include_str!(concat!("../../", env!("CARGO_PKG_NAME"), "/primitive_docs/box_into_raw.md")))]
    |
    = note: this error originates in the macro `env` (in Nightly builds, run with -Z macro-backtrace for more info)


error: environment variable `CARGO_PKG_NAME` not defined
    --> ../library/core/src/../../std/src/primitive_docs.rs:1125:71
     |
1125 | #[doc = concat!("[`ToSocketAddrs`]: ", include_str!(concat!("../../", env!("CARGO_PKG_NAME"), "/primitive_docs/net_tosocketaddrs.md")))]
     |
     = note: this error originates in the macro `env` (in Nightly builds, run with -Z macro-backtrace for more info)


error: environment variable `CARGO_PKG_NAME` not defined
    --> ../library/core/src/../../std/src/primitive_docs.rs:1146:62
     |
1146 | #[doc = concat!("[`Seek`]: ", include_str!(concat!("../../", env!("CARGO_PKG_NAME"), "/primitive_docs/io_seek.md")))]
     |
     = note: this error originates in the macro `env` (in Nightly builds, run with -Z macro-backtrace for more info)


error: environment variable `CARGO_PKG_NAME` not defined
    --> ../library/core/src/../../std/src/primitive_docs.rs:1147:65
     |
1147 | #[doc = concat!("[`BufRead`]: ", include_str!(concat!("../../", env!("CARGO_PKG_NAME"), "/primitive_docs/io_bufread.md")))]
     |
     = note: this error originates in the macro `env` (in Nightly builds, run with -Z macro-backtrace for more info)


error: environment variable `CARGO_PKG_NAME` not defined
    --> ../library/core/src/../../std/src/primitive_docs.rs:1148:62
     |
1148 | #[doc = concat!("[`Read`]: ", include_str!(concat!("../../", env!("CARGO_PKG_NAME"), "/primitive_docs/io_read.md")))]
     |
     = note: this error originates in the macro `env` (in Nightly builds, run with -Z macro-backtrace for more info)


error: environment variable `CARGO_PKG_NAME` not defined
    --> ../library/core/src/../../std/src/primitive_docs.rs:1149:67
     |
1149 | #[doc = concat!("[`io::Write`]: ", include_str!(concat!("../../", env!("CARGO_PKG_NAME"), "/primitive_docs/io_write.md")))]
     |
     = note: this error originates in the macro `env` (in Nightly builds, run with -Z macro-backtrace for more info)


error: unexpected token: `(/*ERROR*/)`
   --> ../library/core/src/../../std/src/primitive_docs.rs:103:9
    |
103 | #[doc = concat!("[`exit`]: ", include_str!(concat!("../../", env!("CARGO_PKG_NAME"), "/primitive_docs/process_exit.md")))]


error: unexpected token: `(/*ERROR*/)`
   --> ../library/core/src/../../std/src/primitive_docs.rs:188:9
    |
188 | #[doc = concat!("[`String`]: ", include_str!(concat!("../../", env!("CARGO_PKG_NAME"), "/primitive_docs/string_string.md")))]


error: unexpected token: `(/*ERROR*/)`
   --> ../library/core/src/../../std/src/primitive_docs.rs:264:9
    |
264 | #[doc = concat!("[`File`]: ", include_str!(concat!("../../", env!("CARGO_PKG_NAME"), "/primitive_docs/fs_file.md")))]


error: unexpected token: `(/*ERROR*/)`
   --> ../library/core/src/../../std/src/primitive_docs.rs:303:9
    |
303 | #[doc = concat!("[`String`]: ", include_str!(concat!("../../", env!("CARGO_PKG_NAME"), "/primitive_docs/string_string.md")))]


error: unexpected token: `(/*ERROR*/)`
   --> ../library/core/src/../../std/src/primitive_docs.rs:495:9
    |
495 | #[doc = concat!("[`into_raw`]: ", include_str!(concat!("../../", env!("CARGO_PKG_NAME"), "/primitive_docs/box_into_raw.md")))]


error: unexpected token: `(/*ERROR*/)`
    --> ../library/core/src/../../std/src/primitive_docs.rs:1125:9
     |
1125 | #[doc = concat!("[`ToSocketAddrs`]: ", include_str!(concat!("../../", env!("CARGO_PKG_NAME"), "/primitive_docs/net_tosocketaddrs.md")))]


error: unexpected token: `(/*ERROR*/)`
    --> ../library/core/src/../../std/src/primitive_docs.rs:1146:9
     |
1146 | #[doc = concat!("[`Seek`]: ", include_str!(concat!("../../", env!("CARGO_PKG_NAME"), "/primitive_docs/io_seek.md")))]


error: unexpected token: `(/*ERROR*/)`
    --> ../library/core/src/../../std/src/primitive_docs.rs:1147:9
     |
1147 | #[doc = concat!("[`BufRead`]: ", include_str!(concat!("../../", env!("CARGO_PKG_NAME"), "/primitive_docs/io_bufread.md")))]


error: unexpected token: `(/*ERROR*/)`
    --> ../library/core/src/../../std/src/primitive_docs.rs:1148:9
     |
1148 | #[doc = concat!("[`Read`]: ", include_str!(concat!("../../", env!("CARGO_PKG_NAME"), "/primitive_docs/io_read.md")))]


error: unexpected token: `(/*ERROR*/)`
    --> ../library/core/src/../../std/src/primitive_docs.rs:1149:9
     |
1149 | #[doc = concat!("[`io::Write`]: ", include_str!(concat!("../../", env!("CARGO_PKG_NAME"), "/primitive_docs/io_write.md")))]

error: aborting due to 20 previous errors

