plain
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
   Compiling cargo_metadata v0.11.1
   Compiling aho-corasick v0.7.13
   Compiling regex v1.4.3
   Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
error[E0658]: use of unstable library feature 'str_split_once': newly added
  --> src/tools/tidy/src/error_codes_check.rs:89:18
   |
89 |                 .split_once(':')
   |
   = note: see issue #74773 <https://github.com/rust-lang/rust/issues/74773> for more information
   = note: see issue #74773 <https://github.com/rust-lang/rust/issues/74773> for more information
   = help: add `#![feature(str_split_once)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'str_split_once': newly added
   --> src/tools/tidy/src/error_codes_check.rs:102:40
    |
102 |             let md_file_name = match s.split_once("include_str!(\"") {
    |
    = note: see issue #74773 <https://github.com/rust-lang/rust/issues/74773> for more information
    = note: see issue #74773 <https://github.com/rust-lang/rust/issues/74773> for more information
    = help: add `#![feature(str_split_once)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'str_split_once': newly added
   --> src/tools/tidy/src/error_codes_check.rs:104:43
    |
104 |                 Some((_, md)) => match md.split_once("\")") {
    |
    = note: see issue #74773 <https://github.com/rust-lang/rust/issues/74773> for more information
    = note: see issue #74773 <https://github.com/rust-lang/rust/issues/74773> for more information
    = help: add `#![feature(str_split_once)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'str_split_once': newly added
   --> src/tools/tidy/src/error_codes_check.rs:135:36
    |
135 |             let err_code = match s.split_once(',') {
    |
    = note: see issue #74773 <https://github.com/rust-lang/rust/issues/74773> for more information
    = note: see issue #74773 <https://github.com/rust-lang/rust/issues/74773> for more information
    = help: add `#![feature(str_split_once)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'str_split_once': newly added
   --> src/tools/tidy/src/error_codes_check.rs:154:36
    |
154 |             let err_code = match s.split_once(']') {
    |
    = note: see issue #74773 <https://github.com/rust-lang/rust/issues/74773> for more information
    = note: see issue #74773 <https://github.com/rust-lang/rust/issues/74773> for more information
    = help: add `#![feature(str_split_once)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'str_split_once': newly added
   --> src/tools/tidy/src/error_codes_check.rs:156:55
    |
156 |                 Some((err_code, _)) => match err_code.split_once('[') {
    |
    = note: see issue #74773 <https://github.com/rust-lang/rust/issues/74773> for more information
    = note: see issue #74773 <https://github.com/rust-lang/rust/issues/74773> for more information
    = help: add `#![feature(str_split_once)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'str_split_once': newly added
  --> src/tools/tidy/src/extdeps.rs:26:27
   |
26 |         let source = line.split_once('=').unwrap().1.trim();
   |
   = note: see issue #74773 <https://github.com/rust-lang/rust/issues/74773> for more information
   = note: see issue #74773 <https://github.com/rust-lang/rust/issues/74773> for more information
   = help: add `#![feature(str_split_once)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'str_split_once': newly added
  --> src/tools/tidy/src/ui_tests.rs:68:74
   |
68 |                         file_path.file_name().unwrap().to_str().unwrap().split_once('.').unwrap().0;
   |
   = note: see issue #74773 <https://github.com/rust-lang/rust/issues/74773> for more information
   = note: see issue #74773 <https://github.com/rust-lang/rust/issues/74773> for more information
   = help: add `#![feature(str_split_once)]` to the crate attributes to enable
error: aborting due to 8 previous errors

For more information about this error, try `rustc --explain E0658`.
error: could not compile `tidy`
