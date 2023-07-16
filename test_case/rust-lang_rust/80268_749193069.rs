
~/t/kernel ❯❯❯ lldb cargo                                  ✘ 101 
(lldb) target create "cargo"
Current executable set to 'cargo' (x86_64).
(lldb) run build --target=aarch64-unknown-none.json               Process 14517 launched: '/Users/tnishinaga/.cargo/bin/cargo' (x86_64)
Process 14517 stopped
* thread #2, stop reason = exec
    frame #0: 0x0000000100eed000 dyld`_dyld_start
dyld`_dyld_start:
->  0x100eed000 <+0>: popq   %rdi
    0x100eed001 <+1>: pushq  $0x0
    0x100eed003 <+3>: movq   %rsp, %rbp
    0x100eed006 <+6>: andq   $-0x10, %rsp
Target 0: (cargo) stopped.
(lldb) c
Process 14517 resuming
   Compiling core v0.0.0 (/Users/tnishinaga/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/src/rust/library/core)
   Compiling rustc-std-workspace-core v1.99.0 (/Users/tnishinaga/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/src/rust/library/rustc-std-workspace-core)
   Compiling compiler_builtins v0.1.36
error: could not compile `core`

Caused by:
  process didn't exit successfully: `rustc --crate-name core --edition=2018 /Users/tnishinaga/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/src/rust/library/core/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts --crate-type lib --emit=dep-info,metadata,link -C panic=abort -C embed-bitcode=no -C debuginfo=2 -C metadata=0e695d8bdba0e0fe -C extra-filename=-0e695d8bdba0e0fe --out-dir /Users/tnishinaga/tmp/kernel/target/aarch64-unknown-none/debug/deps --target /Users/tnishinaga/tmp/kernel/aarch64-unknown-none.json -Z force-unstable-if-unmarked -L dependency=/Users/tnishinaga/tmp/kernel/target/aarch64-unknown-none/debug/deps -L dependency=/Users/tnishinaga/tmp/kernel/target/debug/deps --cap-lints allow` (signal: 11, SIGSEGV: invalid memory reference)
warning: build failed, waiting for other jobs to finish...
Process 14517 stopped
* thread #3, stop reason = signal SIGUSR1
    frame #0: 0x00007fff72696062 libsystem_kernel.dylib`__psynch_mutexwait + 10
libsystem_kernel.dylib`__psynch_mutexwait:
->  0x7fff72696062 <+10>: jae    0x7fff7269606c            ; <+20>
    0x7fff72696064 <+12>: movq   %rax, %rdi
    0x7fff72696067 <+15>: jmp    0x7fff72694629            ; cerror_nocancel
    0x7fff7269606c <+20>: retq   
Target 0: (cargo) stopped.
