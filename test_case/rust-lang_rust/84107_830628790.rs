plain
      Memory: 14 GB
      Boot ROM Version: VMW71.00V.13989454.B64.1906190538
      Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
      SMC Version (system): 2.8f0
      Serial Number (system): VMijQDQ+cIoX

hw.ncpu: 3
hw.byteorder: 1234
+ ./x.py --stage 2 test
---
   Compiling rustc-std-workspace-core v1.99.0 (/Users/runner/work/rust/rust/library/rustc-std-workspace-core)
[RUSTC-TIMING] rustc_std_workspace_core test:false 0.037
[RUSTC-TIMING] libc test:false 0.812
   Compiling alloc v0.0.0 (/Users/runner/work/rust/rust/library/alloc)
error: unknown token in expression
    pushq  %rbp
error: unknown token in expression
error: unknown token in expression
    movq   %rsp, %rbp
error: unknown token in expression
error: unknown token in expression
    mov    %rax,%r11        // duplicate %rax as we're clobbering %r11
error: unknown token in expression
error: unknown token in expression
    cmp    $0x1000,%r11
error: unknown token in expression
error: unknown token in expression
    sub    $0x1000,%rsp
error: unknown token in expression
error: unknown token in expression
    test   %rsp,8(%rsp)
error: unknown token in expression
error: unknown token in expression
    sub    $0x1000,%r11
error: unknown token in expression
error: unknown token in expression
    cmp    $0x1000,%r11
error: unknown token in expression
error: unknown token in expression
    sub    %r11,%rsp
error: unknown token in expression
error: unknown token in expression
    test   %rsp,8(%rsp)
error: unknown token in expression
error: unknown token in expression
    add    %rax,%rsp
error: unknown token in expression
error: unknown token in expression
    pushq  %rbp
error: unknown token in expression
error: unknown token in expression
    movq   %rsp, %rbp
error: unknown token in expression
error: unknown token in expression
    mov    %rax,%r11        // duplicate %rax as we're clobbering %r11
error: unknown token in expression
error: unknown token in expression
    cmp    $0x1000,%r11
error: unknown token in expression
error: unknown token in expression
    sub    $0x1000,%rsp
error: unknown token in expression
error: unknown token in expression
    test   %rsp,8(%rsp)
error: unknown token in expression
error: unknown token in expression
    sub    $0x1000,%r11
error: unknown token in expression
error: unknown token in expression
    cmp    $0x1000,%r11
error: unknown token in expression
error: unknown token in expression
    sub    %r11,%rsp
error: unknown token in expression
error: unknown token in expression
    test   %rsp,8(%rsp)
error: unknown token in expression
error: unknown token in expression
    add    %rax,%rsp
error: unknown token in expression
error: unknown token in expression
    pushq  %rbp
error: unknown token in expression
error: unknown token in expression
    movq   %rsp, %rbp
error: unknown token in expression
error: unknown token in expression
    mov    %rax,%r11        // duplicate %rax as we're clobbering %r11
error: unknown token in expression
error: unknown token in expression
    cmp    $0x1000,%r11
error: unknown token in expression
error: unknown token in expression
    sub    $0x1000,%rsp
error: unknown token in expression
error: unknown token in expression
    test   %rsp,8(%rsp)
error: unknown token in expression
error: unknown token in expression
    sub    $0x1000,%r11
error: unknown token in expression
error: unknown token in expression
    cmp    $0x1000,%r11
error: unknown token in expression
error: unknown token in expression
    sub    %r11,%rsp
error: unknown token in expression
error: unknown token in expression
    test   %rsp,8(%rsp)
error: unknown token in expression
error: unknown token in expression
    add    %rax,%rsp
error: unknown token in expression
error: unknown token in expression
    pushq  %rbp
error: unknown token in expression
error: unknown token in expression
    movq   %rsp, %rbp
error: unknown token in expression
error: unknown token in expression
    mov    %rax,%r11        // duplicate %rax as we're clobbering %r11
error: unknown token in expression
error: unknown token in expression
    cmp    $0x1000,%r11
error: unknown token in expression
error: unknown token in expression
    sub    $0x1000,%rsp
error: unknown token in expression
error: unknown token in expression
    test   %rsp,8(%rsp)
error: unknown token in expression
error: unknown token in expression
    sub    $0x1000,%r11
error: unknown token in expression
error: unknown token in expression
    cmp    $0x1000,%r11
error: unknown token in expression
error: unknown token in expression
    sub    %r11,%rsp
error: unknown token in expression
error: unknown token in expression
    test   %rsp,8(%rsp)
error: unknown token in expression
error: unknown token in expression
    add    %rax,%rsp
error: unknown token in expression
  |
note: instantiated into assembly here
note: instantiated into assembly here
 --> <inline asm>:7:12
  |
7 |     pushq  %rbp

error: unknown token in expression
   |
note: instantiated into assembly here
note: instantiated into assembly here
  --> <inline asm>:10:12
   |
10 |     movq   %rsp, %rbp

error: unknown token in expression
   |
note: instantiated into assembly here
note: instantiated into assembly here
  --> <inline asm>:13:12
   |
13 |     mov    %rax,%r11        // duplicate %rax as we're clobbering %r11

error: unknown token in expression
   |
note: instantiated into assembly here
note: instantiated into assembly here
  --> <inline asm>:29:12
   |
29 |     cmp    $0x1000,%r11

error: unknown token in expression
   |
note: instantiated into assembly here
note: instantiated into assembly here
  --> <inline asm>:32:12
   |
32 |     sub    $0x1000,%rsp

error: unknown token in expression
   |
note: instantiated into assembly here
note: instantiated into assembly here
  --> <inline asm>:33:12
   |
33 |     test   %rsp,8(%rsp)

error: unknown token in expression
   |
note: instantiated into assembly here
note: instantiated into assembly here
  --> <inline asm>:34:12
   |
34 |     sub    $0x1000,%r11

error: unknown token in expression
   |
note: instantiated into assembly here
note: instantiated into assembly here
  --> <inline asm>:35:12
   |
35 |     cmp    $0x1000,%r11

error: unknown token in expression
   |
note: instantiated into assembly here
note: instantiated into assembly here
  --> <inline asm>:41:12
   |
41 |     sub    %r11,%rsp

error: unknown token in expression
   |
note: instantiated into assembly here
note: instantiated into assembly here
  --> <inline asm>:42:12
   |
42 |     test   %rsp,8(%rsp)

error: unknown token in expression
   |
note: instantiated into assembly here
note: instantiated into assembly here
  --> <inline asm>:47:12
   |
47 |     add    %rax,%rsp

error: aborting due to 11 previous errors

[RUSTC-TIMING] compiler_builtins test:false 3.925
[RUSTC-TIMING] compiler_builtins test:false 3.925
error: could not compile `compiler_builtins`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
[RUSTC-TIMING] alloc test:false 3.958
[RUSTC-TIMING] core test:false 45.024
error: build failed
command did not execute successfully: "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0/bin/cargo" "build" "--target" "x86_64-apple-darwin" "-Zbinary-dep-depinfo" "-j" "3" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace profiler compiler-builtins-c" "--manifest-path" "/Users/runner/work/rust/rust/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /Users/runner/work/rust/rust/build/bootstrap/debug/bootstrap --stage 2 test
Build completed unsuccessfully in 0:32:54
