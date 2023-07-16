 bash
maketest: no-stack-check
----- /Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-nopt-t/build/src/test/run-make/no-stack-check/ --------------------
------ stdout ---------------------------------------------
DYLD_LIBRARY_PATH="/Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-nopt-t/build/obj/x86_64-apple-darwin/test/run-make/no-stack-check:/Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-nopt-t/build/obj/x86_64-apple-darwin/stage2/lib:" /Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-nopt-t/build/obj/x86_64-apple-darwin/stage2/bin/rustc --out-dir /Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-nopt-t/build/obj/x86_64-apple-darwin/test/run-make/no-stack-check -L /Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-nopt-t/build/obj/x86_64-apple-darwin/test/run-make/no-stack-check --emit asm attr.rs
! grep -q morestack /Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-nopt-t/build/obj/x86_64-apple-darwin/test/run-make/no-stack-check/attr.s

------ stderr ---------------------------------------------
make[1]: *** [all] Error 1
