plain
Successfully built 4c4c73934f6f
Successfully tagged rust-ci:latest
Built container sha256:4c4c73934f6f1d307eac937ed6ba30911be0f135076478f9404a43db26c0b2a2
Uploading finished image to https://ci-caches.rust-lang.org/docker/93608764ca974412130ba4a304c50b0ea4a89a9b5853d488b55e530021cbeebcb5549b17247c6674f12706902ca4687aae74f60088fb03044e2d4026dd4d59e0
upload failed: - to s3://rust-lang-ci-sccache2/docker/93608764ca974412130ba4a304c50b0ea4a89a9b5853d488b55e530021cbeebcb5549b17247c6674f12706902ca4687aae74f60088fb03044e2d4026dd4d59e0 Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-llvm-12]
---
Testing error-index stage2
doc tests for: /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md


command did not execute successfully: "/checkout/obj/build/bootstrap/debug/rustdoc" "-Wrustdoc::invalid_codeblock_attributes" "-Dwarnings" "-Znormalize-docs" "-Z" "unstable-options" "--test" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md" "--test-args" ""

stdout ----

running 1029 tests
---
test /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0785 (line 16565) ... ok

failures:

---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0660 (line 13692) stdout ----
error: cannot find macro `llvm_asm` in this scope
 --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:13693:1
  |
3 | llvm_asm!("nop" "nop");

error: aborting due to previous error


Some expected error codes were not found: ["E0660"]
---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0661 (line 13706) stdout ----
error: cannot find macro `llvm_asm` in this scope
 --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:13708:1
  |
4 | llvm_asm!("nop" : "r"(a));

error: aborting due to previous error


Some expected error codes were not found: ["E0661"]
---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0662 (line 13722) stdout ----
error: cannot find macro `llvm_asm` in this scope
 --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:13723:1
  |
3 | llvm_asm!("xor %eax, %eax"

error: aborting due to previous error


Some expected error codes were not found: ["E0662"]
---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0663 (line 13740) stdout ----
error: cannot find macro `llvm_asm` in this scope
 --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:13741:1
  |
3 | llvm_asm!("xor %eax, %eax"

error: aborting due to previous error


Some expected error codes were not found: ["E0663"]
---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0664 (line 13757) stdout ----
error: cannot find macro `llvm_asm` in this scope
 --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:13758:1
  |
3 | llvm_asm!("mov $$0x200, %eax"

error: aborting due to previous error


Some expected error codes were not found: ["E0664"]
---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0668 (line 13837) stdout ----
error: cannot find macro `llvm_asm` in this scope
 --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:13843:9
  |
7 |         llvm_asm!("" :"={rax"(rax));

error: aborting due to previous error


Some expected error codes were not found: ["E0668"]
---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0669 (line 13861) stdout ----
error: cannot find macro `llvm_asm` in this scope
 --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:13866:9
  |
6 |         llvm_asm!("" :: "r"("")); // error!

error: aborting due to previous error


Some expected error codes were not found: ["E0669"]
failures:
    /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0660 (line 13692)
    /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0661 (line 13706)
    /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0662 (line 13722)
