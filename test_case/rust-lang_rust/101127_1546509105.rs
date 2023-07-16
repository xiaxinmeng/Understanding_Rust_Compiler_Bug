sh
$ rustc +1.66 --print asdf
error: unknown print request `asdf`
$ rustc +1.67 --print asdf
error: unknown print request `asdf`. Valid print requests are: `crate-name`, `file-names`, `sysroot`, `target-libdir`, `cfg`, `calling-conventions`, `target-list`, `target-cpus`, `target-features`, `relocation-models`, `code-models`, `tls-models`, `native-static-libs`, `stack-protector-strategies`, `target-spec-json`, `link-args`, `split-debuginfo`
