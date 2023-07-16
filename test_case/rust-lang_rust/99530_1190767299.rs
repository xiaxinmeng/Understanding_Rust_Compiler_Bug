plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between a7468c60f8dbf5feb23ad840b174d7e57113a846 and 5c3bdd9c77a295f0ab17326ba4bbce1836d11315
Submodules were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
         Compiling git2 v0.14.2
         Compiling rustc-workspace-hack v1.0.0 (/checkout/src/tools/rustc-workspace-hack)
         Compiling cargo-miri v0.1.0 (/checkout/src/tools/miri/cargo-miri)
          Finished dev [unoptimized + debuginfo] target(s) in 33.56s
           Running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/debug/cargo-miri miri run --manifest-path test_dependencies/Cargo.toml --target-dir=target/test_dependencies --message-format=json -Zunstable-options`
      error: could not execute process `rustc -vV` (never executed)
      Caused by:
        No such file or directory (os error 2)



      stdout:

Backtrace omitted. Run with RUST_BACKTRACE=1 environment variable to display it.
Run with RUST_BACKTRACE=full to include source snippets.

2 command(s) did not execute successfully:

  - "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rustbook" "test" "/checkout/src/doc/reference"
  - "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rustbook" "test" "/checkout/src/doc/reference"

  - "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zcheck-cfg=names,values,output,features" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/miri/Cargo.toml" "--features" "rustc-workspace-hack/all-static" "--"
Build completed unsuccessfully in 0:24:00
{"reference":"test-fail","book":"test-pass","miri":"test-fail","cargo-miri":"test-fail","edition-guide":"test-pass","nomicon":"test-pass","rls":"test-pass","embedded-book":"test-pass","rustbook":"test-fail","rust-by-example":"test-pass"}Building rustbuild
    Finished dev [unoptimized] target(s) in 0.06s
Verifying status of book...
---
This PR updated 'src/tools/miri', verifying if status is 'test-pass'...

We detected that this PR updated 'miri', but its tests failed.

If you do intend to update 'miri', please check the error messages above and
commit another update.

If you do NOT intend to update 'miri', please ensure you did not accidentally
change the submodule at 'src/tools/miri'. You may ask your reviewer for the
Build completed unsuccessfully in 0:00:00
