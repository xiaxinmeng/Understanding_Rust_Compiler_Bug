Bash
$ cat main.rs
pub extern "stdcall" fn foo() {
  println!("Hello");
}

fn main() {
    foo();
}
$ rustc main.rs --target=powerpc-unknown-linux-gnu
Assertion failed: ((CallConv == CallingConv::C || CallConv == CallingConv::Fast) && "Unknown calling convention!"), function LowerCall_32SVR4, file /Users/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-mac/build/src/llvm/lib/Target/PowerPC/PPCISelLowering.cpp, line 4741.
