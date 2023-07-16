plain
    100% |████████████████████████████████| 51kB 8.0MB/s 
Collecting botocore==1.10.51 (from awscli)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/e3/ba/f6c9220d87784a85f24a8f2425edccb2f330d15c304ea2373ed8206a03ca/botocore-1.10.51-py2.py3-none-any.whl (4.4MB)
    0% |                                | 10kB 39.5MB/s eta 0:00:01
    0% |▏                               | 20kB 15.8MB/s eta 0:00:01
    0% |▎                               | 30kB 20.5MB/s eta 0:00:01
    0% |▎                               | 40kB 15.8MB/s eta 0:00:01
---
[00:46:31] ....................................................................................................
[00:46:35] ....................................................................................................
[00:46:38] ....................................................................................................
[00:46:42] ....................................................................................................
[00:46:47] ..........F.........................................................................................
[00:46:59] ....................................................................................................
[00:47:04] ....................................................................................................
[00:47:12] ...............................i....................................................................
[00:47:17] ....................i...............................................................................
---
[00:47:39] 
[00:47:39] ---- [ui] ui/feature-gate-wasm_import_module.rs stdout ----
[00:47:39] diff of stderr:
[00:47:39] 
[00:47:39] - error[E0658]: experimental attribute (see issue #51088)
[00:47:39] + error[E0658]: experimental attribute (see issue #52090)
[00:47:39] 2   --> $DIR/feature-gate-wasm_import_module.rs:11:1
[00:47:39] 3    |
[00:47:39] 4 LL | #[wasm_import_module = "test"] //~ ERROR: experimental
[00:47:39] 
[00:47:39] The actual stderr differed from the expected stderr.
[00:47:39] The actual stderr differed from the expected stderr.
[00:47:39] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gate-wasm_import_module/feature-gate-wasm_import_module.stderr
[00:47:39] To update references, rerun the tests and pass the `--bless` flag
[00:47:39] To only update this specific test, also pass `--test-args feature-gate-wasm_import_module.rs`
[00:47:39] error: 1 errors occurred comparing output.
[00:47:39] status: exit code: 101
[00:47:39] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:47:39] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:47:39] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gate-wasm_import_module.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gate-wasm_import_module/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gate-wasm_import_module/auxiliary" "-A" "unused"
[00:47:39] ------------------------------------------
[00:47:39] 
[00:47:39] ------------------------------------------
[00:47:39] stderr:
[00:47:39] stderr:
[00:47:39] ------------------------------------------
[00:47:39] {"message":"experimental attribute (see issue #52090)","code":{"code":"E0658","explanation":"\nAn unstable feature was used.\n\nErroneous code example:\n\n