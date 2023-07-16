plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
tidy: Skipping binary file check, read-only filesystem
Checking which error codes lack tests...
* 630 error codes
* highest error code: E0788
tidy error: Stray file with UI testing output: "/checkout/src/test/ui/lifetimes/issue-90170-elision-mismatch-nll.stderr"
Found 0 error(s) in error codes
Done!
* 378 features
some tidy checks failed
