plain
[01:47:38] test build_script::adding_an_override_invalidates ... ok
[01:47:38] test build_script::assume_build_script_when_build_rs_present ... ok
[01:47:38] test build_script::build_deps_not_for_normal ... ok
[01:47:38] test build_script::build_script_only ... ok
[01:47:38] test build_script::build_script_scan_eacces ... ok
[01:47:39] test build_script::build_deps_simple ... ok
[01:47:39] test build_script::build_script_with_lto ... ok
[01:47:40] test build_script::cfg_env_vars_available ... ok
[01:47:40] test build_script::cfg_feedback ... ok
---
[01:49:40] test install::uninstall_cwd ... ok
[01:49:40] test install::uninstall_piecemeal ... ok
[01:49:40] test install::uninstall_multiple_and_some_pkg_does_not_exist ... ok
[01:49:41] test install::vers_precise ... ok
[01:49:41] test install_upgrade::ambiguous_version_no_longer_allowed ... ok
[01:49:41] test install::workspace_uses_workspace_target_dir ... ok
[01:49:41] test install::version_too ... ok
[01:49:41] test install::version_too ... ok
[01:49:41] test install_upgrade::change_target_rebuilds ... ok
[01:49:42] test install_upgrade::fails_for_conflicts_known ... ok
[01:49:42] test install_upgrade::fails_for_conflicts_unknown ... ok
[01:49:42] test install_upgrade::change_profile_rebuilds ... ok
[01:49:43] test install_upgrade::change_bin_sets_rebuilds ... ok
[01:49:43] test install_upgrade::change_features_rebuilds ... ok
[01:49:43] test install_upgrade::no_track_gated ... ok
[01:49:43] test install_upgrade::no_track ... ok
[01:49:43] test install_upgrade::forwards_compatible ... ok
[01:49:44] test install_upgrade::path_is_always_dirty ... ok
[01:49:46] test install_upgrade::supports_multiple_binary_names ... ok
[01:49:46] test install_upgrade::uninstall ... ok
[01:49:46] test install_upgrade::registry_upgrade ... ok
[01:49:47] test install_upgrade::switch_sources ... ok
[01:49:47] test install_upgrade::multiple_report ... ok
[01:49:48] test install_upgrade::upgrade_force ... ok
[01:49:48] test install_upgrade::v1_already_installed_fresh ... ok
[01:49:48] test install_upgrade::upgrade_git ... ok
[01:49:48] test jobserver::jobserver_and_j ... ok
[01:49:48] test install_upgrade::v1_already_installed_dirty ... ok
[01:49:49] test list_targets::build_list_targets ... ok
[01:49:49] test list_targets::check_list_targets ... ok
[01:49:49] test list_targets::doc_list_targets ... ok
[01:49:49] test jobserver::jobserver_exists ... ok
---
[01:49:52] test local_registry::path_dep_rewritten ... ok
[01:49:52] test lockfile_compat::current_lockfile_format ... ok
[01:49:52] test lockfile_compat::listed_checksum_bad_if_we_cannot_compute ... ok
[01:49:52] test lockfile_compat::locked_correct_error ... ok
[01:49:52] test install_upgrade::v2_syncs ... ok
[01:49:52] test local_registry::simple ... ok
[01:49:52] test lockfile_compat::unlisted_checksum_is_bad_if_we_calculate ... ok
[01:49:52] test lockfile_compat::wrong_checksum_is_an_error ... ok
[01:49:52] test login::login_with_new_credentials ... ok
---
[01:51:52] test workspaces::ws_warn_unused ... ok
[01:51:52] 
[01:51:52] failures:
[01:51:52] 
[01:51:52] ---- workspaces::new_warning_with_corrupt_ws stdout ----
[01:51:52] running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo new bar`
[01:51:52] thread 'workspaces::new_warning_with_corrupt_ws' panicked at '
[01:51:52]     but: exited with exit code: 101
[01:51:52] --- stdout
[01:51:52] 
[01:51:52] --- stderr
[01:51:52] --- stderr
[01:51:52] error: Failed to create package `bar` at `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t1467/foo/bar`
[01:51:52] Caused by:
[01:51:52] Caused by:
[01:51:52]   could not determine the current user, please set $USER
[01:51:52] 
[01:51:52] 
[01:51:52] failures:
[01:51:52]     workspaces::new_warning_with_corrupt_ws
---
[01:51:52] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/cargo/Cargo.toml" "--features" "rustc-workspace-hack/all-static"
[01:51:52] expected success, got: exit code: 101
[01:51:52] 
[01:51:52] 
[01:51:52] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/test/run-pass/pretty src/test/run-fail/pretty src/test/run-pass-valgrind/pretty src/test/run-pass-fulldeps/pretty src/tools/cargo src/tools/cargotest
[01:51:52] Build completed unsuccessfully in 0:31:24
[01:51:52] Makefile:50: recipe for target 'check-aux' failed
[01:51:52] make: *** [check-aux] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0f527088
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Apr 16 10:16:51 UTC 2019
---
travis_time:end:04156d88:start=1555409814747787247,finish=1555409814754448793,duration=6661546
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1d0e2a5e
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:start:crashlog
obj/cores/core.8919.!checkout!obj!build!x86_64-unknown-linux-gnu!stage2!bin!rustc
[New LWP 8924]
[New LWP 8919]
warning: Could not load shared library symbols for 8 libraries, e.g. /lib/x86_64-linux-gnu/libc.so.6.
Use the "info sharedlibrary" command to see the complete listing.
Do you need "set solib-search-path" or "set sysroot"?
Core was generated by `rustc --crate-name foo src/lib.rs --color never --crate-type lib --emit=dep-inf'.
Program terminated with signal SIGABRT, Aborted.
#0  0x00007f04bc7ae428 in ?? ()
[Current thread is 1 (LWP 8924)]
#0  0x00007f04bc7ae428 in ?? ()
#1  0x00007f04bc7b002a in ?? ()
#2  0x0000000000000020 in ?? ()
#3  0x0000000000000000 in ?? ()
travis_time:end:1d0e2a5e:start=1555409814759952918,finish=1555409816714895817,duration=1954942899
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:09c951d8
travis_time:start:09c951d8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1226f59f
$ dmesg | grep -i kill
