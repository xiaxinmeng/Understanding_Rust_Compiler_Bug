bash
$ echo 'fn main() {
    let mut v: Vec<u8> = Vec::with_capacity(1); v.extend_from_slice(b"a"); std::mem::forget(v);
}' | rustc -Zdump-mir=all -Zvalidate-mir -Cpanic=abort -
error: internal compiler error: broken MIR in DefId(2:2112 ~ core[44b8]::ops[0]::function[0]::FnOnce[0]::call_once[0]) (input to phase Const) at bb0[0]:
encountered non-callable type Self in `Call` terminator
   --> /home/bjorn/.rustup/toolchains/nightly-2020-06-12-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libcore/ops/function.rs:232:5
    |
232 |     extern "rust-call" fn call_once(self, args: Args) -> Self::Output;
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

[...]
