plain
tidy check
[00:05:20] * 540 error codes
[00:05:20] * highest error code: E0911
[00:05:21] * 200 features
[00:05:21] Empty file with UI testing output: "/checkout/src/test/ui/reachable/expr_oror.stderr"
[00:05:21] Empty file with UI testing output: "/checkout/src/test/ui/reachable/expr_andand.stderr"
[00:05:21] Empty file with UI testing output: "/checkout/src/test/ui/feature-gate-const-indexing.stderr"
[00:05:21] Empty file with UI testing output: "/checkout/src/test/ui/const-eval/const_transmute.stderr"
[00:05:21] Empty file with UI testing output: "/checkout/src/test/ui/const-pattern-not-const-evaluable.stderr"
[00:05:21] Empty file with UI testing output: "/checkout/src/test/ui/union/union-const-eval.stderr"
[00:05:21] Empty file with UI testing output: "/checkout/src/test/ui/nll/maybe-initialized-drop-uninitialized.stderr"
[00:05:21] Empty file with UI testing output: "/checkout/src/test/ui/nll/drop-may-dangle.stderr"
[00:05:21] Empty file with UI testing output: "/checkout/src/test/ui/nll/ty-outlives/ty-param-implied-bounds.stderr"
[00:05:21] Empty file with UI testing output: "/checkout/src/test/ui/print_type_sizes/anonymous.stderr"
[00:05:21] Empty file with UI testing output: "/checkout/src/test/ui/print_type_sizes/anonymous.stdout"
[00:05:21] Empty file with UI testing output: "/checkout/src/test/ui/print_type_sizes/repr-align.stderr"
[00:05:21] Empty file with UI testing output: "/checkout/src/test/ui/print_type_sizes/multiple_types.stderr"
[00:05:21] Empty file with UI testing output: "/checkout/src/test/ui/print_type_sizes/packed.stderr"
[00:05:21] Empty file with UI testing output: "/checkout/src/test/ui/issue-38875/issue_38875.stderr"
[00:05:21] Empty file with UI testing output: "/checkout/src/test/ui/rfc1598-generic-associated-types/shadowing.stdout"
[00:05:21] Empty file with UI testing output: "/checkout/src/test/ui/rfc1598-generic-associated-types/generic-associated-types-where.stderr"
[00:05:21] Empty file with UI testing output: "/checkout/src/test/ui/compare-method/region-extra.stdout"
[00:05:21] Empty file with UI testing output: "/checkout/src/test/ui/compare-method/region-unrelated.stdout"
[00:05:21] Empty file with UI testing output: "/checkout/src/test/ui/compare-method/proj-outlives-region.stdout"
[00:05:21] Empty file with UI testing output: "/checkout/src/test/ui/const-expr-addr-operator.stderr"
[00:05:21] some tidy checks failed
[00:05:21] 
[00:05:21] 
[00:05:21] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:21] 
[00:05:21] 
[00:05:21] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:21] Build completed unsuccessfully in 0:02:08
[00:05:21] Build completed unsuccessfully in 0:02:08
[00:05:21] Makefile:79: recipe for target 'tidy' failed
[00:05:21] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0fe93180
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
