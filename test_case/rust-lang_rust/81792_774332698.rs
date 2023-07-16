plain
 Success!
 

The actual stderr differed from the expected stderr.
Actual stderr saved to /tmp/compiletestY7Q69H/panic/catch_panic.stderr
To update references, run this command from build directory:
tests/run-pass/update-references.sh '/tmp/compiletestY7Q69H' 'panic/catch_panic.rs'
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/panic/catch_panic.rs" "-L" "/tmp/compiletestY7Q69H" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestY7Q69H/panic/catch_panic.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-Zmiri-symbolic-alignment-check" "-L" "/tmp/compiletestY7Q69H/panic/catch_panic.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---

failures:

---- check_that_clippy_has_the_same_major_version_as_rustc stdout ----
thread 'check_that_clippy_has_the_same_major_version_as_rustc' panicked at 'assertion failed: `(left == right)`
  left: `"51"`,
 right: `"52"`: clippy patch version does not equal rustc minor version', src/tools/clippy/tests/versioncheck.rs:61:13


failures:
    check_that_clippy_has_the_same_major_version_as_rustc
