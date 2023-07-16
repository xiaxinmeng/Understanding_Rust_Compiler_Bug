plain
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/2d/99/b2c4e9d5a30f6471e410a146232b4118e697fa3ffc06d6a65efde84debd0/futures-3.2.0-py2-none-any.whl
Requirement already satisfied: six>=1.5 in /usr/lib/python2.7/dist-packages (from python-dateutil<3.0.0,>=2.1; python_version >= "2.7"->botocore==1.10.55->awscli)
Building wheels for collected packages: awscli
  Running setup.py bdist_wheel for awscli ... - \ | / - \ done
Successfully built awscli
Installing collected packages: docutils, jmespath, python-dateutil, botocore, colorama, pyasn1, rsa, futures, s3transfer, awscli
Successfully installed awscli-1.15.56 botocore-1.10.55 colorama-0.3.9 docutils-0.14 futures-3.2.0 jmespath-0.9.3 pyasn1-0.4.3 python-dateutil-2.7.3 rsa-3.4.2 s3transfer-0.1.13
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
---
[00:04:36]     Checking rustc-rayon v0.1.1
[00:04:37]     Checking backtrace v0.3.9
[00:04:38]     Checking flate2 v1.0.1
[00:04:39]     Checking rustc_data_structures v0.0.0 (file:///checkout/src/librustc_data_structures)
[00:04:39] error: trait objects without an explicit `dyn` are deprecated
[00:04:39]    --> librustc_data_structures/sync.rs:271:46
[00:04:39]     |
[00:04:39] 271 |         pub type MetadataRef = OwningRef<Box<Erased + Send + Sync>, [u8]>;
[00:04:39]     |                                              ^^^^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn Erased + Send + Sync`
[00:04:39] note: lint level defined here
[00:04:39]    --> librustc_data_structures/lib.rs:19:9
[00:04:39]     |
[00:04:39]     |
[00:04:39] 19  | #![deny(bare_trait_objects)]
[00:04:39] 
[00:04:40] error: aborting due to previous error
[00:04:40] 
[00:04:40] error: Could not compile `rustc_data_structures`.
[00:04:40] error: Could not compile `rustc_data_structures`.
[00:04:40] 
[00:04:40] Caused by:
[00:04:40]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_data_structures librustc_data_structures/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,metadata -C prefer-dynamic -C opt-level=2 -C metadata=461b7b50453d68b9 -C extra-filename=-461b7b50453d68b9 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern cfg_if=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libcfg_if-bc29d816f37c79ce.rmeta --extern ena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libena-7e262e38f2e046cd.rmeta --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-f65a6b6472ec671d.rmeta --extern parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libparking_lot-056e53de5b502e7f.rmeta --extern parking_lot_core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libparking_lot_core-78305e7d6d5fc245.rmeta --extern rustc_hash=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_hash-64bb9cdc1ab1c2ca.rmeta --extern rustc_rayon=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_rayon-b64425f1f54b957b.rmeta --extern rustc_rayon_core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_rayon_core-2a48bc514d5ebf7f.rmeta --extern rustc_cratesio_shim=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_cratesio_shim-4255bacc8f6a4044.rmeta --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-92c83a4033c4c4c6.rmeta --extern stable_deref_trait=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libstable_deref_trait-15d40ba0bbf4860d.rmeta` (exit code: 101)
[00:04:40] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:04:40] expected success, got: exit code: 101
[00:04:40] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1117:9
[00:04:40] travis_fold:end:stage0-rustc

[00:04:40] travis_time:end:stage0-rustc:start=1531312237677891773,finish=1531312256095917675,duration=18418025902

---
travis_time:end:1b47174d:start=1531312256634539532,finish=1531312256641439856,duration=6900324
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0a74efca
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2f3f9087
$ dmesg | grep -i kill
