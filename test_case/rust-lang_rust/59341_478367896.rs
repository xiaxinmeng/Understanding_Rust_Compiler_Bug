plain
travis_time:end:0d845600:start=1554057613463363447,finish=1554057614405988571,duration=942625124
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:02:55]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:02:58] error[E0277]: can't compare `std::path::PathBuf` with `&str`
[00:02:58]    --> src/bootstrap/dist.rs:487:32
[00:02:58]     |
[00:02:58] 487 |             if libdir_relative != "bin" {
[00:02:58]     |                                ^^ no implementation for `std::path::PathBuf == &str`
[00:02:58]     |
[00:02:58]     = help: the trait `std::cmp::PartialEq<&str>` is not implemented for `std::path::PathBuf`
[00:02:59] error[E0308]: mismatched types
[00:02:59]    --> src/bootstrap/builder.rs:652:13
[00:02:59]     |
[00:02:59]     |
[00:02:59] 650 |     pub fn libdir_relative(&self, compiler: Compiler) -> PathBuf {
[00:02:59]     |                                                          ------- expected `std::path::PathBuf` because of return type
[00:02:59] 651 |         if compiler.is_snapshot(self) {
[00:02:59] 652 |             libdir(&self.config.build)
[00:02:59]     |             ^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `std::path::PathBuf`, found reference
[00:02:59]     = note: expected type `std::path::PathBuf`
[00:02:59]                found type `&'static str`
[00:02:59] 
[00:02:59] error[E0308]: mismatched types
[00:02:59] error[E0308]: mismatched types
[00:02:59]    --> src/bootstrap/builder.rs:656:24
[00:02:59]     |
[00:02:59] 650 |     pub fn libdir_relative(&self, compiler: Compiler) -> PathBuf {
[00:02:59]     |                                                          ------- expected `std::path::PathBuf` because of return type
[00:02:59] 656 |                     => relative_libdir,
[00:02:59]     |                        ^^^^^^^^^^^^^^^
[00:02:59]     |                        |
[00:02:59]     |                        expected struct `std::path::PathBuf`, found reference
[00:02:59]     |                        expected struct `std::path::PathBuf`, found reference
[00:02:59]     |                        help: try using a conversion method: `relative_libdir.to_path_buf()`
[00:02:59]     = note: expected type `std::path::PathBuf`
[00:02:59]                found type `&std::path::Path`
[00:02:59] 
[00:02:59] error: aborting due to 3 previous errors
---
[00:03:00]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:03:03] error[E0277]: can't compare `std::path::PathBuf` with `&str`
[00:03:03]    --> src/bootstrap/dist.rs:487:32
[00:03:03]     |
[00:03:03] 487 |             if libdir_relative != "bin" {
[00:03:03]     |                                ^^ no implementation for `std::path::PathBuf == &str`
[00:03:03]     |
[00:03:03]     = help: the trait `std::cmp::PartialEq<&str>` is not implemented for `std::path::PathBuf`
[00:03:04] error[E0308]: mismatched types
[00:03:04]    --> src/bootstrap/builder.rs:652:13
[00:03:04]     |
[00:03:04]     |
[00:03:04] 650 |     pub fn libdir_relative(&self, compiler: Compiler) -> PathBuf {
[00:03:04]     |                                                          ------- expected `std::path::PathBuf` because of return type
[00:03:04] 651 |         if compiler.is_snapshot(self) {
[00:03:04] 652 |             libdir(&self.config.build)
[00:03:04]     |             ^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `std::path::PathBuf`, found reference
[00:03:04]     = note: expected type `std::path::PathBuf`
[00:03:04]                found type `&'static str`
[00:03:04] 
[00:03:04] error[E0308]: mismatched types
[00:03:04] error[E0308]: mismatched types
[00:03:04]    --> src/bootstrap/builder.rs:656:24
[00:03:04]     |
[00:03:04] 650 |     pub fn libdir_relative(&self, compiler: Compiler) -> PathBuf {
[00:03:04]     |                                                          ------- expected `std::path::PathBuf` because of return type
[00:03:04] 656 |                     => relative_libdir,
[00:03:04]     |                        ^^^^^^^^^^^^^^^
[00:03:04]     |                        |
[00:03:04]     |                        expected struct `std::path::PathBuf`, found reference
[00:03:04]     |                        expected struct `std::path::PathBuf`, found reference
[00:03:04]     |                        help: try using a conversion method: `relative_libdir.to_path_buf()`
[00:03:04]     = note: expected type `std::path::PathBuf`
[00:03:04]                found type `&std::path::Path`
[00:03:04] 
[00:03:04] error: aborting due to 3 previous errors
---
[00:03:07]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:03:10] error[E0277]: can't compare `std::path::PathBuf` with `&str`
[00:03:10]    --> src/bootstrap/dist.rs:487:32
[00:03:10]     |
[00:03:10] 487 |             if libdir_relative != "bin" {
[00:03:10]     |                                ^^ no implementation for `std::path::PathBuf == &str`
[00:03:10]     |
[00:03:10]     = help: the trait `std::cmp::PartialEq<&str>` is not implemented for `std::path::PathBuf`
[00:03:11] error[E0308]: mismatched types
[00:03:11]    --> src/bootstrap/builder.rs:652:13
[00:03:11]     |
[00:03:11]     |
[00:03:11] 650 |     pub fn libdir_relative(&self, compiler: Compiler) -> PathBuf {
[00:03:11]     |                                                          ------- expected `std::path::PathBuf` because of return type
[00:03:11] 651 |         if compiler.is_snapshot(self) {
[00:03:11] 652 |             libdir(&self.config.build)
[00:03:11]     |             ^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `std::path::PathBuf`, found reference
[00:03:11]     = note: expected type `std::path::PathBuf`
[00:03:11]                found type `&'static str`
[00:03:11] 
[00:03:11] error[E0308]: mismatched types
[00:03:11] error[E0308]: mismatched types
[00:03:11]    --> src/bootstrap/builder.rs:656:24
[00:03:11]     |
[00:03:11] 650 |     pub fn libdir_relative(&self, compiler: Compiler) -> PathBuf {
[00:03:11]     |                                                          ------- expected `std::path::PathBuf` because of return type
[00:03:11] 656 |                     => relative_libdir,
[00:03:11]     |                        ^^^^^^^^^^^^^^^
[00:03:11]     |                        |
[00:03:11]     |                        expected struct `std::path::PathBuf`, found reference
[00:03:11]     |                        expected struct `std::path::PathBuf`, found reference
[00:03:11]     |                        help: try using a conversion method: `relative_libdir.to_path_buf()`
[00:03:11]     = note: expected type `std::path::PathBuf`
[00:03:11]                found type `&std::path::Path`
[00:03:11] 
[00:03:11] error: aborting due to 3 previous errors
---
[00:03:15]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:03:18] error[E0277]: can't compare `std::path::PathBuf` with `&str`
[00:03:18]    --> src/bootstrap/dist.rs:487:32
[00:03:18]     |
[00:03:18] 487 |             if libdir_relative != "bin" {
[00:03:18]     |                                ^^ no implementation for `std::path::PathBuf == &str`
[00:03:18]     |
[00:03:18]     = help: the trait `std::cmp::PartialEq<&str>` is not implemented for `std::path::PathBuf`
[00:03:19] error[E0308]: mismatched types
[00:03:19]    --> src/bootstrap/builder.rs:652:13
[00:03:19]     |
[00:03:19]     |
[00:03:19] 650 |     pub fn libdir_relative(&self, compiler: Compiler) -> PathBuf {
[00:03:19]     |                                                          ------- expected `std::path::PathBuf` because of return type
[00:03:19] 651 |         if compiler.is_snapshot(self) {
[00:03:19] 652 |             libdir(&self.config.build)
[00:03:19]     |             ^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `std::path::PathBuf`, found reference
[00:03:19]     = note: expected type `std::path::PathBuf`
[00:03:19]                found type `&'static str`
[00:03:19] 
[00:03:19] error[E0308]: mismatched types
[00:03:19] error[E0308]: mismatched types
[00:03:19]    --> src/bootstrap/builder.rs:656:24
[00:03:19]     |
[00:03:19] 650 |     pub fn libdir_relative(&self, compiler: Compiler) -> PathBuf {
[00:03:19]     |                                                          ------- expected `std::path::PathBuf` because of return type
[00:03:19] 656 |                     => relative_libdir,
[00:03:19]     |                        ^^^^^^^^^^^^^^^
[00:03:19]     |                        |
[00:03:19]     |                        expected struct `std::path::PathBuf`, found reference
[00:03:19]     |                        expected struct `std::path::PathBuf`, found reference
[00:03:19]     |                        help: try using a conversion method: `relative_libdir.to_path_buf()`
[00:03:19]     = note: expected type `std::path::PathBuf`
[00:03:19]                found type `&std::path::Path`
[00:03:19] 
[00:03:19] error: aborting due to 3 previous errors
---
[00:03:23]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:03:26] error[E0277]: can't compare `std::path::PathBuf` with `&str`
[00:03:26]    --> src/bootstrap/dist.rs:487:32
[00:03:26]     |
[00:03:26] 487 |             if libdir_relative != "bin" {
[00:03:26]     |                                ^^ no implementation for `std::path::PathBuf == &str`
[00:03:26]     |
[00:03:26]     = help: the trait `std::cmp::PartialEq<&str>` is not implemented for `std::path::PathBuf`
[00:03:27] error[E0308]: mismatched types
[00:03:27]    --> src/bootstrap/builder.rs:652:13
[00:03:27]     |
[00:03:27]     |
[00:03:27] 650 |     pub fn libdir_relative(&self, compiler: Compiler) -> PathBuf {
[00:03:27]     |                                                          ------- expected `std::path::PathBuf` because of return type
[00:03:27] 651 |         if compiler.is_snapshot(self) {
[00:03:27] 652 |             libdir(&self.config.build)
[00:03:27]     |             ^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `std::path::PathBuf`, found reference
[00:03:27]     = note: expected type `std::path::PathBuf`
[00:03:27]                found type `&'static str`
[00:03:27] 
[00:03:27] error[E0308]: mismatched types
[00:03:27] error[E0308]: mismatched types
[00:03:27]    --> src/bootstrap/builder.rs:656:24
[00:03:27]     |
[00:03:27] 650 |     pub fn libdir_relative(&self, compiler: Compiler) -> PathBuf {
[00:03:27]     |                                                          ------- expected `std::path::PathBuf` because of return type
[00:03:27] 656 |                     => relative_libdir,
[00:03:27]     |                        ^^^^^^^^^^^^^^^
[00:03:27]     |                        |
[00:03:27]     |                        expected struct `std::path::PathBuf`, found reference
[00:03:27]     |                        expected struct `std::path::PathBuf`, found reference
[00:03:27]     |                        help: try using a conversion method: `relative_libdir.to_path_buf()`
[00:03:27]     = note: expected type `std::path::PathBuf`
[00:03:27]                found type `&std::path::Path`
[00:03:27] 
[00:03:27] error: aborting due to 3 previous errors
