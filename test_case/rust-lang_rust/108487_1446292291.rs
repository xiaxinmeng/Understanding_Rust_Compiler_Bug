plain
      System Firmware Version: VMW71.00V.13989454.B64.1906190538
      OS Loader Version: 540.120.3~22
      Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
      SMC Version (system): 2.8f0
      Serial Number (system): VMmTmlrsGJlu
      Provisioning UDID: 4203018E-580F-C1B5-9525-B745CECA79EB

hw.ncpu: 3
hw.byteorder: 1234
---
    Finished release [optimized] target(s) in 3m 51s
[TIMING] tool::Rustdoc { compiler: Compiler { stage: 2, host: aarch64-apple-darwin } } -- 231.728
Building tool rust-analyzer-proc-macro-srv (stage1:x86_64-apple-darwin -> stage1:aarch64-apple-darwin)
    Updating crates.io index
warning: spurious network error (2 tries remaining): failed to get successful HTTP response from `https://index.crates.io/ro/wa/rowan`, got 503


warning: spurious network error (1 tries remaining): failed to get successful HTTP response from `https://index.crates.io/ro/wa/rowan`, got 503


error: failed to get `rowan` as a dependency of package `syntax v0.0.0 (/Users/runner/work/rust/rust/src/tools/rust-analyzer/crates/syntax)`
Caused by:
  failed to query replaced source registry `crates-io`

Caused by:
Caused by:
  download of ro/wa/rowan failed
Caused by:
Caused by:
  failed to get successful HTTP response from `https://index.crates.io/ro/wa/rowan`, got 503
Build completed unsuccessfully in 1:44:43
