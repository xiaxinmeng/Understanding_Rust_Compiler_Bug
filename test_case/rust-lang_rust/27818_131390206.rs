
maketest: alloc-extern-crates
maketest: allow-non-lint-warnings-cmdline
maketest: allow-warnings-cmdline-stability
----- /Users/rustbuild/src/rust-buildbot/slave/auto-mac-32-opt/build/src/test/run-make/allow-warnings-cmdline-stability/ --------------------
------ stdout ---------------------------------------------
DYLD_LIBRARY_PATH="/Users/rustbuild/src/rust-buildbot/slave/auto-mac-32-opt/build/obj/i686-apple-darwin/test/run-make/allow-warnings-cmdline-stability:/Users/rustbuild/src/rust-buildbot/slave/auto-mac-32-opt/build/obj/i686-apple-darwin/stage2/lib:" /Users/rustbuild/src/rust-buildbot/slave/auto-mac-32-opt/build/obj/i686-apple-darwin/stage2/bin/rustc --out-dir /Users/rustbuild/src/rust-buildbot/slave/auto-mac-32-opt/build/obj/i686-apple-darwin/test/run-make/allow-warnings-cmdline-stability -L /Users/rustbuild/src/rust-buildbot/slave/auto-mac-32-opt/build/obj/i686-apple-darwin/test/run-make/allow-warnings-cmdline-stability bar.rs

------ stderr ---------------------------------------------
bar.rs:11:1: 16:16 error: non-deprecated unstable items need to point to an issue with `issue = "NNN"`
bar.rs:11 #![crate_type = "lib"]
bar.rs:12 #![feature(staged_api)]
bar.rs:13 #![staged_api]
bar.rs:14 #![unstable(feature = "test_feature")]
bar.rs:15 
bar.rs:16 pub fn baz() { }
error: aborting due to previous error
make[1]: *** [bar] Error 101

------        ---------------------------------------------
