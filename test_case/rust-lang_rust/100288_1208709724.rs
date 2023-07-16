plain
Successfully built 0be3437e3ddb
Successfully tagged rust-ci:latest
Built container sha256:0be3437e3ddbe9c806a31304ec47c991fbe691b9df66334f35cd94b8c9414b64
Uploading finished image to https://ci-caches.rust-lang.org/docker/55e0616340de9767316ac8a2060e4b2402700344c201ac04865af7e91102302ab3c0bf7f297a771d5ca396e00e9848a4e4279034a0339b65793190c46eafc724
upload failed: - to s3://rust-lang-ci-sccache2/docker/55e0616340de9767316ac8a2060e4b2402700344c201ac04865af7e91102302ab3c0bf7f297a771d5ca396e00e9848a4e4279034a0339b65793190c46eafc724 Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-llvm-12]
---
   Compiling rustc_target v0.0.0 (/checkout/compiler/rustc_target)
error: unreachable call
   --> compiler/rustc_target/src/asm/mod.rs:314:39
    |
314 |             InlineAsmArch::Nvptx64 => Self::Nvptx(NvptxInlineAsmReg::parse(name)?),
    |                                       ^^^^^^^^^^^ ------------------------------- any code following this expression is unreachable
    |                                       unreachable call
    |
    |
    = note: `-D unreachable-code` implied by `-D warnings`
error: unreachable call
   --> compiler/rustc_target/src/asm/mod.rs:323:37
    |
    |
323 |             InlineAsmArch::SpirV => Self::SpirV(SpirVInlineAsmReg::parse(name)?),
    |                                     ^^^^^^^^^^^ ------------------------------- any code following this expression is unreachable
    |                                     unreachable call

error: unreachable call
   --> compiler/rustc_target/src/asm/mod.rs:325:17
   --> compiler/rustc_target/src/asm/mod.rs:325:17
    |
325 |                 Self::Wasm(WasmInlineAsmReg::parse(name)?)
    |                 ^^^^^^^^^^ ------------------------------ any code following this expression is unreachable
    |                 unreachable call

   Compiling rustc_ast_pretty v0.0.0 (/checkout/compiler/rustc_ast_pretty)
error: could not compile `rustc_target` due to 3 previous errors
