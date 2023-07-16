plain
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
   Compiling regex v1.4.3
   Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
    Finished release [optimized] target(s) in 8.97s
tidy check
tidy error: Stray file with UI testing output: "/checkout/src/test/ui/suggestions/const-fn-in-vec.stderr"
Found 435 error codes
Found 0 error codes with no tests
Done!
some tidy checks failed
