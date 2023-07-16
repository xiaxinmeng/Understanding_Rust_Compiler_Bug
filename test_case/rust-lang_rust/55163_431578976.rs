plain
[00:01:45] Step 5/11 : RUN bash musl-toolchain.sh x86_64-linux-musl && rm -rf build
[00:01:45]  ---> Running in cfa19a1c5cda
[00:01:46] + TARGET=x86_64-linux-musl
[00:01:46] + ARCH=x86_64
[00:01:46] + OUTPUT=/usr/local
[00:01:46] + shift
[00:01:46] + git clone https://github.com/richfelker/musl-cross-make -b v0.9.7
[00:01:46] Cloning into 'musl-cross-make'...
[00:01:46] Note: checking out 'b85e29c00d35c8c8c196d6713505b837816ad47f'.
[00:01:46] 
[00:01:46] You are in 'detached HEAD' state. You can look around, make experimental
[00:01:46] changes and commit them, and you can discard any commits you make in this
[00:01:46] state without impacting any branches by performing another checkout.
[00:01:46] 
[00:01:46] If you want to create a new branch to retain commits you create, you may
[00:01:46] do so (now or later) by using -b with the checkout command again. Example:
[00:01:46] 
[00:01:46]   git checkout -b <new-branch-name>
[00:01:46] + cd musl-cross-make
[00:01:46] ++ nproc
[00:01:46] + hide_output make -j4 TARGET=x86_64-linux-musl
[00:01:46] + set +x
---
[00:09:47] Sat Oct 20 10:31:23 UTC 2018 - building ...
[00:10:17] Sat Oct 20 10:31:53 UTC 2018 - building ...
[00:10:47] Sat Oct 20 10:32:23 UTC 2018 - building ...
[00:11:17] Sat Oct 20 10:32:53 UTC 2018 - building ...
[00:11:42] musl-toolchain.sh: line 13:    19 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:11:42] + hide_output make install TARGET=x86_64-linux-musl OUTPUT=/usr/local
[00:11:53] musl-toolchain.sh: line 13:  6178 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:11:53] + cd ..
[00:11:53] + cd ..
[00:11:53] + ln -s /usr/local/x86_64-linux-musl/lib/ld-musl-x86_64.so.1 /lib
[00:11:53] + ln -s /usr/local/x86_64-linux-musl/lib/libc.so /lib
[00:11:53] + echo /usr/local/x86_64-linux-musl/lib
[00:11:53] + export CC=x86_64-linux-musl-gcc
[00:11:53] + CC=x86_64-linux-musl-gcc
[00:11:53] + export CXX=x86_64-linux-musl-g++
[00:11:53] + CXX=x86_64-linux-musl-g++
[00:11:53] + '[' '!' -d libunwind-release_60 ']'
[00:11:53] + curl -L https://github.com/llvm-mirror/llvm/archive/release_60.tar.gz
[00:11:53] + tar xzf -
[00:11:53]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
---
[00:12:02] 
100   97k    0   97k    0     0   138k      0 --:--:-- --:--:-- --:--:--  138k
[00:12:02] + mkdir libunwind-build
[00:12:02] + cd libunwind-build
[00:12:02] + cmake ../libunwind-release_60 -DLLVM_PATH=/build/llvm-release_60 -DLIBUNWIND_ENABLE_SHARED=0 -DCMAKE_C_COMPILER=x86_64-linux-musl-gcc -DCMAKE_CXX_COMPILER=x86_64-linux-musl-g++ -DCMAKE_C_FLAGS= -DCMAKE_CXX_FLAGS=
[00:12:02] -- The CXX compiler identification is GNU 6.3.0
[00:12:02] -- Check for working C compiler: /usr/local/bin/x86_64-linux-musl-gcc
[00:12:02] -- Check for working C compiler: /usr/local/bin/x86_64-linux-musl-gcc -- works
[00:12:02] -- Detecting C compiler ABI info
---
[00:15:03] Step 8/11 : ENV RUST_CONFIGURE_ARGS --musl-root-x86_64=/usr/local/x86_64-linux-musl       --enable-extended       --disable-docs
[00:15:03]  ---> Running in 0cdde5c658ba
[00:15:03]  ---> 77444e2eea06
[00:15:03] Removing intermediate container 0cdde5c658ba
[00:15:03] Step 9/11 : ENV HOSTS x86_64-unknown-linux-musl CC_x86_64_unknown_linux_musl x86_64-linux-musl-gcc CXX_x86_64_unknown_linux_musl x86_64-linux-musl-g++
[00:15:03]  ---> d1bf3fc5628f
[00:15:03] Removing intermediate container 44a8873fb867
[00:15:03] Removing intermediate container 44a8873fb867
[00:15:03] Step 10/11 : ENV RUSTFLAGS "-C target-feature=-crt-static"
[00:15:03]  ---> e72e8a6bec0b
[00:15:03] Removing intermediate container 3e5cfa9c7d9a
[00:15:03] Step 11/11 : ENV SCRIPT python2.7 ../x.py test --host $HOSTS --target $HOSTS &&       python2.7 ../x.py dist --host $HOSTS --target $HOSTS
[00:15:03]  ---> Running in a37f30021806
[00:15:03]  ---> Running in a37f30021806
[00:15:04]  ---> 463dca0081e1
[00:15:04] Removing intermediate container a37f30021806
[00:15:04] Successfully built 463dca0081e1
[00:15:04] Successfully tagged rust-ci:latest
[00:15:04] Built container sha256:463dca0081e1bf9a4a91797d2d0122e5672fd98db1b80e18a0a36f004bc7f3ce
[00:15:04] Uploading finished image to s3://rust-lang-ci-sccache2/docker/bcd201e9129968932908b2b5edacc204ccfb5e2342274751124ed41efbc1d81a8d1bbb4c75c4eff37e3e062458c9422439be4a2ef0b8f65b010fb21de14ae6fa
[00:15:04] 
[00:15:04] Partial credentials found in env, missing: AWS_SECRET_ACCESS_KEY
[00:16:21] xargs: docker: terminated by signal 13

