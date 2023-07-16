
 cargo +nightly rustc -- -Zunstable-options  --force-warn warnings
   Compiling deleteme v0.1.0 (/home/rylevick/deleteme)
warning: extern declarations without an explicit ABI are deprecated
 --> src/main.rs:7:1
  |
7 | extern fn foo() {}
  | ^^^^^^^^^^^^^^^ ABI should be specified here
  |
  = note: `--force-warn missing-abi` implied by `--force-warn warnings`
  = help: the default ABI is C

warning: function is never used: `foo`
 --> src/main.rs:7:11
  |
7 | extern fn foo() {}
  |           ^^^
  |
  = note: `--force-warn dead-code` implied by `--force-warn warnings`

warning: `deleteme` (bin "deleteme") generated 2 warnings
    Finished dev [unoptimized + debuginfo] target(s) in 0.21s
