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
   Compiling regex v1.3.9
   Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
    Finished release [optimized] target(s) in 9.20s
tidy check
tidy error: Stray file with UI testing output: "/checkout/src/test/ui/issues/issue-8460-const.opt_with_overflow_checks.stderr"
tidy error: Stray file with UI testing output: "/checkout/src/test/ui/issues/issue-8460-const.opt.stderr"
tidy error: Stray file with UI testing output: "/checkout/src/test/ui/issues/issue-8460-const.noopt.stderr"
tidy error: Stray file with UI testing output: "/checkout/src/test/ui/issues/issue-76547.nll.stderr"
tidy error: Stray file with UI testing output: "/checkout/src/test/ui/nll/issue-28848.nll.stderr"
Found 435 error codes
Found 0 error codes with no tests
Done!
some tidy checks failed
