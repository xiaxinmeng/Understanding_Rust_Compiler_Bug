
> rm -r build/x86_64-unknown-linux-gnu/llvm/
> x clippy
Updating only changed submodules
Submodules updated in 0.01 seconds
    Finished dev [unoptimized + debuginfo] target(s) in 0.09s
Checking stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
    Checking core v0.0.0 (/home/joshua/rustc/library/core)
warning: this `if` has identical blocks
  --> library/unwind/build.rs:13:5
   |
13 | /     {
14 | |         // Build the unwinding from libunwind C/C++ source code.
15 | |         llvm_libunwind::compile();
16 | |     } else if target.contains("x86_64-fortanix-unknown-sgx") {
   | |_____^
