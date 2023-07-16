plain
   Compiling bitflags v1.2.1
error[E0432]: unresolved import `std::env`
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/proc-macro2-1.0.24/build.rs:37:5
   |
37 | use std::env;
   |     ^^^^^^^^ no `env` in the root
error[E0432]: unresolved imports `std::process`, `std::process`
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/proc-macro2-1.0.24/build.rs:38:10
   |
   |
38 | use std::process::{self, Command};
   |          ^^^^^^^   ^^^^ no `process` in the root
   |          could not find `process` in `std`

error[E0432]: unresolved import `std::str`
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/proc-macro2-1.0.24/build.rs:39:5
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/proc-macro2-1.0.24/build.rs:39:5
   |
39 | use std::str;
   |     ^^^^^^^^ no `str` in the root

error: cannot find macro `cfg` in this scope
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/proc-macro2-1.0.24/build.rs:107:5
    |
107 |     cfg!(feature = "proc-macro")

error: cannot find macro `println` in this scope
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/proc-macro2-1.0.24/build.rs:42:5
   |
   |
42 |     println!("cargo:rerun-if-changed=build.rs");


error: cannot find macro `eprintln` in this scope
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/proc-macro2-1.0.24/build.rs:50:9
   |
50 |         eprintln!("Minimum supported rustc version is 1.31");


error: cannot find macro `cfg` in this scope
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/proc-macro2-1.0.24/build.rs:54:25
   |
54 |     let semver_exempt = cfg!(procmacro2_semver_exempt);

error: cannot find macro `println` in this scope
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/proc-macro2-1.0.24/build.rs:57:9
   |
   |
57 |         println!("cargo:rustc-cfg=procmacro2_semver_exempt");


error[E0433]: failed to resolve: maybe a missing crate `std`?
  |
2 | use std::process::Command;
2 | use std::process::Command;
  |     ^^^ maybe a missing crate `std`?

error: cannot find macro `cfg` in this scope
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/proc-macro2-1.0.24/build.rs:60:25
   |
