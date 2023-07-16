plain
running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo build`
thread 'build::cargo_compile_with_warnings_in_the_root_package' panicked at '
test failed running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo build`
error: expected to find:
[..]function is never used: `dead`[..]
did not find in output:
did not find in output:
warning: path `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t259/foo/src/foo.rs` was erroneously implicitly accepted for binary `foo`,
please set bin.path in Cargo.toml
   Compiling foo v0.5.0 (/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t259/foo)
warning: function `dead` is never used
  |
  |
1 | fn main() {} fn dead() {}
  |
  = note: `#[warn(dead_code)]` on by default

warning: `foo` (bin "foo") generated 1 warning
---
running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo build`
thread 'build::cargo_compile_with_warnings_in_a_dep_package' panicked at '
test failed running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo build`
error: expected to find:
[..]function is never used: `dead`[..]
did not find in output:
did not find in output:
warning: path `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t258/foo/src/foo.rs` was erroneously implicitly accepted for binary `foo`,
please set bin.path in Cargo.toml
   Compiling bar v0.5.0 (/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t258/foo/bar)
warning: function `dead` is never used
 --> bar/src/bar.rs:6:20
6 |                 fn dead() {}
  |                    ^^^^
  |
  = note: `#[warn(dead_code)]` on by default
---
running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo build -vv`
thread 'registry::upstream_warnings_on_extra_verbose_git' panicked at '
test failed running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo build -vv`
error: expected to find:
[..]warning: function is never used[..]
did not find in output:
did not find in output:
    Updating `dummy-registry` index
did not find in output:
    Updating `dummy-registry` index
 Downloading crates ...
  Downloaded foo v0.1.0 (registry `dummy-registry`)
   Compiling foo v0.1.0
     Running `CARGO=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo CARGO_CRATE_NAME=foo CARGO_MANIFEST_DIR=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t2035/home/.cargo/registry/src/-72085e400083f6a9/foo-0.1.0 CARGO_PKG_AUTHORS='' CARGO_PKG_DESCRIPTION='' CARGO_PKG_HOMEPAGE='' CARGO_PKG_LICENSE='' CARGO_PKG_LICENSE_FILE='' CARGO_PKG_NAME=foo CARGO_PKG_REPOSITORY='' CARGO_PKG_RUST_VERSION='' CARGO_PKG_VERSION=0.1.0 CARGO_PKG_VERSION_MAJOR=0 CARGO_PKG_VERSION_MINOR=1 CARGO_PKG_VERSION_PATCH=0 CARGO_PKG_VERSION_PRE='' LD_LIBRARY_PATH='/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t2035/foo/target/debug/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/curl-sys-2059a1860c7139e1/out/build:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/libgit2-sys-16e516b85982c770/out/build:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/libnghttp2-sys-cafd2a451efa3249/out/i/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/libssh2-sys-dda961c5adeb5274/out/build:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/libz-sys-980f51235e5a31a3/out/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/openssl-sys-4f3bfc27a3c188fc/out/openssl-build/install/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib' rustc --crate-name foo /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t2035/home/.cargo/registry/src/-72085e400083f6a9/foo-0.1.0/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata,link -C embed-bitcode=no -C debuginfo=2 -C metadata=9cfd601f77201324 -C extra-filename=-9cfd601f77201324 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t2035/foo/target/debug/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t2035/foo/target/debug/deps --cap-lints warn`
warning: function `unused` is never used
 --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t2035/home/.cargo/registry/src/-72085e400083f6a9/foo-0.1.0/src/lib.rs:1:4
1 | fn unused() {}
  |    ^^^^^^
  |
  = note: `#[warn(dead_code)]` on by default
  = note: `#[warn(dead_code)]` on by default

warning: `foo` (lib) generated 1 warning
   Compiling bar v0.5.0 (/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t2035/foo)
     Running `CARGO=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo CARGO_BIN_NAME=bar CARGO_CRATE_NAME=bar CARGO_MANIFEST_DIR=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t2035/foo CARGO_PKG_AUTHORS='' CARGO_PKG_DESCRIPTION='' CARGO_PKG_HOMEPAGE='' CARGO_PKG_LICENSE='' CARGO_PKG_LICENSE_FILE='' CARGO_PKG_NAME=bar CARGO_PKG_REPOSITORY='' CARGO_PKG_RUST_VERSION='' CARGO_PKG_VERSION=0.5.0 CARGO_PKG_VERSION_MAJOR=0 CARGO_PKG_VERSION_MINOR=5 CARGO_PKG_VERSION_PATCH=0 CARGO_PKG_VERSION_PRE='' CARGO_PRIMARY_PACKAGE=1 LD_LIBRARY_PATH='/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t2035/foo/target/debug/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/curl-sys-2059a1860c7139e1/out/build:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/libgit2-sys-16e516b85982c770/out/build:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/libnghttp2-sys-cafd2a451efa3249/out/i/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/libssh2-sys-dda961c5adeb5274/out/build:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/libz-sys-980f51235e5a31a3/out/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/openssl-sys-4f3bfc27a3c188fc/out/openssl-build/install/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib' rustc --crate-name bar src/main.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type bin --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 -C metadata=6a81d9018bf344d0 -C extra-filename=-6a81d9018bf344d0 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t2035/foo/target/debug/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t2035/foo/target/debug/deps --extern foo=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t2035/foo/target/debug/deps/libfoo-9cfd601f77201324.rlib`
