plain
[command]/usr/bin/git submodule foreach --recursive git config --local --name-only --get-regexp 'http\.https\:\/\/github\.com\/\.extraheader' && git config --local --unset-all 'http.https://github.com/.extraheader' || :
[command]/usr/bin/git config --local http.https://github.com/.extraheader AUTHORIZATION: basic ***
##[endgroup]
##[group]Fetching the repository
[command]/usr/bin/git -c protocol.version=2 fetch --no-tags --prune --progress --no-recurse-submodules --depth=2 origin +4d1c17e616ec495f01eaccd66351212c55393ce3:refs/remotes/pull/75384/merge
---
   Compiling libc v0.2.79
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.36
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: internal compiler error: compiler/rustc_middle/src/hir/map/collector.rs:251:17: inconsistent DepNode at `"library/core/src/num/f32.rs:636:36: 636:39"` for `::f32::{impl#0}::to_int_unchecked::Int`: current_dep_node_owner=::f32::{impl#0}::to_int_unchecked (DefId(0:136 ~ core[c848]::f32::{impl#0}::to_int_unchecked)), hir_id.owner=::f32::{impl#0}::to_int_unchecked::Int (DefId(0:137 ~ core[c848]::f32::{impl#0}::to_int_unchecked::Int))
    |
    |
636 |     pub unsafe fn to_int_unchecked<Int>(self) -> Int

thread 'rustc' panicked at 'Box<Any>', /checkout/compiler/rustc_errors/src/lib.rs:904:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md
note: rustc 1.50.0-nightly (4d1c17e61 2020-11-13) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z macro-backtrace -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -C overflow-checks=off -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes --crate-type lib
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [index_hir] index HIR
#1 [hir_owner] HIR owner of `{misc#0}`
end of query stack

error: could not compile `core`

To learn more, run the command again with --verbose.
