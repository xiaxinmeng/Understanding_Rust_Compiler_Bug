plain
[00:13:20]    Compiling rustc_traits v0.0.0 (file:///checkout/src/librustc_traits)
[00:14:07] error: method is never used: `universal_regions_outlived_by`
[00:14:07]    --> librustc_mir/borrow_check/nll/region_infer/mod.rs:655:5
[00:14:07]     |
[00:14:07] 655 | /     pub fn universal_regions_outlived_by<'a>(
[00:14:07] 656 | |         &'a self,
[00:14:07] 657 | |         r: RegionVid
[00:14:07] 658 | |     ) -> impl Iterator<Item = RegionVid> + 'a {
[00:14:07] 659 | |         let borrow_scc = self.constraint_sccs.scc(r);
[00:14:07] 660 | |         self.scc_values.universal_regions_outlived_by(borrow_scc)
[00:14:07]     | |_____^
[00:14:07]     |
[00:14:07]     = note: `-D dead-code` implied by `-D warnings`
[00:14:07] 
[00:14:07] 
[00:14:07] error: aborting due to previous error
[00:14:07] 
[00:14:07] error: Could not compile `rustc_mir`.
[00:14:07] warning: build failed, waiting for other jobs to finish...
[00:14:51] error: build failed
[00:14:51] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:14:51] expected success, got: exit code: 101
[00:14:51] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1155:9
[00:14:51] travis_fold:end:stage0-rustc

[00:14:51] travis_time:end:stage0-rustc:start=1536934182315591359,finish=1536934734686578937,duration=552370987578


[00:14:51] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:14:51] Build completed unsuccessfully in 0:10:05
[00:14:51] Makefile:28: recipe for target 'all' failed
[00:14:51] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0b3373a6
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