', src/tools/cargo/tests/testsuite/registry.rs:1965:10

---- registry::upstream_warnings_on_extra_verbose_http stdout ----
---- registry::upstream_warnings_on_extra_verbose_http stdout ----
running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo build -vv -Zhttp-registry`
thread 'registry::upstream_warnings_on_extra_verbose_http' panicked at '
test failed running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo build -vv -Zhttp-registry`
error: expected to find:
[..]warning: function is never used[..]
did not find in output:
did not find in output:
    Updating `dummy-registry` index
did not find in output:
    Updating `dummy-registry` index
 Downloading crates ...
  Downloaded foo v0.1.0 (registry `dummy-registry`)
   Compiling foo v0.1.0
     Running `CARGO=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo CARGO_CRATE_NAME=foo CARGO_MANIFEST_DIR=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t2036/home/.cargo/registry/src/127.0.0.1-969f5e0e41639ed7/foo-0.1.0 CARGO_PKG_AUTHORS='' CARGO_PKG_DESCRIPTION='' CARGO_PKG_HOMEPAGE='' CARGO_PKG_LICENSE='' CARGO_PKG_LICENSE_FILE='' CARGO_PKG_NAME=foo CARGO_PKG_REPOSITORY='' CARGO_PKG_RUST_VERSION='' CARGO_PKG_VERSION=0.1.0 CARGO_PKG_VERSION_MAJOR=0 CARGO_PKG_VERSION_MINOR=1 CARGO_PKG_VERSION_PATCH=0 CARGO_PKG_VERSION_PRE='' LD_LIBRARY_PATH='/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t2036/foo/target/debug/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/curl-sys-2059a1860c7139e1/out/build:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/libgit2-sys-16e516b85982c770/out/build:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/libnghttp2-sys-cafd2a451efa3249/out/i/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/libssh2-sys-dda961c5adeb5274/out/build:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/libz-sys-980f51235e5a31a3/out/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/openssl-sys-4f3bfc27a3c188fc/out/openssl-build/install/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib' rustc --crate-name foo /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t2036/home/.cargo/registry/src/127.0.0.1-969f5e0e41639ed7/foo-0.1.0/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata,link -C embed-bitcode=no -C debuginfo=2 -C metadata=9cfd601f77201324 -C extra-filename=-9cfd601f77201324 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t2036/foo/target/debug/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t2036/foo/target/debug/deps --cap-lints warn`
warning: function `unused` is never used
 --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t2036/home/.cargo/registry/src/127.0.0.1-969f5e0e41639ed7/foo-0.1.0/src/lib.rs:1:4
1 | fn unused() {}
  |    ^^^^^^
  |
  = note: `#[warn(dead_code)]` on by default
  = note: `#[warn(dead_code)]` on by default

warning: `foo` (lib) generated 1 warning
   Compiling bar v0.5.0 (/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t2036/foo)
     Running `CARGO=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo CARGO_BIN_NAME=bar CARGO_CRATE_NAME=bar CARGO_MANIFEST_DIR=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t2036/foo CARGO_PKG_AUTHORS='' CARGO_PKG_DESCRIPTION='' CARGO_PKG_HOMEPAGE='' CARGO_PKG_LICENSE='' CARGO_PKG_LICENSE_FILE='' CARGO_PKG_NAME=bar CARGO_PKG_REPOSITORY='' CARGO_PKG_RUST_VERSION='' CARGO_PKG_VERSION=0.5.0 CARGO_PKG_VERSION_MAJOR=0 CARGO_PKG_VERSION_MINOR=5 CARGO_PKG_VERSION_PATCH=0 CARGO_PKG_VERSION_PRE='' CARGO_PRIMARY_PACKAGE=1 LD_LIBRARY_PATH='/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t2036/foo/target/debug/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/curl-sys-2059a1860c7139e1/out/build:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/libgit2-sys-16e516b85982c770/out/build:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/libnghttp2-sys-cafd2a451efa3249/out/i/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/libssh2-sys-dda961c5adeb5274/out/build:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/libz-sys-980f51235e5a31a3/out/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/openssl-sys-4f3bfc27a3c188fc/out/openssl-build/install/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib' rustc --crate-name bar src/main.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type bin --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 -C metadata=6a81d9018bf344d0 -C extra-filename=-6a81d9018bf344d0 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t2036/foo/target/debug/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t2036/foo/target/debug/deps --extern foo=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t2036/foo/target/debug/deps/libfoo-9cfd601f77201324.rlib`
', src/tools/cargo/tests/testsuite/registry.rs:1965:10


failures:
---
test result: FAILED. 2578 passed; 4 failed; 7 ignored; 0 measured; 0 filtered out; finished in 123.52s

error: test failed, to rerun pass '--test testsuite'
Build completed unsuccessfully in 0:26:10
Makefile:44: recipe for target 'check-aux' failed
make: *** [check-aux] Error 1