[00:17:33] travis_time:end:01225ecd:start=1540030952123867839,finish=1540031878473559001,duration=926349691162
[CI_JOB_NAME=dist-x86_64-musl]
[00:17:33] [CI_JOB_NAME=dist-x86_64-musl]
---
[02:14:12] .................................................................................................... 2200/4658
[02:14:23] ....................................i............................................................... 2300/4658
[02:14:33] .................................................................................................... 2400/4658
[02:14:43] .................................................................................................... 2500/4658
[02:14:52] ...................................................iiiiiiiii........................................ 2600/4658
[02:15:11] .................................................................................................... 2800/4658
[02:15:21] .................................................................................................... 2900/4658
[02:15:30] .................................................................................i.................. 3000/4658
[02:15:38] .................................................................................................... 3100/4658
---
travis_time:start:test_ui-fulldeps
Check compiletest suite=ui-fulldeps mode=ui (x86_64-unknown-linux-musl -> x86_64-unknown-linux-musl)
[02:43:40] 
[02:43:40] running 42 tests
[02:43:47] FFFFFFF.FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF.
[02:43:47] 
[02:43:47] ---- [ui] ui-fulldeps/attribute-order-restricted.rs stdout ----
[02:43:47] 
[02:43:47] 
[02:43:47] error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/attr_proc_macro.rs" failed to compile: 
[02:43:47] status: exit code: 1
[02:43:47] command: "/checkout/obj/build/x86_64-unknown-linux-musl/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/attr_proc_macro.rs" "--target=x86_64-unknown-linux-musl" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-musl/test/ui-fulldeps/attribute-order-restricted/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Clinker=x86_64-linux-musl-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-musl/test/ui-fulldeps/attribute-order-restricted/auxiliary"
[02:43:47] ------------------------------------------
[02:43:47] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:503:22
[02:43:47] 
[02:43:47] ------------------------------------------
[02:43:47] ------------------------------------------
[02:43:47] stderr:
[02:43:47] ------------------------------------------
[02:43:47] {"message":"dropping unsupported crate type `proc-macro` for target `x86_64-unknown-linux-musl`","code":null,"level":"warning","spans":[],"children":[],"rendered":"warning: dropping unsupported crate type `proc-macro` for target `x86_64-unknown-linux-musl`\n\n"}
[02:43:47] {"message":"the `#[proc_macro_attribute]` attribute is only usable with crates of the `proc-macro` crate type","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/auxiliary/attr_proc_macro.rs","byte_start":590,"byte_end":613,"line_start":20,"line_end":20,"column_start":1,"column_end":24,"is_primary":true,"text":[{"text":"#[proc_macro_attribute]","highlight_start":1,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: the `#[proc_macro_attribute]` attribute is only usable with crates of the `proc-macro` crate type\n  --> /checkout/src/test/ui-fulldeps/auxiliary/attr_proc_macro.rs:20:1\n   |\nLL | #[proc_macro_attribute]\n   | ^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[02:43:47] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[02:43:47] ------------------------------------------
[02:43:47] 
[02:43:47] thread '[ui] ui-fulldeps/attribute-order-restricted.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
[02:43:47] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[02:43:47] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[02:43:47] 
[02:43:47] ---- [ui] ui-fulldeps/custom-derive/helper-attr-blocked-by-import.rs stdout ----
[02:43:47] 
[02:43:47] error: auxiliary build of "/checkout/src/test/ui-fulldeps/custom-derive/auxiliary/plugin.rs" failed to compile: 
[02:43:47] status: exit code: 1
[02:43:47] command: "/checkout/obj/build/x86_64-unknown-linux-musl/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/custom-derive/auxiliary/plugin.rs" "--target=x86_64-unknown-linux-musl" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-musl/test/ui-fulldeps/custom-derive/helper-attr-blocked-by-import/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-musl/native/rust-test-helpers" "-Clinker=x86_64-linux-musl-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-musl/test/ui-fulldeps/custom-derive/helper-attr-blocked-by-import/auxiliary"
[02:43:47] ------------------------------------------
[02:43:47] 
[02:43:47] ------------------------------------------
[02:43:47] stderr:
[02:43:47] stderr:
[02:43:47] ------------------------------------------
[02:43:47] {"message":"dropping unsupported crate type `proc-macro` for target `x86_64-unknown-linux-musl`","code":null,"level":"warning","spans":[],"children":[],"rendered":"warning: dropping unsupported crate type `proc-macro` for target `x86_64-unknown-linux-musl`\n\n"}
[02:43:47] {"message":"the `#[proc_macro_derive]` attribute is only usable with crates of the `proc-macro` crate type","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/custom-derive/auxiliary/plugin.rs","byte_start":576,"byte_end":601,"line_start":19,"line_end":19,"column_start":1,"column_end":26,"is_primary":true,"text":[{"text":"#[proc_macro_derive(Foo)]","highlight_start":1,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: the `#[proc_macro_derive]` attribute is only usable with crates of the `proc-macro` crate type\n  --> /checkout/src/test/ui-fulldeps/custom-derive/auxiliary/plugin.rs:19:1\n   |\nLL | #[proc_macro_derive(Foo)]\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[02:43:47] {"message":"the `#[proc_macro_derive]` attribute is only usable with crates of the `proc-macro` crate type","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/custom-derive/auxiliary/plugin.rs","byte_start":670,"byte_end":695,"line_start":24,"line_end":24,"column_start":1,"column_end":26,"is_primary":true,"text":[{"text":"#[proc_macro_derive(Bar)]","highlight_start":1,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: the `#[proc_macro_derive]` attribute is only usable with crates of the `proc-macro` crate type\n  --> /checkout/src/test/ui-fulldeps/custom-derive/auxiliary/plugin.rs:24:1\n   |\nLL | #[proc_macro_derive(Bar)]\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[02:43:47] {"message":"the `#[proc_macro_derive]` attribute is only usable with crates of the `proc-macro` crate type","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/custom-derive/auxiliary/plugin.rs","byte_start":777,"byte_end":829,"line_start":29,"line_end":29,"column_start":1,"column_end":53,"is_primary":true,"text":[{"text":"#[proc_macro_derive(WithHelper, attributes(helper))]","highlight_start":1,"highlight_end":53}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: the `#[proc_macro_derive]` attribute is only usable with crates of the `proc-macro` crate type\n  --> /checkout/src/test/ui-fulldeps/custom-derive/auxiliary/plugin.rs:29:1\n   |\nLL | #[proc_macro_derive(WithHelper, attributes(helper))]\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[02:43:47] {"message":"the `#[proc_macro_attribute]` attribute is only usable with crates of the `proc-macro` crate type","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/custom-derive/auxiliary/plugin.rs","byte_start":912,"byte_end":935,"line_start":34,"line_end":34,"column_start":1,"column_end":24,"is_primary":true,"text":[{"text":"#[proc_macro_attribute]","highlight_start":1,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: the `#[proc_macro_attribute]` attribute is only usable with crates of the `proc-macro` crate type\n  --> /checkout/src/test/ui-fulldeps/custom-derive/auxiliary/plugin.rs:34:1\n   |\nLL | #[proc_macro_attribute]\n   | ^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[02:43:47] {"message":"aborting due to 4 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 4 previous errors\n\n"}
[02:43:47] ------------------------------------------
[02:43:47] 
[02:43:47] thread '[ui] ui-fulldeps/custom-derive/helper-attr-blocked-by-import.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
[02:43:47] 
[02:43:47] 
[02:43:47] ---- [ui] ui-fulldeps/custom-derive/issue-36935.rs stdout ----
[02:43:47] 
[02:43:47] error: auxiliary build of "/checkout/src/test/ui-fulldeps/custom-derive/auxiliary/plugin.rs" failed to compile: 
[02:43:47] status: exit code: 1
[02:43:47] command: "/checkout/obj/build/x86_64-unknown-linux-musl/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/custom-derive/auxiliary/plugin.rs" "--target=x86_64-unknown-linux-musl" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-musl/test/ui-fulldeps/custom-derive/issue-36935/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-musl/native/rust-test-helpers" "-Clinker=x86_64-linux-musl-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-musl/test/ui-fulldeps/custom-derive/issue-36935/auxiliary"
[02:43:47] ------------------------------------------
[02:43:47] 
[02:43:47] ------------------------------------------
[02:43:47] stderr:
[02:43:47] stderr:
[02:43:47] ------------------------------------------
[02:43:47] {"message":"dropping unsupported crate type `proc-macro` for target `x86_64-unknown-linux-musl`","code":null,"level":"warning","spans":[],"children":[],"rendered":"warning: dropping unsupported crate type `proc-macro` for target `x86_64-unknown-linux-musl`\n\n"}
[02:43:47] {"message":"the `#[proc_macro_derive]` attribute is only usable with crates of the `proc-macro` crate type","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/custom-derive/auxiliary/plugin.rs","byte_start":576,"byte_end":601,"line_start":19,"line_end":19,"column_start":1,"column_end":26,"is_primary":true,"text":[{"text":"#[proc_macro_derive(Foo)]","highlight_start":1,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: the `#[proc_macro_derive]` attribute is only usable with crates of the `proc-macro` crate type\n  --> /checkout/src/test/ui-fulldeps/custom-derive/auxiliary/plugin.rs:19:1\n   |\nLL | #[proc_macro_derive(Foo)]\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[02:43:47] {"message":"the `#[proc_macro_derive]` attribute is only usable with crates of the `proc-macro` crate type","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/custom-derive/auxiliary/plugin.rs","byte_start":670,"byte_end":695,"line_start":24,"line_end":24,"column_start":1,"column_end":26,"is_primary":true,"text":[{"text":"#[proc_macro_derive(Bar)]","highlight_start":1,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: the `#[proc_macro_derive]` attribute is only usable with crates of the `proc-macro` crate type\n  --> /checkout/src/test/ui-fulldeps/custom-derive/auxiliary/plugin.rs:24:1\n   |\nLL | #[proc_macro_derive(Bar)]\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[02:43:47] {"message":"the `#[proc_macro_derive]` attribute is only usable with crates of the `proc-macro` crate type","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/custom-derive/auxiliary/plugin.rs","byte_start":777,"byte_end":829,"line_start":29,"line_end":29,"column_start":1,"column_end":53,"is_primary":true,"text":[{"text":"#[proc_macro_derive(WithHelper, attributes(helper))]","highlight_start":1,"highlight_end":53}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: the `#[proc_macro_derive]` attribute is only usable with crates of the `proc-macro` crate type\n  --> /checkout/src/test/ui-fulldeps/custom-derive/auxiliary/plugin.rs:29:1\n   |\nLL | #[proc_macro_derive(WithHelper, attributes(helper))]\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[02:43:47] {"message":"the `#[proc_macro_attribute]` attribute is only usable with crates of the `proc-macro` crate type","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/custom-derive/auxiliary/plugin.rs","byte_start":912,"byte_end":935,"line_start":34,"line_end":34,"column_start":1,"column_end":24,"is_primary":true,"text":[{"text":"#[proc_macro_attribute]","highlight_start":1,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: the `#[proc_macro_attribute]` attribute is only usable with crates of the `proc-macro` crate type\n  --> /checkout/src/test/ui-fulldeps/custom-derive/auxiliary/plugin.rs:34:1\n   |\nLL | #[proc_macro_attribute]\n   | ^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[02:43:47] {"message":"aborting due to 4 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 4 previous errors\n\n"}
[02:43:47] ------------------------------------------
[02:43:47] 
[02:43:47] thread '[ui] ui-fulldeps/custom-derive/issue-36935.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
[02:43:47] 
[02:43:47] 
[02:43:47] ---- [ui] ui-fulldeps/custom-derive/helper-attr-blocked-by-import-ambig.rs stdout ----
[02:43:47] 
[02:43:47] error: auxiliary build of "/checkout/src/test/ui-fulldeps/custom-derive/auxiliary/plugin.rs" failed to compile: 
[02:43:47] status: exit code: 1
[02:43:47] command: "/checkout/obj/build/x86_64-unknown-linux-musl/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/custom-derive/auxiliary/plugin.rs" "--target=x86_64-unknown-linux-musl" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-musl/test/ui-fulldeps/custom-derive/helper-attr-blocked-by-import-ambig/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-musl/native/rust-test-helpers" "-Clinker=x86_64-linux-musl-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-musl/test/ui-fulldeps/custom-derive/helper-attr-blocked-by-import-ambig/auxiliary"
[02:43:47] ------------------------------------------
[02:43:47] 
[02:43:47] ------------------------------------------
[02:43:47] stderr:
[02:43:47] stderr:
[02:43:47] ------------------------------------------
[02:43:47] {"message":"dropping unsupported crate type `proc-macro` for target `x86_64-unknown-linux-musl`","code":null,"level":"warning","spans":[],"children":[],"rendered":"warning: dropping unsupported crate type `proc-macro` for target `x86_64-unknown-linux-musl`\n\n"}
[02:43:47] {"message":"the `#[proc_macro_derive]` attribute is only usable with crates of the `proc-macro` crate type","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/custom-derive/auxiliary/plugin.rs","byte_start":576,"byte_end":601,"line_start":19,"line_end":19,"column_start":1,"column_end":26,"is_primary":true,"text":[{"text":"#[proc_macro_derive(Foo)]","highlight_start":1,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: the `#[proc_macro_derive]` attribute is only usable with crates of the `proc-macro` crate type\n  --> /checkout/src/test/ui-fulldeps/custom-derive/auxiliary/plugin.rs:19:1\n   |\nLL | #[proc_macro_derive(Foo)]\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[02:43:47] {"message":"the `#[proc_macro_derive]` attribute is only usable with crates of the `proc-macro` crate type","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/custom-derive/auxiliary/plugin.rs","byte_start":670,"byte_end":695,"line_start":24,"line_end":24,"column_start":1,"column_end":26,"is_primary":true,"text":[{"text":"#[proc_macro_derive(Bar)]","highlight_start":1,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: the `#[proc_macro_derive]` attribute is only usable with crates of the `proc-macro` crate type\n  --> /checkout/src/test/ui-fulldeps/custom-derive/auxiliary/plugin.rs:24:1\n   |\nLL | #[proc_macro_derive(Bar)]\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[02:43:47] {"message":"the `#[proc_macro_derive]` attribute is only usable with crates of the `proc-macro` crate type","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/custom-derive/auxiliary/plugin.rs","byte_start":777,"byte_end":829,"line_start":29,"line_end":29,"column_start":1,"column_end":53,"is_primary":true,"text":[{"text":"#[proc_macro_derive(WithHelper, attributes(helper))]","highlight_start":1,"highlight_end":53}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: the `#[proc_macro_derive]` attribute is only usable with crates of the `proc-macro` crate type\n  --> /checkout/src/test/ui-fulldeps/custom-derive/auxiliary/plugin.rs:29:1\n   |\nLL | #[proc_macro_derive(WithHelper, attributes(helper))]\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[02:43:47] {"message":"the `#[proc_macro_attribute]` attribute is only usable with crates of the `proc-macro` crate type","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/custom-derive/auxiliary/plugin.rs","byte_start":912,"byte_end":935,"line_start":34,"line_end":34,"column_start":1,"column_end":24,"is_primary":true,"text":[{"text":"#[proc_macro_attribute]","highlight_start":1,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: the `#[proc_macro_attribute]` attribute is only usable with crates of the `proc-macro` crate type\n  --> /checkout/src/test/ui-fulldeps/custom-derive/auxiliary/plugin.rs:34:1\n   |\nLL | #[proc_macro_attribute]\n   | ^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[02:43:47] {"message":"aborting due to 4 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 4 previous errors\n\n"}
[02:43:47] ------------------------------------------
[02:43:47] 
[02:43:47] thread '[ui] ui-fulldeps/custom-derive/helper-attr-blocked-by-import-ambig.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
[02:43:47] 
[02:43:47] 
[02:43:47] ---- [ui] ui-fulldeps/invalid-punct-ident-1.rs stdout ----
[02:43:47] 
[02:43:47] error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/invalid-punct-ident.rs" failed to compile: 
[02:43:47] status: exit code: 1
[02:43:47] command: "/checkout/obj/build/x86_64-unknown-linux-musl/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/invalid-punct-ident.rs" "--target=x86_64-unknown-linux-musl" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-musl/test/ui-fulldeps/invalid-punct-ident-1/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Clinker=x86_64-linux-musl-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-musl/test/ui-fulldeps/invalid-punct-ident-1/auxiliary"
[02:43:47] ------------------------------------------
[02:43:47] 
[02:43:47] ------------------------------------------
[02:43:47] stderr:
[02:43:47] stderr:
[02:43:47] ------------------------------------------
[02:43:47] {"message":"dropping unsupported crate type `proc-macro` for target `x86_64-unknown-linux-musl`","code":null,"level":"warning","spans":[],"children":[],"rendered":"warning: dropping unsupported crate type `proc-macro` for target `x86_64-unknown-linux-musl`\n\n"}
[02:43:47] {"message":"the `#[proc_macro]` attribute is only usable with crates of the `proc-macro` crate type","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/auxiliary/invalid-punct-ident.rs","byte_start":613,"byte_end":626,"line_start":20,"line_end":20,"column_start":1,"column_end":14,"is_primary":true,"text":[{"text":"#[proc_macro]","highlight_start":1,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: the `#[proc_macro]` attribute is only usable with crates of the `proc-macro` crate type\n  --> /checkout/src/test/ui-fulldeps/auxiliary/invalid-punct-ident.rs:20:1\n   |\nLL | #[proc_macro]\n   | ^^^^^^^^^^^^^\n\n"}
[02:43:47] {"message":"the `#[proc_macro]` attribute is only usable with crates of the `proc-macro` crate type","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/auxiliary/invalid-punct-ident.rs","byte_start":744,"byte_end":757,"line_start":25,"line_end":25,"column_start":1,"column_end":14,"is_primary":true,"text":[{"text":"#[proc_macro]","highlight_start":1,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: the `#[proc_macro]` attribute is only usable with crates of the `proc-macro` crate type\n  --> /checkout/src/test/ui-fulldeps/auxiliary/invalid-punct-ident.rs:25:1\n   |\nLL | #[proc_macro]\n   | ^^^^^^^^^^^^^\n\n"}
[02:43:47] {"message":"the `#[proc_macro]` attribute is only usable with crates of the `proc-macro` crate type","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/auxiliary/invalid-punct-ident.rs","byte_start":878,"byte_end":891,"line_start":30,"line_end":30,"column_start":1,"column_end":14,"is_primary":true,"text":[{"text":"#[proc_macro]","highlight_start":1,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: the `#[proc_macro]` attribute is only usable with crates of the `proc-macro` crate type\n  --> /checkout/src/test/ui-fulldeps/auxiliary/invalid-punct-ident.rs:30:1\n   |\nLL | #[proc_macro]\n   | ^^^^^^^^^^^^^\n\n"}
[02:43:47] {"message":"the `#[proc_macro]` attribute is only usable with crates of the `proc-macro` crate type","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/auxiliary/invalid-punct-ident.rs","byte_start":1023,"byte_end":1036,"line_start":35,"line_end":35,"column_start":1,"column_end":14,"is_primary":true,"text":[{"text":"#[proc_macro]","highlight_start":1,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: the `#[proc_macro]` attribute is only usable with crates of the `proc-macro` crate type\n  --> /checkout/src/test/ui-fulldeps/auxiliary/invalid-punct-ident.rs:35:1\n   |\nLL | #[proc_macro]\n   | ^^^^^^^^^^^^^\n\n"}
[02:43:47] {"message":"aborting due to 4 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 4 previous errors\n\n"}
[02:43:47] ------------------------------------------
[02:43:47] 
[02:43:47] thread '[ui] ui-fulldeps/invalid-punct-ident-1.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
[02:43:47] 
[02:43:47] 
[02:43:47] ---- [ui] ui-fulldeps/invalid-punct-ident-3.rs stdout ----
[02:43:47] 
[02:43:47] error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/invalid-punct-ident.rs" failed to compile: 
[02:43:47] status: exit code: 1
[02:43:47] command: "/checkout/obj/build/x86_64-unknown-linux-musl/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/invalid-punct-ident.rs" "--target=x86_64-unknown-linux-musl" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-musl/test/ui-fulldeps/invalid-punct-ident-3/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Clinker=x86_64-linux-musl-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-musl/test/ui-fulldeps/invalid-punct-ident-3/auxiliary"
[02:43:47] ------------------------------------------
[02:43:47] 
[02:43:47] ------------------------------------------
[02:43:47] stderr:
[02:43:47] stderr:
[02:43:47] ------------------------------------------
[02:43:47] {"message":"dropping unsupported crate type `proc-macro` for target `x86_64-unknown-linux-musl`","code":null,"level":"warning","spans":[],"children":[],"rendered":"warning: dropping unsupported crate type `proc-macro` for target `x86_64-unknown-linux-musl`\n\n"}
[02:43:47] {"message":"the `#[proc_macro]` attribute is only usable with crates of the `proc-macro` crate type","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/auxiliary/invalid-punct-ident.rs","byte_start":613,"byte_end":626,"line_start":20,"line_end":20,"column_start":1,"column_end":14,"is_primary":true,"text":[{"text":"#[proc_macro]","highlight_start":1,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: the `#[proc_macro]` attribute is only usable with crates of the `proc-macro` crate type\n  --> /checkout/src/test/ui-fulldeps/auxiliary/invalid-punct-ident.rs:20:1\n   |\nLL | #[proc_macro]\n   | ^^^^^^^^^^^^^\n\n"}
[02:43:47] {"message":"the `#[proc_macro]` attribute is only usable with crates of the `proc-macro` crate type","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/auxiliary/invalid-punct-ident.rs","byte_start":744,"byte_end":757,"line_start":25,"line_end":25,"column_start":1,"column_end":14,"is_primary":true,"text":[{"text":"#[proc_macro]","highlight_start":1,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: the `#[proc_macro]` attribute is only usable with crates of the `proc-macro` crate type\n  --> /checkout/src/test/ui-fulldeps/auxiliary/invalid-punct-ident.rs:25:1\n   |\nLL | #[proc_macro]\n   | ^^^^^^^^^^^^^\n\n"}
[02:43:47] {"message":"the `#[proc_macro]` attribute is only usable with crates of the `proc-macro` crate type","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/auxiliary/invalid-punct-ident.rs","byte_start":878,"byte_end":891,"line_start":30,"line_end":30,"column_start":1,"column_end":14,"is_primary":true,"text":[{"text":"#[proc_macro]","highlight_start":1,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: the `#[proc_macro]` attribute is only usable with crates of the `proc-macro` crate type\n  --> /checkout/src/test/ui-fulldeps/auxiliary/invalid-punct-ident.rs:30:1\n   |\nLL | #[proc_macro]\n   | ^^^^^^^^^^^^^\n\n"}
[02:43:47] {"message":"the `#[proc_macro]` attribute is only usable with crates of the `proc-macro` crate type","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/auxiliary/invalid-punct-ident.rs","byte_start":1023,"byte_end":1036,"line_start":35,"line_end":35,"column_start":1,"column_end":14,"is_primary":true,"text":[{"text":"#[proc_macro]","highlight_start":1,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: the `#[proc_macro]` attribute is only usable with crates of the `proc-macro` crate type\n  --> /checkout/src/test/ui-fulldeps/auxiliary/invalid-punct-ident.rs:35:1\n   |\nLL | #[proc_macro]\n   | ^^^^^^^^^^^^^\n\n"}
[02:43:47] {"message":"aborting due to 4 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 4 previous errors\n\n"}
[02:43:47] ------------------------------------------
[02:43:47] 
[02:43:47] thread '[ui] ui-fulldeps/invalid-punct-ident-3.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
[02:43:47] 
[02:43:47] 
[02:43:47] ---- [ui] ui-fulldeps/invalid-punct-ident-2.rs stdout ----
[02:43:47] 
[02:43:47] error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/invalid-punct-ident.rs" failed to compile: 
[02:43:47] status: exit code: 1
[02:43:47] command: "/checkout/obj/build/x86_64-unknown-linux-musl/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/invalid-punct-ident.rs" "--target=x86_64-unknown-linux-musl" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-musl/test/ui-fulldeps/invalid-punct-ident-2/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Clinker=x86_64-linux-musl-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-musl/test/ui-fulldeps/invalid-punct-ident-2/auxiliary"
[02:43:47] ------------------------------------------
[02:43:47] 
[02:43:47] ------------------------------------------
[02:43:47] stderr:
[02:43:47] stderr:
[02:43:47] ------------------------------------------
[02:43:47] {"message":"dropping unsupported crate type `proc-macro` for target `x86_64-unknown-linux-musl`","code":null,"level":"warning","spans":[],"children":[],"rendered":"warning: dropping unsupported crate type `proc-macro` for target `x86_64-unknown-linux-musl`\n\n"}
[02:43:47] {"message":"the `#[proc_macro]` attribute is only usable with crates of the `proc-macro` crate type","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/auxiliary/invalid-punct-ident.rs","byte_start":613,"byte_end":626,"line_start":20,"line_end":20,"column_start":1,"column_end":14,"is_primary":true,"text":[{"text":"#[proc_macro]","highlight_start":1,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: the `#[proc_macro]` attribute is only usable with crates of the `proc-macro` crate type\n  --> /checkout/src/test/ui-fulldeps/auxiliary/invalid-punct-ident.rs:20:1\n   |\nLL | #[proc_macro]\n   | ^^^^^^^^^^^^^\n\n"}
[02:43:47] {"message":"the `#[proc_macro]` attribute is only usable with crates of the `proc-macro` crate type","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/auxiliary/invalid-punct-ident.rs","byte_start":744,"byte_end":757,"line_start":25,"line_end":25,"column_start":1,"column_end":14,"is_primary":true,"text":[{"text":"#[proc_macro]","highlight_start":1,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: the `#[proc_macro]` attribute is only usable with crates of the `proc-macro` crate type\n  --> /checkout/src/test/ui-fulldeps/auxiliary/invalid-punct-ident.rs:25:1\n   |\nLL | #[proc_macro]\n   | ^^^^^^^^^^^^^\n\n"}
[02:43:47] {"message":"the `#[proc_macro]` attribute is only usable with crates of the `proc-macro` crate type","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/auxiliary/invalid-punct-ident.rs","byte_start":878,"byte_end":891,"line_start":30,"line_end":30,"column_start":1,"column_end":14,"is_primary":true,"text":[{"text":"#[proc_macro]","highlight_start":1,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: the `#[proc_macro]` attribute is only usable with crates of the `proc-macro` crate type\n  --> /checkout/src/test/ui-fulldeps/auxiliary/invalid-punct-ident.rs:30:1\n   |\nLL | #[proc_macro]\n   | ^^^^^^^^^^^^^\n\n"}
[02:43:47] {"message":"the `#[proc_macro]` attribute is only usable with crates of the `proc-macro` crate type","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/auxiliary/invalid-punct-ident.rs","byte_start":1023,"byte_end":1036,"line_start":35,"line_end":35,"column_start":1,"column_end":14,"is_primary":true,"text":[{"text":"#[proc_macro]","highlight_start":1,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: the `#[proc_macro]` attribute is only usable with crates of the `proc-macro` crate type\n  --> /checkout/src/test/ui-fulldeps/auxiliary/invalid-punct-ident.rs:35:1\n   |\nLL | #[proc_macro]\n   | ^^^^^^^^^^^^^\n\n"}
[02:43:47] {"message":"aborting due to 4 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 4 previous errors\n\n"}
[02:43:47] ------------------------------------------
[02:43:47] 
[02:43:47] thread '[ui] ui-fulldeps/invalid-punct-ident-2.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
[02:43:47] 
[02:43:47] 
[02:43:47] ---- [ui] ui-fulldeps/invalid-punct-ident-4.rs stdout ----
[02:43:47] 
[02:43:47] error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/invalid-punct-ident.rs" failed to compile: 
[02:43:47] status: exit code: 1
[02:43:47] command: "/checkout/obj/build/x86_64-unknown-linux-musl/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/invalid-punct-ident.rs" "--target=x86_64-unknown-linux-musl" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-musl/test/ui-fulldeps/invalid-punct-ident-4/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Clinker=x86_64-linux-musl-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-musl/test/ui-fulldeps/invalid-punct-ident-4/auxiliary"
[02:43:47] ------------------------------------------
[02:43:47] 
[02:43:47] ------------------------------------------
[02:43:47] stderr:
[02:43:47] stderr:
[02:43:47] ------------------------------------------
[02:43:47] {"message":"dropping unsupported crate type `proc-macro` for target `x86_64-unknown-linux-musl`","code":null,"level":"warning","spans":[],"children":[],"rendered":"warning: dropping unsupported crate type `proc-macro` for target `x86_64-unknown-linux-musl`\n\n"}
[02:43:47] {"message":"the `#[proc_macro]` attribute is only usable with crates of the `proc-macro` crate type","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/auxiliary/invalid-punct-ident.rs","byte_start":613,"byte_end":626,"line_start":20,"line_end":20,"column_start":1,"column_end":14,"is_primary":true,"text":[{"text":"#[proc_macro]","highlight_start":1,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: the `#[proc_macro]` attribute is only usable with crates of the `proc-macro` crate type\n  --> /checkout/src/test/ui-fulldeps/auxiliary/invalid-punct-ident.rs:20:1\n   |\nLL | #[proc_macro]\n   | ^^^^^^^^^^^^^\n\n"}
[02:43:47] {"message":"the `#[proc_macro]` attribute is only usable with crates of the `proc-macro` crate type","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/auxiliary/invalid-punct-ident.rs","byte_start":744,"byte_end":757,"line_start":25,"line_end":25,"column_start":1,"column_end":14,"is_primary":true,"text":[{"text":"#[proc_macro]","highlight_start":1,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: the `#[proc_macro]` attribute is only usable with crates of the `proc-macro` crate type\n  --> /checkout/src/test/ui-fulldeps/auxiliary/invalid-punct-ident.rs:25:1\n   |\nLL | #[proc_macro]\n   | ^^^^^^^^^^^^^\n\n"}
[02:43:47] {"message":"the `#[proc_macro]` attribute is only usable with crates of the `proc-macro` crate type","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/auxiliary/invalid-punct-ident.rs","byte_start":878,"byte_end":891,"line_start":30,"line_end":30,"column_start":1,"column_end":14,"is_primary":true,"text":[{"text":"#[proc_macro]","highlight_start":1,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: the `#[proc_macro]` attribute is only usable with crates of the `proc-macro` crate type\n  --> /checkout/src/test/ui-fulldeps/auxiliary/invalid-punct-ident.rs:30:1\n   |\nLL | #[proc_macro]\n   | ^^^^^^^^^^^^^\n\n"}
[02:43:47] {"message":"the `#[proc_macro]` attribute is only usable with crates of the `proc-macro` crate type","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/auxiliary/invalid-punct-ident.rs","byte_start":1023,"byte_end":1036,"line_start":35,"line_end":35,"column_start":1,"column_end":14,"is_primary":true,"text":[{"text":"#[proc_macro]","highlight_start":1,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: the `#[proc_macro]` attribute is only usable with crates of the `proc-macro` crate type\n  --> /checkout/src/test/ui-fulldeps/auxiliary/invalid-punct-ident.rs:35:1\n   |\nLL | #[proc_macro]\n   | ^^^^^^^^^^^^^\n\n"}
[02:43:47] {"message":"aborting due to 4 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 4 previous errors\n\n"}
[02:43:47] ------------------------------------------
[02:43:47] 
[02:43:47] thread '[ui] ui-fulldeps/invalid-punct-ident-4.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
[02:43:47] 
[02:43:47] 
[02:43:47] ---- [ui] ui-fulldeps/lifetimes.rs stdout ----
[02:43:47] 
[02:43:47] error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lifetimes.rs" failed to compile: 
[02:43:47] status: exit code: 1
[02:43:47] command: "/checkout/obj/build/x86_64-unknown-linux-musl/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lifetimes.rs" "--target=x86_64-unknown-linux-musl" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-musl/test/ui-fulldeps/lifetimes/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-musl/native/rust-test-helpers" "-Clinker=x86_64-linux-musl-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-musl/test/ui-fulldeps/lifetimes/auxiliary"
[02:43:47] ------------------------------------------
[02:43:47] 
[02:43:47] ------------------------------------------
[02:43:47] stderr:
[02:43:47] stderr:
[02:43:47] ------------------------------------------
[02:43:47] {"message":"dropping unsupported crate type `proc-macro` for target `x86_64-unknown-linux-musl`","code":null,"level":"warning","spans":[],"children":[],"rendered":"warning: dropping unsupported crate type `proc-macro` for target `x86_64-unknown-linux-musl`\n\n"}
[02:43:47] {"message":"the `#[proc_macro]` attribute is only usable with crates of the `proc-macro` crate type","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/auxiliary/lifetimes.rs","byte_start":566,"byte_end":579,"line_start":19,"line_end":19,"column_start":1,"column_end":14,"is_primary":true,"text":[{"text":"#[proc_macro]","highlight_start":1,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: the `#[proc_macro]` attribute is only usable with crates of the `proc-macro` crate type\n  --> /checkout/src/test/ui-fulldeps/auxiliary/lifetimes.rs:19:1\n   |\nLL | #[proc_macro]\n   | ^^^^^^^^^^^^^\n\n"}
[02:43:47] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[02:43:47] ------------------------------------------
[02:43:47] 
[02:43:47] thread '[ui] ui-fulldeps/lifetimes.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
[02:43:47] 
[02:43:47] 
[02:43:47] ---- [ui] ui-fulldeps/lint-plugin-cmdline-allow.rs stdout ----
[02:43:47] 
[02:43:47] error: test compilation failed although it shouldn't!
[02:43:47] status: exit code: 1
[02:43:47] command: "/checkout/obj/build/x86_64-unknown-linux-musl/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint-plugin-cmdline-allow.rs" "--target=x86_64-unknown-linux-musl" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-musl/test/ui-fulldeps/lint-plugin-cmdline-allow/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-musl/native/rust-test-helpers" "-Clinker=x86_64-linux-musl-gcc" "-A" "test-lint" "-L" "/checkout/obj/build/x86_64-unknown-linux-musl/test/ui-fulldeps/lint-plugin-cmdline-allow/auxiliary" "-A" "unused"
[02:43:47] ------------------------------------------
[02:43:47] 
[02:43:47] ------------------------------------------
[02:43:47] stderr:
[02:43:47] stderr:
[02:43:47] ------------------------------------------
[02:43:47] {"message":"can't find crate for `lint_plugin_test`","code":{"code":"E0463","explanation":"\nA plugin/crate was declared but cannot be found. Erroneous code example:\n\n