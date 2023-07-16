plain
Updating files:  98% (31108/31742)
Updating files:  99% (31425/31742)
Updating files: 100% (31742/31742)
Updating files: 100% (31742/31742), done.
Switched to a new branch 'try'
Branch 'try' set up to track remote branch 'try' from 'origin'.
[command]/usr/local/bin/git log -1 --format='%H'
'4ea8b01f08bbc8c92530a9d9045517a287a84875'
##[group]Run src/ci/scripts/setup-environment.sh
src/ci/scripts/setup-environment.sh
---
      Memory: 14 GB
      System Firmware Version: VMW71.00V.13989454.B64.1906190538
      Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
      SMC Version (system): 2.8f0
      Serial Number (system): VM3SOqVGSuXU
      Provisioning UDID: 4203018E-580F-C1B5-9525-B745CECA79EB

hw.ncpu: 3
hw.byteorder: 1234
---
pkgbuild: Adding top-level postinstall script
pkgbuild: Wrote package to /Users/runner/work/rust/rust/build/tmp/dist/combined-tarball/pkg/rustc.pkg
pkgbuild: Adding top-level postinstall script
pkgbuild: Wrote package to /Users/runner/work/rust/rust/build/tmp/dist/combined-tarball/pkg/cargo.pkg
thread 'main' panicked at 'could not read dir "/Users/runner/work/rust/rust/build/tmp/tarball/rust/aarch64-apple-darwin/rust-docs-nightly-aarch64-apple-darwin": Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/bootstrap/lib.rs:1466:25
Build completed unsuccessfully in 2:15:17
