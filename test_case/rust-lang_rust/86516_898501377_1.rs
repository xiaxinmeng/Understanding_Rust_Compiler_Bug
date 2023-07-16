
 cargo +nightly rustc -- -Zunstable-options --force-warn warnings
   Compiling deleteme v0.1.0 (/home/rylevick/deleteme)
warning: function is never used: `foo`
 --> src/main.rs:7:4
  |
7 | fn foo() {}
  |    ^^^
  |
  = note: `--force-warn dead-code` implied by `--force-warn warnings`

warning: `deleteme` (bin "deleteme") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 0.24s
