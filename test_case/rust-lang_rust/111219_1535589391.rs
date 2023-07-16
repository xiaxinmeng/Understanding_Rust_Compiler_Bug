plain
      System Firmware Version: VMW71.00V.13989454.B64.1906190538
      OS Loader Version: 540.120.3~22
      Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
      SMC Version (system): 2.8f0
      Serial Number (system): VMNlaJ2Uk8MQ
      Provisioning UDID: 4203018E-580F-C1B5-9525-B745CECA79EB

hw.ncpu: 3
hw.byteorder: 1234
---

---- [ui] tests/ui/deployment-target/macos-target.rs stdout ----
diff of stdout:

- deployment_target=$CURRENT_MAJOR_VERSION.0
2 


The actual stdout differed from the expected stdout.
The actual stdout differed from the expected stdout.
Actual stdout saved to /Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/ui/deployment-target/macos-target/macos-target.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args deployment-target/macos-target.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/bin/rustc" "/Users/runner/work/rust/rust/tests/ui/deployment-target/macos-target.rs" "-Zthreads=1" "--sysroot" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2" "--target=x86_64-apple-darwin" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/Users/runner/work/rust/rust/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/ui/deployment-target/macos-target" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/Users/runner/work/rust/rust/build/x86_64-apple-darwin/native/rust-test-helpers" "-L" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/ui/deployment-target/macos-target/auxiliary" "--print" "deployment-target"
deployment_target=10.8
------------------------------------------
stderr: none