60 |     if semver_exempt || cfg!(feature = "span-locations") {

error: cannot find macro `println` in this scope
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/proc-macro2-1.0.24/build.rs:61:9
   |
   |
61 |         println!("cargo:rustc-cfg=span_locations");

error: cannot find macro `println` in this scope
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/proc-macro2-1.0.24/build.rs:65:9
   |
   |
65 |         println!("cargo:rustc-cfg=no_libprocmacro_unwind_safe");

error: cannot find macro `println` in this scope
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/proc-macro2-1.0.24/build.rs:69:9
   |
   |
69 |         println!("cargo:rustc-cfg=no_bind_by_move_pattern_guard");

error: cannot find macro `println` in this scope
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/proc-macro2-1.0.24/build.rs:73:9
   |
   |
73 |         println!("cargo:rustc-cfg=lexerror_display");

error: cannot find macro `println` in this scope
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/proc-macro2-1.0.24/build.rs:77:9
   |
   |
77 |         println!("cargo:rustc-cfg=hygiene");

error: cannot find macro `println` in this scope
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/proc-macro2-1.0.24/build.rs:85:5
   |
   |
85 |     println!("cargo:rustc-cfg=use_proc_macro");

error: cannot find macro `println` in this scope
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/proc-macro2-1.0.24/build.rs:88:9
   |
   |
88 |         println!("cargo:rustc-cfg=wrap_proc_macro");

error: cannot find macro `println` in this scope
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/proc-macro2-1.0.24/build.rs:92:9
   |
   |
92 |         println!("cargo:rustc-cfg=proc_macro_span");

error: cannot find macro `println` in this scope
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/proc-macro2-1.0.24/build.rs:96:9
   |
   |
96 |         println!("cargo:rustc-cfg=super_unstable");

error[E0432]: unresolved import `std`
 --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.93/build.rs:1:5
  |
  |
1 | use std::env;
  |     ^^^ maybe a missing crate `std`?
error[E0432]: unresolved import `std`
 --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.93/build.rs:3:5
  |
3 | use std::str;
3 | use std::str;
  |     ^^^ maybe a missing crate `std`?
error: cannot find macro `println` in this scope
 --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.93/build.rs:7:5
  |
  |
7 |     println!("cargo:rerun-if-changed=build.rs");

error: cannot find macro `println` in this scope
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.93/build.rs:17:9
   |
   |
17 |         println!(
   |         ^^^^^^^

error: cannot find macro `println` in this scope
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.93/build.rs:30:13
   |
30 |             println!("cargo:rustc-cfg=freebsd10")

error[E0531]: cannot find tuple struct or tuple variant `Some` in this scope
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/proc-macro2-1.0.24/build.rs:45:9
   |
   |
45 |         Some(version) => version,

error: cannot find macro `println` in this scope
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.93/build.rs:32:32
   |
   |
32 |         Some(11) if libc_ci => println!("cargo:rustc-cfg=freebsd11"),

error: cannot find macro `println` in this scope
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.93/build.rs:33:32
   |
   |
33 |         Some(12) if libc_ci => println!("cargo:rustc-cfg=freebsd12"),

error[E0412]: cannot find type `Option` in this scope
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/proc-macro2-1.0.24/build.rs:115:23
    |
    |
115 | fn rustc_version() -> Option<RustcVersion> {
    |                       ^^^^^^ not found in this scope

error: cannot find macro `println` in this scope
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.93/build.rs:34:32
   |
34 |         Some(13) if libc_ci => println!("cargo:rustc-cfg=freebsd13"),

error: cannot find macro `println` in this scope
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.93/build.rs:35:27
   |
   |
35 |         Some(_) | None => println!("cargo:rustc-cfg=freebsd11"),

error[E0425]: cannot find function, tuple struct or tuple variant `Some` in this scope
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/proc-macro2-1.0.24/build.rs:121:25
    |
    |
121 |     if pieces.next() != Some("rustc 1") {

error: cannot find macro `println` in this scope
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.93/build.rs:40:9
   |
   |
40 |         println!("cargo:rustc-cfg=libc_deny_warnings");

error: cannot find macro `println` in this scope
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.93/build.rs:45:9
   |
   |
45 |         println!("cargo:rustc-cfg=libc_priv_mod_use");

error: cannot find macro `println` in this scope
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.93/build.rs:50:9
   |
   |
50 |         println!("cargo:rustc-cfg=libc_union");

error: cannot find macro `println` in this scope
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.93/build.rs:55:9
   |
   |
55 |         println!("cargo:rustc-cfg=libc_const_size_of");

error: cannot find macro `println` in this scope
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.93/build.rs:60:9
   |
   |
60 |         println!("cargo:rustc-cfg=libc_align");

error: cannot find macro `println` in this scope
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.93/build.rs:67:9
   |
   |
67 |         println!("cargo:rustc-cfg=libc_core_cvoid");

error: cannot find macro `println` in this scope
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.93/build.rs:72:9
   |
   |
72 |         println!("cargo:rustc-cfg=libc_packedN");

error: cannot find macro `println` in this scope
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.93/build.rs:73:9
   |
   |
73 |         println!("cargo:rustc-cfg=libc_cfg_target_vendor");


error[E0425]: cannot find value `None` in this scope
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/proc-macro2-1.0.24/build.rs:122:16
122 |         return None;
    |                ^^^^ not found in this scope

error: cannot find macro `println` in this scope
error: cannot find macro `println` in this scope
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.93/build.rs:78:9
   |
78 |         println!("cargo:rustc-cfg=libc_thread_local");

error: cannot find macro `panic` in this scope
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.93/build.rs:83:13
   |
   |
83 |             panic!("const-extern-fn requires a nightly compiler >= 1.40")

error: cannot find macro `println` in this scope
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.93/build.rs:85:9
   |
   |
85 |         println!("cargo:rustc-cfg=libc_const_extern_fn");

error[E0425]: cannot find function, tuple struct or tuple variant `Some` in this scope
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/proc-macro2-1.0.24/build.rs:125:5
    |
    |
125 |     Some(RustcVersion { minor, nightly })

error: cannot find macro `println` in this scope
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.93/build.rs:91:9
   |
   |
91 |         println!("cargo:rustc-link-lib=iconv");

error[E0531]: cannot find tuple struct or tuple variant `Some` in this scope
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/proc-macro2-1.0.24/build.rs:135:12
    |
    |
135 |     if let Some(rustflags) = env::var_os("RUSTFLAGS") {


error[E0408]: variable `None` is not bound in all patterns
   |
   |
35 |         Some(_) | None => println!("cargo:rustc-cfg=freebsd11"),
   |         ^^^^^^^   ---- variable not in all patterns
   |         pattern doesn't bind `None`
   |
   |
help: if you meant to match on a variant or a `const` item, consider making the path in the pattern qualified: `?::None`
   |
   |
35 |         Some(_) | None => println!("cargo:rustc-cfg=freebsd11"),

error[E0433]: failed to resolve: use of undeclared type `Command`
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.93/build.rs:106:24
    |
    |
106 |     let output = otry!(Command::new(rustc).arg("--version").output().ok());
    |                        ^^^^^^^ use of undeclared type `Command`
error[E0433]: failed to resolve: could not find `process` in `std`
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.93/build.rs:131:23
    |
    |
131 |     let output = std::process::Command::new("freebsd-version").output().ok();

error[E0433]: failed to resolve: use of undeclared type `String`
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.93/build.rs:140:18
    |
    |
140 |     let stdout = String::from_utf8(output.stdout).ok();
    |                  ^^^^^^ use of undeclared type `String`
error[E0531]: cannot find tuple struct or tuple variant `Some` in this scope
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.93/build.rs:29:9
   |
   |
29 |         Some(10) if libc_ci || rustc_dep_of_std => {

error[E0531]: cannot find tuple struct or tuple variant `Some` in this scope
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.93/build.rs:32:9
   |
   |
32 |         Some(11) if libc_ci => println!("cargo:rustc-cfg=freebsd11"),

error[E0531]: cannot find tuple struct or tuple variant `Some` in this scope
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.93/build.rs:33:9
   |
   |
33 |         Some(12) if libc_ci => println!("cargo:rustc-cfg=freebsd12"),

error[E0531]: cannot find tuple struct or tuple variant `Some` in this scope
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.93/build.rs:34:9
   |
   |
34 |         Some(13) if libc_ci => println!("cargo:rustc-cfg=freebsd13"),

error[E0531]: cannot find tuple struct or tuple variant `Some` in this scope
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.93/build.rs:35:9
   |
   |
35 |         Some(_) | None => println!("cargo:rustc-cfg=freebsd11"),

error[E0412]: cannot find type `Option` in this scope
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.93/build.rs:95:29
   |
   |
95 | fn rustc_minor_nightly() -> Option<(u32, bool)> {

error[E0531]: cannot find tuple struct or tuple variant `Some` in this scope
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.93/build.rs:99:17
    |
    |
99  |                 Some(e) => e,
...
...
105 |     let rustc = otry!(env::var_os("RUSTC"));
    |
    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: language item required, but not found: `eh_personality`
error[E0432]: unresolved import `std::env`
 --> /cargo/registry/src/github.com-1ecc6299db9ec823/syn-1.0.65/build.rs:1:5
  |
1 | use std::env;
1 | use std::env;
  |     ^^^^^^^^ no `env` in the root
error[E0531]: cannot find tuple struct or tuple variant `Some` in this scope
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.93/build.rs:99:17
    |
    |
99  |                 Some(e) => e,
...
...
106 |     let output = otry!(Command::new(rustc).arg("--version").output().ok());
    |
    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0432]: unresolved import `std::process`
---

error[E0531]: cannot find tuple struct or tuple variant `Some` in this scope
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.93/build.rs:99:17
    |
99  |                 Some(e) => e,
...
...
107 |     let version = otry!(str::from_utf8(&output.stdout).ok());
    |
    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: cannot find macro `println` in this scope
error: cannot find macro `println` in this scope
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/syn-1.0.65/build.rs:23:9
   |
23 |         println!("cargo:rustc-cfg=syn_disable_nightly_tests");

error[E0425]: cannot find function, tuple struct or tuple variant `Some` in this scope
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.93/build.rs:110:25
    |
    |
110 |     if pieces.next() != Some("rustc 1") {

error: cannot find macro `println` in this scope
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/syn-1.0.65/build.rs:19:9
   |
   |
19 |         println!("cargo:rustc-cfg=syn_no_const_vec_new");


error[E0425]: cannot find value `None` in this scope
    |
111 |         return None;
    |                ^^^^ not found in this scope


error: cannot find macro `println` in this scope
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/syn-1.0.65/build.rs:15:9
   |
15 |         println!("cargo:rustc-cfg=syn_omit_await_from_token_macro");

error[E0531]: cannot find tuple struct or tuple variant `Some` in this scope
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.93/build.rs:99:17
    |
    |
99  |                 Some(e) => e,
...
...
121 |     let nightly_raw = otry!(pieces.next()).split('-').nth(1);
    |
    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0531]: cannot find tuple struct or tuple variant `Some` in this scope
error[E0531]: cannot find tuple struct or tuple variant `Some` in this scope
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.93/build.rs:99:17
    |
99  |                 Some(e) => e,
...
...
125 |     let minor = otry!(otry!(minor).parse().ok());
    |
    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0531]: cannot find tuple struct or tuple variant `Some` in this scope
error[E0531]: cannot find tuple struct or tuple variant `Some` in this scope
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.93/build.rs:99:17
    |
99  |                 Some(e) => e,
...
...
125 |     let minor = otry!(otry!(minor).parse().ok());
    |
    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0531]: cannot find tuple struct or tuple variant `Some` in this scope
error[E0531]: cannot find tuple struct or tuple variant `Some` in this scope
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/syn-1.0.65/build.rs:10:9
   |
10 |         Some(compiler) => compiler,

error[E0425]: cannot find function, tuple struct or tuple variant `Some` in this scope
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.93/build.rs:127:5
    |
    |
127 |     Some((minor, nightly))

error[E0412]: cannot find type `Option` in this scope
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.93/build.rs:130:23
    |
    |
130 | fn which_freebsd() -> Option<i32> {

error[E0412]: cannot find type `Option` in this scope
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/syn-1.0.65/build.rs:32:23
   |
   |
32 | fn rustc_version() -> Option<Compiler> {
   |                       ^^^^^^ not found in this scope

error[E0425]: cannot find value `None` in this scope
    |
133 |         return None;
    |                ^^^^ not found in this scope


error[E0425]: cannot find function, tuple struct or tuple variant `Some` in this scope
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/syn-1.0.65/build.rs:37:25
   |
37 |     if pieces.next() != Some("rustc 1") {


error[E0425]: cannot find value `None` in this scope
    |
137 |         return None;
    |                ^^^^ not found in this scope


error[E0425]: cannot find value `None` in this scope
   |
38 |         return None;
   |                ^^^^ not found in this scope


error[E0425]: cannot find value `None` in this scope
    |
142 |         return None;
    |                ^^^^ not found in this scope


error[E0425]: cannot find function, tuple struct or tuple variant `Some` in this scope
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/syn-1.0.65/build.rs:42:5
   |
42 |     Some(Compiler { minor, nightly })

error[E0425]: cannot find function, tuple struct or tuple variant `Some` in this scope
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.93/build.rs:147:37
    |
    |
147 |         s if s.starts_with("10") => Some(10),

error[E0425]: cannot find function, tuple struct or tuple variant `Some` in this scope
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.93/build.rs:148:37
    |
    |
148 |         s if s.starts_with("11") => Some(11),

error[E0425]: cannot find function, tuple struct or tuple variant `Some` in this scope
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.93/build.rs:149:37
    |
    |
149 |         s if s.starts_with("12") => Some(12),

error[E0425]: cannot find function, tuple struct or tuple variant `Some` in this scope
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.93/build.rs:150:37
    |
    |
150 |         s if s.starts_with("13") => Some(13),


error[E0425]: cannot find value `None` in this scope
    |
151 |         _ => None,
    |              ^^^^ not found in this scope


error: requires `sized` lang_item
error: aborting due to 26 previous errors

Some errors have detailed explanations: E0412, E0425, E0432, E0531.
For more information about an error, try `rustc --explain E0412`.
For more information about an error, try `rustc --explain E0412`.
error: language item required, but not found: `eh_personality`

error: language item required, but not found: `eh_personality`

error: requires `sized` lang_item

error: requires `sized` lang_item
error: aborting due to 13 previous errors

Some errors have detailed explanations: E0412, E0425, E0432, E0531.
For more information about an error, try `rustc --explain E0412`.
---
warning: build failed, waiting for other jobs to finish...
error[E0601]: `main` function not found in crate `build_script_build`
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/log-0.4.11/build.rs:1:1
   |
1  | / //! This build script detects target platforms that lack proper support for
2  | | //! atomics and sets `cfg` flags accordingly.
4  | | use std::env;
...  |
...  |
13 | |     println!("cargo:rerun-if-changed=build.rs");
   | |_^ consider adding a `main` function to `/cargo/registry/src/github.com-1ecc6299db9ec823/log-0.4.11/build.rs`

error: aborting due to previous error


For more information about this error, try `rustc --explain E0601`.
error[E0432]: unresolved import `std::time`
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/instant-0.1.6/src/lib.rs:22:14
   |
22 | pub use std::time::Duration;
   |              ^^^^ could not find `time` in `std`

error[E0433]: failed to resolve: could not find `time` in `std`
 --> /cargo/registry/src/github.com-1ecc6299db9ec823/instant-0.1.6/src/native.rs:1:25
1 | pub type Instant = std::time::Instant;
1 | pub type Instant = std::time::Instant;
  |                         ^^^^ could not find `time` in `std`
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0432, E0433.
For more information about an error, try `rustc --explain E0432`.
For more information about an error, try `rustc --explain E0432`.
error[E0433]: failed to resolve: maybe a missing crate `std`?
  |
2 | use std::process::Command;
2 | use std::process::Command;
  |     ^^^ maybe a missing crate `std`?
error[E0432]: unresolved import `std`
 --> /cargo/registry/src/github.com-1ecc6299db9ec823/bitflags-1.2.1/build.rs:1:5
  |
1 | use std::env;
1 | use std::env;
  |     ^^^ maybe a missing crate `std`?

error[E0433]: failed to resolve: maybe a missing crate `std`?
  |
  |
3 | use std::str::{self, FromStr};
  |     ^^^ maybe a missing crate `std`?
error[E0432]: unresolved import `std`
 --> /cargo/registry/src/github.com-1ecc6299db9ec823/bitflags-1.2.1/build.rs:3:5
  |
  |
3 | use std::str::{self, FromStr};
  |     ^^^ maybe a missing crate `std`?
error: cannot find macro `println` in this scope
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/bitflags-1.2.1/build.rs:13:9
   |
   |
13 |         println!("cargo:rustc-cfg=bitflags_const_fn");

error[E0433]: failed to resolve: use of undeclared type `Command`
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/bitflags-1.2.1/build.rs:23:24
   |
   |
23 |     let output = match Command::new(rustc).arg("--version").output() {
   |                        ^^^^^^^ use of undeclared type `Command`
error[E0531]: cannot find tuple struct or tuple variant `Some` in this scope
 --> /cargo/registry/src/github.com-1ecc6299db9ec823/bitflags-1.2.1/build.rs:7:9
  |
  |
7 |         Some(minor) => minor,

error[E0412]: cannot find type `Option` in this scope
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/bitflags-1.2.1/build.rs:17:29
   |
   |
17 | fn rustc_minor_version() -> Option<u32> {
   |                             ^^^^^^ not found in this scope

error[E0531]: cannot find tuple struct or tuple variant `Some` in this scope
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/bitflags-1.2.1/build.rs:19:9
   |
19 |         Some(rustc) => rustc,


error[E0531]: cannot find tuple struct or tuple variant `Ok` in this scope
   |
   |
24 |         Ok(output) => output,
   |         ^^ not found in this scope
error[E0531]: cannot find tuple struct or tuple variant `Err` in this scope
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/bitflags-1.2.1/build.rs:25:9
   |
   |
25 |         Err(_) => return None,


error[E0425]: cannot find value `None` in this scope
   |
   |
25 |         Err(_) => return None,


error[E0531]: cannot find tuple struct or tuple variant `Ok` in this scope
   |
   |
29 |         Ok(version) => version,
   |         ^^ not found in this scope
error[E0531]: cannot find tuple struct or tuple variant `Err` in this scope
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/bitflags-1.2.1/build.rs:30:9
   |
   |
30 |         Err(_) => return None,


error[E0425]: cannot find value `None` in this scope
   |
   |
30 |         Err(_) => return None,

error[E0425]: cannot find function, tuple struct or tuple variant `Some` in this scope
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/bitflags-1.2.1/build.rs:34:25
   |
   |
34 |     if pieces.next() != Some("rustc 1") {


error[E0425]: cannot find value `None` in this scope
   |
35 |         return None;
   |                ^^^^ not found in this scope


error[E0531]: cannot find tuple struct or tuple variant `Some` in this scope
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/bitflags-1.2.1/build.rs:39:9
   |
39 |         Some(next) => next,


error: language item required, but not found: `eh_personality`

error: requires `sized` lang_item
error: aborting due to 20 previous errors

Some errors have detailed explanations: E0412, E0425, E0432, E0433, E0531.
For more information about an error, try `rustc --explain E0412`.
For more information about an error, try `rustc --explain E0412`.
error[E0433]: failed to resolve: maybe a missing crate `std`?
  |
2 | use std::process::Command;
2 | use std::process::Command;
  |     ^^^ maybe a missing crate `std`?

error[E0433]: failed to resolve: maybe a missing crate `std`?
  |
3 | use std::str::FromStr;
3 | use std::str::FromStr;
  |     ^^^ maybe a missing crate `std`?
error[E0432]: unresolved import `std`
 --> /cargo/registry/src/github.com-1ecc6299db9ec823/maybe-uninit-2.0.0/build.rs:1:5
  |
1 | use std::env;
1 | use std::env;
  |     ^^^ maybe a missing crate `std`?
error: cannot find macro `println` in this scope
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/maybe-uninit-2.0.0/build.rs:17:9
   |
   |
17 |         println!("cargo:rustc-cfg=native_uninit");

error: cannot find macro `println` in this scope
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/maybe-uninit-2.0.0/build.rs:14:9
   |
   |
14 |         println!("cargo:rustc-cfg=repr_transparent");

error: cannot find macro `println` in this scope
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/maybe-uninit-2.0.0/build.rs:11:9
   |
   |
11 |         println!("cargo:rustc-cfg=derive_copy");

error[E0433]: failed to resolve: use of undeclared type `Command`
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/maybe-uninit-2.0.0/build.rs:26:9
   |
   |
26 |         Command::new(rustc).arg("--version").output().ok()
   |         ^^^^^^^ use of undeclared type `Command`
error[E0433]: failed to resolve: use of undeclared type `String`
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/maybe-uninit-2.0.0/build.rs:30:9
   |
   |
30 |         String::from_utf8(output.stdout).ok()
   |         ^^^^^^ use of undeclared type `String`
error[E0531]: cannot find tuple struct or tuple variant `Some` in this scope
 --> /cargo/registry/src/github.com-1ecc6299db9ec823/maybe-uninit-2.0.0/build.rs:7:9
  |
  |
7 |         Some(minor) => minor,

error[E0412]: cannot find type `Option` in this scope
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/maybe-uninit-2.0.0/build.rs:22:29
   |
   |
22 | fn rustc_minor_version() -> Option<u32> {
   |                             ^^^^^^ not found in this scope

error[E0531]: cannot find tuple struct or tuple variant `Some` in this scope
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/maybe-uninit-2.0.0/build.rs:33:26
   |
33 |     let version = if let Some(version) = version {


error[E0425]: cannot find value `None` in this scope
   |
36 |         return None;
   |                ^^^^ not found in this scope


error[E0425]: cannot find function, tuple struct or tuple variant `Some` in this scope
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/maybe-uninit-2.0.0/build.rs:40:25
   |
40 |     if pieces.next() != Some("rustc 1") {


error[E0425]: cannot find value `None` in this scope
   |
41 |         return None;
   |                ^^^^ not found in this scope


error[E0531]: cannot find tuple struct or tuple variant `Some` in this scope
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/maybe-uninit-2.0.0/build.rs:45:9
   |
45 |         Some(next) => next,


error: language item required, but not found: `eh_personality`

error: requires `sized` lang_item
error: aborting due to 17 previous errors

Some errors have detailed explanations: E0412, E0425, E0432, E0433, E0531.
For more information about an error, try `rustc --explain E0412`.
