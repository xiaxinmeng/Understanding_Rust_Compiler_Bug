plain

[00:01:02] travis_time:end:2ad5efeb:start=1533918863708771880,finish=1533918903079288630,duration=39370516750
[CI_JOB_NAME=x86_64-gnu-llvm-5.0]
[00:01:02] [CI_JOB_NAME=x86_64-gnu-llvm-5.0]
[00:01:02] Attempting with retry: curl -sfSO http://security.ubuntu.com/ubuntu/pool/main/b/bind9/dnsutils_9.10.3.dfsg.P4-8ubuntu1.10_amd64.deb
[00:01:03] Attempting with retry: curl -sfSO http://security.ubuntu.com/ubuntu/pool/main/b/bind9/libdns162_9.10.3.dfsg.P4-8ubuntu1.10_amd64.deb
[00:01:04] Attempting with retry: curl -sfSO http://security.ubuntu.com/ubuntu/pool/main/b/bind9/liblwres141_9.10.3.dfsg.P4-8ubuntu1.10_amd64.deb
[00:01:04] Attempting with retry: curl -sfSO http://security.ubuntu.com/ubuntu/pool/main/b/bind9/libbind9-140_9.10.3.dfsg.P4-8ubuntu1.10_amd64.deb
[00:01:04] Attempting with retry: curl -sfSO http://security.ubuntu.com/ubuntu/pool/main/b/bind9/libisc160_9.10.3.dfsg.P4-8ubuntu1.10_amd64.deb
[00:01:05] Attempting with retry: curl -sfSO http://security.ubuntu.com/ubuntu/pool/main/b/bind9/libisccfg140_9.10.3.dfsg.P4-8ubuntu1.10_amd64.deb
[00:01:05] Attempting with retry: curl -sfSO http://security.ubuntu.com/ubuntu/pool/main/g/geoip/libgeoip1_1.6.9-1_amd64.deb
[00:01:06] Attempting with retry: curl -sfSO http://security.ubuntu.com/ubuntu/pool/main/o/openssl/libssl1.0.0_1.0.2g-1ubuntu4.13_amd64.deb
[00:01:06] travis_fold:start:check_dns_9696
Checking DNS for travis-ci/travis-ci#9696! --------------------------
[00:01:06] travis_fold:start:resolv-conf
cat /etc/resolv.conf --------------------------------------------
---
[00:52:33] ....................................................................................................
[00:52:35] ....................................................................................................
[00:52:38] ....................................................................................................
[00:52:41] ....................................................................................................
[00:52:44] .............iiiiiiiii..............................................................................
[00:52:46] .....................................................................................F..............
[00:52:53] ...................i................................................................................
[00:52:56] ..............................i.....................................................................
[00:52:59] ....................................................................................................
[00:53:02] ....................................................................................................
---
[00:53:08] diff of stderr:
[00:53:08] 
[00:53:08] 8   --> $DIR/ex2a-push-one-existing-name-early-bound.rs:17:5
[00:53:08] 9    |
[00:53:08] 10 LL | fn baz<'a, 'b, T>(x: &mut Vec<&'a T>, y: &T)
[00:53:08] -    |                                       - consider changing the type of `y` to `&'a T`
[00:53:08] +    |                                          -- help: add explicit lifetime `'a` to the type of `y`: `&'a T`
[00:53:08] 12 ...
[00:53:08] 13 LL |     x.push(y); //~ ERROR explicit lifetime required
[00:53:08] 14    |     ^^^^^^^^^ lifetime `'a` required
[00:53:08] 
[00:53:08] The actual stderr differed from the expected stderr.
[00:53:08] The actual stderr differed from the expected stderr.
[00:53:08] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetime-errors/ex2a-push-one-existing-name-early-bound.nll/ex2a-push-one-existing-name-early-bound.nll.stderr
[00:53:08] To update references, rerun the tests and pass the `--bless` flag
[00:53:08] To only update this specific test, also pass `--test-args lifetime-errors/ex2a-push-one-existing-name-early-bound.rs`
[00:53:08] error: 1 errors occurred comparing output.
[00:53:08] status: exit code: 1
[00:53:08] status: exit code: 1
[00:53:08] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lifetime-errors/ex2a-push-one-existing-name-early-bound.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetime-errors/ex2a-push-one-existing-name-early-bound.nll/a" "-Zborrowck=mir" "-Ztwo-phase-borrows" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetime-errors/ex2a-push-one-existing-name-early-bound.nll/auxiliary" "-A" "unused"
[00:53:08] ------------------------------------------
[00:53:08] 
[00:53:08] ------------------------------------------
[00:53:08] stderr:
[00:53:08] stderr:
[00:53:08] ------------------------------------------
[00:53:08] {"message":"not reporting region error due to nll","code":null,"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/lifetime-errors/ex2a-push-one-existing-name-early-bound.rs","byte_start":618,"byte_end":619,"line_start":17,"line_end":17,"column_start":12,"column_end":13,"is_primary":true,"text":[{"text":"    x.push(y); //~ ERROR explicit lifetime required","highlight_start":12,"highlight_end":13}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: not reporting region error due to nll\n  --> /checkout/src/test/ui/lifetime-errors/ex2a-push-one-existing-name-early-bound.rs:17:12\n   |\nLL |     x.push(y); //~ ERROR explicit lifetime required\n   |            ^\n\n"}
[00:53:08] {"message":"explicit lifetime required in the type of `y`","code":{"code":"E0621","explanation":"\nThis error code indicates a mismatch between the lifetimes appearing in the\nfunction signature (i.e., the parameter types and the return type) and the\ndata-flow found in the function body.\n\nErroneous code example:\n\n