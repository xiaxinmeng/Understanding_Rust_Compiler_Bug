
 cargo +nightly rustc -- -Zunstable-options  --force-warn warnings -A missing_abi
   Compiling deleteme v0.1.0 (/home/rylevick/deleteme)
warning: extern declarations without an explicit ABI are deprecated
 --> src/main.rs:6:1
  |
6 | extern fn foo() {}
  | ^^^^^^^^^^^^^^^ ABI should be specified here
  |
  = note: `--force-warn missing-abi` implied by `--force-warn warnings`
  = help: the default ABI is C
