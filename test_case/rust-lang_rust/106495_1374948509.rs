plain
      System Firmware Version: VMW71.00V.13989454.B64.1906190538
      OS Loader Version: 540.120.3~22
      Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
      SMC Version (system): 2.8f0
      Serial Number (system): VM2gRtTXeeXH
      Provisioning UDID: 4203018E-580F-C1B5-9525-B745CECA79EB

hw.ncpu: 12
hw.byteorder: 1234
---
---- [ui] src/test/ui/lto/issue-100772.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/bin/rustc" "/Users/runner/work/rust/rust/src/test/ui/lto/issue-100772.rs" "-Zthreads=1" "--target=x86_64-apple-darwin" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-o" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/ui/lto/issue-100772/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/Users/runner/work/rust/rust/build/x86_64-apple-darwin/native/rust-test-helpers" "-L" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/ui/lto/issue-100772/auxiliary" "-Clto" "-Ctarget-feature=-crt-static" "-Zsanitizer=cfi"
stdout: none
--- stderr -------------------------------
error: unsupported symbol modifier in branch relocation
note: instantiated into assembly here
note: instantiated into assembly here
  --> <inline asm>:1:2
   |
LL |     jmp __ZN12issue_1007724main17hf8e2e1e1eb2f0e5eE.cfi@plt

error: aborting due to previous error
------------------------------------------

