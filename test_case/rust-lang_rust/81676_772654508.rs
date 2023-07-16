plain
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  EXTRA_VARIABLES: {
 "CI_ONLY_WHEN_SUBMODULES_CHANGED": 1
##[endgroup]
adding extra environment variable CI_ONLY_WHEN_SUBMODULES_CHANGED
linux builder detected, using docker to run the build
##[group]Run src/ci/scripts/should-skip-this.sh
---
+For more information about this error, try `rustc --explain E0433`.
+

The actual stderr differed from the expected stderr.
Actual stderr saved to /tmp/compiletest7Z0LAE/packed_struct.stderr
To update references, run this command from build directory:
tests/run-pass/update-references.sh '/tmp/compiletest7Z0LAE' 'packed_struct.rs'
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/packed_struct.rs" "-L" "/tmp/compiletest7Z0LAE" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest7Z0LAE/packed_struct.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletest7Z0LAE/packed_struct.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---
+For more information about this error, try `rustc --explain E0433`.
 

The actual stderr differed from the expected stderr.
Actual stderr saved to /tmp/compiletest7Z0LAE/stacked-borrows/stacked-borrows.stderr
To update references, run this command from build directory:
tests/run-pass/update-references.sh '/tmp/compiletest7Z0LAE' 'stacked-borrows/stacked-borrows.rs'
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/stacked-borrows/stacked-borrows.rs" "-L" "/tmp/compiletest7Z0LAE" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest7Z0LAE/stacked-borrows/stacked-borrows.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-Zmiri-track-raw-pointers" "-L" "/tmp/compiletest7Z0LAE/stacked-borrows/stacked-borrows.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---
Verifying status of edition-guide...
Verifying status of rls...
Verifying status of rustfmt...
Verifying status of miri...
{"rust-by-example":"test-pass","embedded-book":"test-pass","book":"test-pass","rustfmt":"build-fail","cargo-miri":"test-fail","edition-guide":"test-pass","miri":"test-fail","rls":"build-fail","reference":"test-pass","rustbook":"test-fail","nomicon":"test-pass"}failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 check-tools
This PR updated 'src/tools/miri', verifying if status is 'test-pass'...


We detected that this PR updated 'miri', but its tests failed.

If you do intend to update 'miri', please check the error messages above and
commit another update.

If you do NOT intend to update 'miri', please ensure you did not accidentally
change the submodule at 'src/tools/miri'. You may ask your reviewer for the
proper steps.
