plain

[00:10:00] travis_time:end:236adcfe:start=1533928975467347859,finish=1533929012624743856,duration=37157395997
[CI_JOB_NAME=x86_64-gnu-llvm-5.0]
[00:10:00] [CI_JOB_NAME=x86_64-gnu-llvm-5.0]
[00:10:00] Attempting with retry: curl -sfSO http://security.ubuntu.com/ubuntu/pool/main/b/bind9/dnsutils_9.10.3.dfsg.P4-8ubuntu1.10_amd64.deb
[00:10:01] Attempting with retry: curl -sfSO http://security.ubuntu.com/ubuntu/pool/main/b/bind9/libdns162_9.10.3.dfsg.P4-8ubuntu1.10_amd64.deb
[00:10:02] Attempting with retry: curl -sfSO http://security.ubuntu.com/ubuntu/pool/main/b/bind9/liblwres141_9.10.3.dfsg.P4-8ubuntu1.10_amd64.deb
[00:10:02] Attempting with retry: curl -sfSO http://security.ubuntu.com/ubuntu/pool/main/b/bind9/libbind9-140_9.10.3.dfsg.P4-8ubuntu1.10_amd64.deb
[00:10:02] Attempting with retry: curl -sfSO http://security.ubuntu.com/ubuntu/pool/main/b/bind9/libisc160_9.10.3.dfsg.P4-8ubuntu1.10_amd64.deb
[00:10:03] Attempting with retry: curl -sfSO http://security.ubuntu.com/ubuntu/pool/main/b/bind9/libisccfg140_9.10.3.dfsg.P4-8ubuntu1.10_amd64.deb
[00:10:04] Attempting with retry: curl -sfSO http://security.ubuntu.com/ubuntu/pool/main/g/geoip/libgeoip1_1.6.9-1_amd64.deb
[00:10:04] Attempting with retry: curl -sfSO http://security.ubuntu.com/ubuntu/pool/main/o/openssl/libssl1.0.0_1.0.2g-1ubuntu4.13_amd64.deb
[00:10:05] travis_fold:start:check_dns_9696
Checking DNS for travis-ci/travis-ci#9696! --------------------------
[00:10:05] travis_fold:start:resolv-conf
cat /etc/resolv.conf --------------------------------------------
---
[00:58:14] ....................................................................................................
[00:58:17] ....................................................................................................
[00:58:19] ....................................................................................................
[00:58:22] ....................................................................................................
[00:58:24] .............iiiiiiiii..............................................................................
[00:58:27] ....................................................................................F...............
[00:58:33] ...................i................................................................................
[00:58:36] ..............................i.....................................................................
[00:58:39] ....................................................................................................
[00:58:42] ....................................................................................................
---
[00:58:47] diff of stderr:
[00:58:47] 
[00:58:47] 8   --> $DIR/ex2a-push-one-existing-name-early-bound.rs:17:5
[00:58:47] 9    |
[00:58:47] 10 LL | fn baz<'a, 'b, T>(x: &mut Vec<&'a T>, y: &T)
[00:58:47] -    |                                       - consider changing the type of `y` to `&'a T`
[00:58:47] +    |                                          -- help: add explicit lifetime `'a` to the type of `y`: `&'a T`
[00:58:47] 12 ...
[00:58:47] 13 LL |     x.push(y); //~ ERROR explicit lifetime required
[00:58:47] 14    |     ^^^^^^^^^ lifetime `'a` required
[00:58:47] 
[00:58:47] The actual stderr differed from the expected stderr.
[00:58:47] The actual stderr differed from the expected stderr.
[00:58:47] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetime-errors/ex2a-push-one-existing-name-early-bound.nll/ex2a-push-one-existing-name-early-bound.nll.stderr
[00:58:47] To update references, rerun the tests and pass the `--bless` flag
[00:58:47] To only update this specific test, also pass `--test-args lifetime-errors/ex2a-push-one-existing-name-early-bound.rs`
[00:58:47] error: 1 errors occurred comparing output.
[00:58:47] status: exit code: 1
[00:58:47] status: exit code: 1
[00:58:47] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lifetime-errors/ex2a-push-one-existing-name-early-bound.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetime-errors/ex2a-push-one-existing-name-early-bound.nll/a" "-Zborrowck=mir" "-Ztwo-phase-borrows" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetime-errors/ex2a-push-one-existing-name-early-bound.nll/auxiliary" "-A" "unused"
[00:58:47] ------------------------------------------
[00:58:47] 
[00:58:47] ------------------------------------------
[00:58:47] stderr:
[00:58:47] stderr:
[00:58:47] ------------------------------------------
[00:58:47] {"message":"not reporting region error due to nll","code":null,"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/lifetime-errors/ex2a-push-one-existing-name-early-bound.rs","byte_start":618,"byte_end":619,"line_start":17,"line_end":17,"column_start":12,"column_end":13,"is_primary":true,"text":[{"text":"    x.push(y); //~ ERROR explicit lifetime required","highlight_start":12,"highlight_end":13}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: not reporting region error due to nll\n  --> /checkout/src/test/ui/lifetime-errors/ex2a-push-one-existing-name-early-bound.rs:17:12\n   |\nLL |     x.push(y); //~ ERROR explicit lifetime required\n   |            ^\n\n"}
[00:58:47] {"message":"explicit lifetime required in the type of `y`","code":{"code":"E0621","explanation":"\nThis error code indicates a mismatch between the lifetimes appearing in the\nfunction signature (i.e., the parameter types and the return type) and the\ndata-flow found in the function body.\n\nErroneous code example:\n\n