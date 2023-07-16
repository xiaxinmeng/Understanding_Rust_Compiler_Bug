plain
  Downloading https://files.pythonhosted.org/packages/d7/14/2a0004d487464d120c9fb85313a75cd3d71a7506955be458eebfe19a6b1d/s3transfer-0.1.13-py2.py3-none-any.whl (59kB)
Collecting botocore==1.10.75 (from awscli)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/91/cd/629d59d751a6bac5e02963e61e5b449619373c6c0e54e8d5af7f7215e081/botocore-1.10.75-py2.py3-none-any.whl (4.4MB)
Collecting docutils>=0.10 (from awscli)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/50/09/c53398e0005b11f7ffb27b7aa720c617aba53be4fb4f4f3f06b9b5c60f28/docutils-0.14-py2-none-any.whl (543kB)
---

[00:01:15] travis_time:end:012fffdb:start=1533938967753048575,finish=1533938993529691211,duration=25776642636
[CI_JOB_NAME=dist-x86_64-musl]
[00:01:15] [CI_JOB_NAME=dist-x86_64-musl]
[00:01:15] Attempting with retry: curl -sfSO http://security.ubuntu.com/ubuntu/pool/main/b/bind9/dnsutils_9.10.3.dfsg.P4-8ubuntu1.10_amd64.deb
[00:01:16] Attempting with retry: curl -sfSO http://security.ubuntu.com/ubuntu/pool/main/b/bind9/libdns162_9.10.3.dfsg.P4-8ubuntu1.10_amd64.deb
[00:01:17] Attempting with retry: curl -sfSO http://security.ubuntu.com/ubuntu/pool/main/b/bind9/liblwres141_9.10.3.dfsg.P4-8ubuntu1.10_amd64.deb
[00:01:17] Attempting with retry: curl -sfSO http://security.ubuntu.com/ubuntu/pool/main/b/bind9/libbind9-140_9.10.3.dfsg.P4-8ubuntu1.10_amd64.deb
[00:01:17] Attempting with retry: curl -sfSO http://security.ubuntu.com/ubuntu/pool/main/b/bind9/libisc160_9.10.3.dfsg.P4-8ubuntu1.10_amd64.deb
[00:01:18] Attempting with retry: curl -sfSO http://security.ubuntu.com/ubuntu/pool/main/b/bind9/libisccfg140_9.10.3.dfsg.P4-8ubuntu1.10_amd64.deb
[00:01:18] Attempting with retry: curl -sfSO http://security.ubuntu.com/ubuntu/pool/main/g/geoip/libgeoip1_1.6.9-1_amd64.deb
[00:01:19] Attempting with retry: curl -sfSO http://security.ubuntu.com/ubuntu/pool/main/o/openssl/libssl1.0.0_1.0.2g-1ubuntu4.13_amd64.deb
[00:01:20] travis_fold:start:check_dns_9696
Checking DNS for travis-ci/travis-ci#9696! --------------------------
[00:01:20] travis_fold:start:resolv-conf
cat /etc/resolv.conf --------------------------------------------
---
[00:43:17] diff of stderr:
[00:43:17] 
[00:43:17] 8   --> $DIR/ex2a-push-one-existing-name-early-bound.rs:17:5
[00:43:17] 9    |
[00:43:17] 10 LL | fn baz<'a, 'b, T>(x: &mut Vec<&'a T>, y: &T)
[00:43:17] -    |                                       - consider changing the type of `y` to `&'a T`
[00:43:17] +    |                                          -- help: add explicit lifetime `'a` to the type of `y`: `&'a T`
[00:43:17] 12 ...
[00:43:17] 13 LL |     x.push(y); //~ ERROR explicit lifetime required
[00:43:17] 14    |     ^^^^^^^^^ lifetime `'a` required
[00:43:17] 
[00:43:17] The actual stderr differed from the expected stderr.
[00:43:17] The actual stderr differed from the expected stderr.
[00:43:17] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetime-errors/ex2a-push-one-existing-name-early-bound.nll/ex2a-push-one-existing-name-early-bound.nll.stderr
[00:43:17] To update references, rerun the tests and pass the `--bless` flag
[00:43:17] To only update this specific test, also pass `--test-args lifetime-errors/ex2a-push-one-existing-name-early-bound.rs`
[00:43:17] error: 1 errors occurred comparing output.
[00:43:17] status: exit code: 1
[00:43:17] status: exit code: 1
[00:43:17] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lifetime-errors/ex2a-push-one-existing-name-early-bound.rs" "--target=x86_64-unknown-linux-musl" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetime-errors/ex2a-push-one-existing-name-early-bound.nll/a" "-Zborrowck=mir" "-Ztwo-phase-borrows" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-musl/native/rust-test-helpers" "-Clinker=/musl-x86_64/bin/musl-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetime-errors/ex2a-push-one-existing-name-early-bound.nll/auxiliary" "-A" "unused"
[00:43:17] ------------------------------------------
[00:43:17] 
[00:43:17] ------------------------------------------
[00:43:17] stderr:
[00:43:17] stderr:
[00:43:17] ------------------------------------------
[00:43:17] {"message":"not reporting region error due to nll","code":null,"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/lifetime-errors/ex2a-push-one-existing-name-early-bound.rs","byte_start":618,"byte_end":619,"line_start":17,"line_end":17,"column_start":12,"column_end":13,"is_primary":true,"text":[{"text":"    x.push(y); //~ ERROR explicit lifetime required","highlight_start":12,"highlight_end":13}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: not reporting region error due to nll\n  --> /checkout/src/test/ui/lifetime-errors/ex2a-push-one-existing-name-early-bound.rs:17:12\n   |\nLL |     x.push(y); //~ ERROR explicit lifetime required\n   |            ^\n\n"}
[00:43:17] {"message":"explicit lifetime required in the type of `y`","code":{"code":"E0621","explanation":"\nThis error code indicates a mismatch between the lifetimes appearing in the\nfunction signature (i.e., the parameter types and the return type) and the\ndata-flow found in the function body.\n\nErroneous code example:\n\n