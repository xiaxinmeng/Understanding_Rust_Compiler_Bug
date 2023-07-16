plain
      Memory: 14 GB
      System Firmware Version: VMW71.00V.13989454.B64.1906190538
      Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
      SMC Version (system): 2.8f0
      Serial Number (system): VM3zpOgKNHxK
      Provisioning UDID: 4203018E-580F-C1B5-9525-B745CECA79EB

hw.ncpu: 3
hw.byteorder: 1234
---
---- [ui] src/test/ui/asm/x86_64/issue-96797.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/bin/rustc" "/Users/runner/work/rust/rust/src/test/ui/asm/x86_64/issue-96797.rs" "-Zthreads=1" "--target=x86_64-apple-darwin" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/ui/asm/x86_64/issue-96797" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/Users/runner/work/rust/rust/build/x86_64-apple-darwin/native/rust-test-helpers" "-O" "-L" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/ui/asm/x86_64/issue-96797/auxiliary"
stdout: none
--- stderr -------------------------------
error: linking with `cc` failed: exit status: 1
   |
   = note: "cc" "-m64" "-arch" "x86_64" "/var/folders/24/8k48jl6d249_n_qfxwsl6xvm0000gn/T/rustcR2neb9/symbols.o" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/ui/asm/x86_64/issue-96797/issue-96797.issue_96797.a3b522df-cgu.0.rcgu.o" "-L" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/native/rust-test-helpers" "-L" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/ui/asm/x86_64/issue-96797/auxiliary" "-L" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "-L" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "-lstd-585325937ee68bad" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libcompiler_builtins-5b6493f86f183d69.rlib" "-lSystem" "-lresolv" "-lc" "-lm" "-liconv" "-L" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "-o" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/ui/asm/x86_64/issue-96797/issue-96797" "-Wl,-dead_strip" "-nodefaultlibs" "-Wl,-rpath,@loader_path/../../../../../stage2/lib/rustlib/x86_64-apple-darwin/lib"
   = note: ld: warning: directory not found for option '-L/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/ui/asm/x86_64/issue-96797/auxiliary'
           Undefined symbols for architecture x86_64:
             "_call_foobar", referenced from:
                 issue_96797::main::hd19441dc242f96e8 in issue-96797.issue_96797.a3b522df-cgu.0.rcgu.o
           ld: symbol(s) not found for architecture x86_64
           clang: error: linker command failed with exit code 1 (use -v to see invocation)

error: aborting due to previous error
------------------------------------------

