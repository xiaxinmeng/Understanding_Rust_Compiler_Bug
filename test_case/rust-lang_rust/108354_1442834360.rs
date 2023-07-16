plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:697bea7ddceb6696743da8f159f268aef8bfb3c6)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---
doc tests for: /checkout/src/doc/rustc/src/platform-support/armv7-unknown-linux-uclibceabihf.md
doc tests for: /checkout/src/doc/rustc/src/platform-support/fuchsia.md


command did not execute successfully: CFG_RELEASE_CHANNEL="nightly" RUSTC_BOOTSTRAP="1" RUSTC_STAGE="2" RUSTC_SYSROOT="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" RUSTDOC_LIBDIR="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" RUSTDOC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" RUST_TEST_THREADS="16" "/checkout/obj/build/bootstrap/debug/rustdoc" "-Wrustdoc::invalid_codeblock_attributes" "-Dwarnings" "-Znormalize-docs" "-Z" "unstable-options" "--test" "/checkout/src/doc/rustc/src/platform-support/fuchsia.md" "--test-args" ""

stdout ----

running 3 tests
---
  |
2 | use std::env::args;
  |

error[E0425]: cannot find value `Cpanic` in this scope
 --> /checkout/src/doc/rustc/src/platform-support/fuchsia.md:741:18
4 |     --test-args -Cpanic=abort                                                 \
  |                  ^^^^^^ not found in this scope

error[E0425]: cannot find value `abort` in this scope
---
  |
2 | use std::env::args;
  |

error[E0425]: cannot find value `Zpanic_abort_tests` in this scope
 --> /checkout/src/doc/rustc/src/platform-support/fuchsia.md:743:18
6 |     --test-args -Zpanic_abort_tests                                           \
  |                  ^^^^^^^^^^^^^^^^^^ not found in this scope


error[E0070]: invalid left-hand side of assignment
 --> /checkout/src/doc/rustc/src/platform-support/fuchsia.md:741:24
  |
3 | / --test-args --target-rustcflags                                           \
4 | |     --test-args -Cpanic=abort                                                 \
  | |                       -^
  |                         cannot assign to this expression

error: aborting due to 20 previous errors

