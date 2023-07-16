
----- /Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-nopt-t/build/src/test/run-make/error-found-staticlib-instead-crate/ --------------------
------ stdout ---------------------------------------------
DYLD_LIBRARY_PATH="/Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-nopt-t/build/obj/x86_64-apple-darwin/test/run-make/error-found-staticlib-instead-crate:/Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-nopt-t/build/obj/x86_64-apple-darwin/stage2/lib:" /Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-nopt-t/build/obj/x86_64-apple-darwin/stage2/bin/rustc --out-dir /Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-nopt-t/build/obj/x86_64-apple-darwin/test/run-make/error-found-staticlib-instead-crate -L /Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-nopt-t/build/obj/x86_64-apple-darwin/test/run-make/error-found-staticlib-instead-crate foo.rs --crate-type staticlib

------ stderr ---------------------------------------------
error: "ar" '"x"' '"/Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-nopt-t/build/obj/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libstd-4e7c5e5c.rlib"' failed with: exit code: 1
note: stdout ---

note: stderr ---
ar: rust.metadata.bin: No space left on device

error: aborting due to previous error
make[1]: *** [all] Error 101
