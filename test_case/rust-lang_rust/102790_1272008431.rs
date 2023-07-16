plain

---- builder::tests::defaults::build_cross_compile stdout ----


failed to execute command: "/checkout/obj/build/bootstrap/debug/build/bootstrap-64ede6da6945bbcf/out/tmp-rustbuild-tests/main/A/llvm/build/bin/llvm-config" "--bindir"
error: test failed, to rerun pass `--lib`
error: No such file or directory (os error 2)

thread 'main' panicked at 'status code: 1', lib.rs:1653:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


---- builder::tests::dist::build_all stdout ----


failed to execute command: "/checkout/obj/build/bootstrap/debug/build/bootstrap-64ede6da6945bbcf/out/tmp-rustbuild-tests/main/A/llvm/build/bin/llvm-config" "--bindir"
error: No such file or directory (os error 2)

thread 'main' panicked at 'status code: 1', lib.rs:1653:9

---- builder::tests::dist::dist_only_cross_host stdout ----
---- builder::tests::dist::dist_only_cross_host stdout ----


failed to execute command: "/checkout/obj/build/bootstrap/debug/build/bootstrap-64ede6da6945bbcf/out/tmp-rustbuild-tests/main/A/llvm/build/bin/llvm-config" "--bindir"
error: No such file or directory (os error 2)

thread 'main' panicked at 'status code: 1', lib.rs:1653:9

---- builder::tests::dist::dist_with_hosts stdout ----
---- builder::tests::dist::dist_with_hosts stdout ----


failed to execute command: "/checkout/obj/build/bootstrap/debug/build/bootstrap-64ede6da6945bbcf/out/tmp-rustbuild-tests/main/A/llvm/build/bin/llvm-config" "--bindir"
error: No such file or directory (os error 2)

thread 'main' panicked at 'status code: 1', lib.rs:1653:9

---- builder::tests::dist::dist_with_same_targets_and_hosts stdout ----
---- builder::tests::dist::dist_with_same_targets_and_hosts stdout ----


failed to execute command: "/checkout/obj/build/bootstrap/debug/build/bootstrap-64ede6da6945bbcf/out/tmp-rustbuild-tests/main/A/llvm/build/bin/llvm-config" "--bindir"
error: No such file or directory (os error 2)

thread 'main' panicked at 'status code: 1', lib.rs:1653:9

---- builder::tests::dist::dist_with_targets_and_hosts stdout ----
---- builder::tests::dist::dist_with_targets_and_hosts stdout ----


failed to execute command: "/checkout/obj/build/bootstrap/debug/build/bootstrap-64ede6da6945bbcf/out/tmp-rustbuild-tests/main/A/llvm/build/bin/llvm-config" "--bindir"
error: No such file or directory (os error 2)

thread 'main' panicked at 'status code: 1', lib.rs:1653:9


