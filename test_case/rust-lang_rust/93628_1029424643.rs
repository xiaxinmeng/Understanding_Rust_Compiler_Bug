plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 4e8fb743ccbec27344b2dd42de7057f41d4ebfdd and 24e178481e9f3e5dbd1ffba37e9a302e2386197c
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
   Compiling idna v0.2.0
error: the feature `let_else` has been stable since 1.60.0 and no longer requires an attribute to enable
 --> src/tools/clippy/clippy_utils/src/lib.rs:3:12
  |
3 | #![feature(let_else)]
  |
  |
  = note: `-D stable-features` implied by `-D warnings`
error: could not compile `clippy_utils` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
Building stage2 tool rls (x86_64-unknown-linux-gnu)
---
   Compiling strip-ansi-escapes v0.1.0
warning: the feature `let_else` has been stable since 1.60.0 and no longer requires an attribute to enable
 --> src/tools/clippy/clippy_utils/src/lib.rs:3:12
  |
3 | #![feature(let_else)]
  |
  = note: `#[warn(stable_features)]` on by default

   Compiling aho-corasick v0.7.18
---
   Compiling cargo v0.61.0 (/checkout/src/tools/cargo)
warning: the feature `let_else` has been stable since 1.60.0 and no longer requires an attribute to enable
 --> src/tools/clippy/clippy_lints/src/lib.rs:8:12
  |
8 | #![feature(let_else)]
  |
  = note: `#[warn(stable_features)]` on by default

   Compiling rls-rustc v0.6.0 (/checkout/src/tools/rls/rls-rustc)
---
-hello dup fd
-

The actual stdout differed from the expected stdout.
Actual stdout saved to /tmp/compiletestHOBcQy/fs.stage-id.stdout

-hello dup fd
+error: unsupported operation: can't call foreign function: readdir64
+   --> /checkout/library/std/src/sys/unix/fs.rs:482:33
---
+
 

The actual stderr differed from the expected stderr.
Actual stderr saved to /tmp/compiletestHOBcQy/fs.stage-id.stderr
To only update this specific test, also pass `--test-args fs.rs`

error: 2 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/fs.rs" "-L" "/tmp/compiletestHOBcQy" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestHOBcQy/fs.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-Zmiri-disable-isolation" "-L" "/tmp/compiletestHOBcQy/fs.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---



The actual stderr differed from the expected stderr.
Actual stderr saved to /tmp/compiletestHOBcQy/portable-simd.stage-id.stderr
To only update this specific test, also pass `--test-args portable-simd.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/portable-simd.rs" "-L" "/tmp/compiletestHOBcQy" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestHOBcQy/portable-simd.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletestHOBcQy/portable-simd.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---
Building stage2 tool clippy-driver (x86_64-unknown-linux-gnu)
warning: the feature `let_else` has been stable since 1.60.0 and no longer requires an attribute to enable
 --> src/tools/clippy/clippy_utils/src/lib.rs:3:12
  |
3 | #![feature(let_else)]
  |
  = note: `#[warn(stable_features)]` on by default

warning: `clippy_utils` (lib) generated 1 warning
warning: `clippy_utils` (lib) generated 1 warning
   Compiling cargo_metadata v0.14.0
   Compiling clippy_lints v0.1.60 (/checkout/src/tools/clippy/clippy_lints)
error: the feature `let_else` has been stable since 1.60.0 and no longer requires an attribute to enable
 --> src/tools/clippy/clippy_lints/src/lib.rs:8:12
  |
8 | #![feature(let_else)]
  |
  |
  = note: `-D stable-features` implied by `-D warnings`
error: could not compile `clippy_lints` due to previous error
error: could not compile `clippy_lints` due to previous error
thread 'main' panicked at 'in-tree tool', src/bootstrap/test.rs:658:14
Build completed unsuccessfully in 0:00:16
