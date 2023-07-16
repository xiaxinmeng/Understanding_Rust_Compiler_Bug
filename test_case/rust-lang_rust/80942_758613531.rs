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
    Finished release [optimized] target(s) in 7.68s
tidy check
tidy error: Stray file with UI testing output: "/checkout/src/test/ui/lifetime_starts_expressions.stderr"
tidy error: Stray file with UI testing output: "/checkout/src/test/ui/suggestions/issue-65284-suggest-generic-trait-bound.stderr"
tidy error: Stray file with UI testing output: "/checkout/src/test/ui/lint/issue-17718-const-naming.stderr"
tidy error: Stray file with UI testing output: "/checkout/src/test/ui/lint/issue-33140-traitobject-crate.stderr"
tidy error: Stray file with UI testing output: "/checkout/src/test/ui/lint/expr_attr_paren_order.stderr"
tidy error: Stray file with UI testing output: "/checkout/src/test/ui/lint/trivial_casts.stderr"
tidy error: Stray file with UI testing output: "/checkout/src/test/ui/feature-gates/trace_macros-gate.stderr"
tidy error: Stray file with UI testing output: "/checkout/src/test/ui/parser/issue-43196.stderr"
tidy error: Stray file with UI testing output: "/checkout/src/test/ui/parser/issue-45296.stderr"
tidy error: Stray file with UI testing output: "/checkout/src/test/ui/parser/issue-20616-1.stderr"
tidy error: Stray file with UI testing output: "/checkout/src/test/ui/parser/issue-72373.stderr"
tidy error: Stray file with UI testing output: "/checkout/src/test/ui/parser/issue-20616-2.stderr"
tidy error: Stray file with UI testing output: "/checkout/src/test/ui/parser/issue-76597.stderr"
tidy error: Stray file with UI testing output: "/checkout/src/test/ui/parser/issue-52496.stderr"
tidy error: Stray file with UI testing output: "/checkout/src/test/ui/parser/issue-58856-2.stderr"
tidy error: Stray file with UI testing output: "/checkout/src/test/ui/parser/priv-in-bad-locations.stderr"
tidy error: Stray file with UI testing output: "/checkout/src/test/ui/parser/issue-46186.stderr"
tidy error: Stray file with UI testing output: "/checkout/src/test/ui/consts/issue-36163.stderr"
tidy error: Stray file with UI testing output: "/checkout/src/test/ui/rustc-args-required-const.stderr"
tidy error: Stray file with UI testing output: "/checkout/src/test/ui/issues/issue-65673.stderr"
tidy error: Stray file with UI testing output: "/checkout/src/test/ui/issues/issue-44021.stderr"
tidy error: Stray file with UI testing output: "/checkout/src/test/ui/issues/issue-8460-const.opt_with_overflow_checks.stderr"
tidy error: Stray file with UI testing output: "/checkout/src/test/ui/issues/issue-76547.stderr"
tidy error: Stray file with UI testing output: "/checkout/src/test/ui/issues/issue-65634-raw-ident-suggestion.stderr"
tidy error: Stray file with UI testing output: "/checkout/src/test/ui/issues/issue-29161.stderr"
tidy error: Stray file with UI testing output: "/checkout/src/test/ui/issues/issue-12470.stderr"
tidy error: Stray file with UI testing output: "/checkout/src/test/ui/issues/issue-28113.stderr"
tidy error: Stray file with UI testing output: "/checkout/src/test/ui/issues/issue-45199.stderr"
tidy error: Stray file with UI testing output: "/checkout/src/test/ui/issues/issue-55511.stderr"
tidy error: Stray file with UI testing output: "/checkout/src/test/ui/issues/issue-8460-const.opt.stderr"
tidy error: Stray file with UI testing output: "/checkout/src/test/ui/issues/issue-32829-2.stderr"
tidy error: Stray file with UI testing output: "/checkout/src/test/ui/issues/issue-27282-move-match-input-into-guard.stderr"
tidy error: Stray file with UI testing output: "/checkout/src/test/ui/issues/issue-17718-const-borrow.stderr"
tidy error: Stray file with UI testing output: "/checkout/src/test/ui/issues/issue-24267-flow-exit.stderr"
tidy error: Stray file with UI testing output: "/checkout/src/test/ui/issues/issue-27282-mutate-before-diverging-arm-2.stderr"
tidy error: Stray file with UI testing output: "/checkout/src/test/ui/issues/issue-8460-const.noopt.stderr"
tidy error: Stray file with UI testing output: "/checkout/src/test/ui/issues/issue-20801.stderr"
tidy error: Stray file with UI testing output: "/checkout/src/test/ui/issues/issue-43398.stderr"
tidy error: Stray file with UI testing output: "/checkout/src/test/ui/issues/issue-30079.stderr"
tidy error: Stray file with UI testing output: "/checkout/src/test/ui/issues/issue-46471.stderr"
tidy error: Stray file with UI testing output: "/checkout/src/test/ui/issues/issue-25793.stderr"
tidy error: Stray file with UI testing output: "/checkout/src/test/ui/issues/issue-76547.nll.stderr"
tidy error: Stray file with UI testing output: "/checkout/src/test/ui/issues/issue-45087-unreachable-unsafe.stderr"
tidy error: Stray file with UI testing output: "/checkout/src/test/ui/issues/issue-27282-reborrow-ref-mut-in-guard.stderr"
tidy error: Stray file with UI testing output: "/checkout/src/test/ui/llvm-asm/inline-asm-bad-constraint.stderr"
tidy error: Stray file with UI testing output: "/checkout/src/test/ui/reachable/unreachable-code-ret.stderr"
tidy error: Stray file with UI testing output: "/checkout/src/test/ui/point-to-type-err-cause-on-impl-trait-return.stderr"
tidy error: Stray file with UI testing output: "/checkout/src/test/ui/did_you_mean/issue-38940.stderr"
tidy error: Stray file with UI testing output: "/checkout/src/test/ui/pattern/issue-67037-pat-tup-scrut-ty-diff-less-fields.stderr"
tidy error: Stray file with UI testing output: "/checkout/src/test/ui/pattern/issue-30302.stderr"
tidy error: Stray file with UI testing output: "/checkout/src/test/ui/nll/issue-28848.nll.stderr"
tidy error: Stray file with UI testing output: "/checkout/src/test/ui/nll/issue-17545.stderr"
Found 435 error codes
Found 0 error codes with no tests
Done!
some tidy checks failed
