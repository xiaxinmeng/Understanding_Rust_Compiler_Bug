
[frankelbot3:miles|zellij](main)$ git s
On branch main
Your branch is up to date with 'origin/main'.

nothing to commit, working tree clean
[frankelbot3:miles|zellij](main)$ cargo t
   Compiling zellij v0.5.0 (/home/miles/git/zellij)
warning: unused import: `utils::consts::ZELLIJ_IPC_PIPE`
  --> src/common/mod.rs:41:5
   |
41 | use utils::consts::ZELLIJ_IPC_PIPE;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: variant is never constructed: `IpcServer`
   --> src/common/errors.rs:144:5
    |
144 |     IpcServer,
    |     ^^^^^^^^^
    |
    = note: `#[warn(dead_code)]` on by default

warning: variant is never constructed: `IoPath`
  --> src/common/input/config.rs:35:5
   |
35 |     IoPath(io::Error, PathBuf),
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: associated function is never used: `from_yaml`
  --> src/common/input/config.rs:47:12
   |
47 |     pub fn from_yaml(yaml_config: &str) -> ConfigResult {
   |            ^^^^^^^^^

warning: associated function is never used: `new`
  --> src/common/input/config.rs:54:12
   |
54 |     pub fn new(path: &Path) -> ConfigResult {
   |            ^^^

warning: associated function is never used: `from_default_path`
  --> src/common/input/config.rs:68:8
   |
68 |     fn from_default_path() -> ConfigResult {
   |        ^^^^^^^^^^^^^^^^^

warning: function is never used: `no_unbind_unbinds_none`
   --> src/common/input/./unit/keybinds_test.rs:139:4
    |
139 | fn no_unbind_unbinds_none() {
    |    ^^^^^^^^^^^^^^^^^^^^^^

warning: variant is never constructed: `ClosePane`
  --> src/common/screen.rs:47:5
   |
47 |     ClosePane(PaneId),
   |     ^^^^^^^^^^^^^^^^^

warning: variant is never constructed: `Error`
   --> src/common/mod.rs:118:5
    |
118 |     Error(String),
    |     ^^^^^^^^^^^^^

thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `Some(Fingerprint(2727097316613672502, 1351600337527818324))`,
 right: `Some(Fingerprint(17924628540112108216, 17421394458642266263))`: found unstable fingerprints for predicates_of(core[ec89]::clone::Clone): GenericPredicates { parent: None, predicates: [(Binder(TraitPredicate(<Self as std::marker::Sized>), []), /home/miles/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/clone.rs:108:18: 108:23 (#0)), (Binder(TraitPredicate(<Self as std::clone::Clone>), []), /home/miles/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/clone.rs:108:1: 108:23 (#0))] }', /rustc/6df26f897cffb2d86880544bb451c6b5f8509b2d/compiler/rustc_query_system/src/query/plumbing.rs:593:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.53.0-nightly (6df26f897 2021-04-20) running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [predicates_of] computing predicates of `std::clone::Clone`
#1 [param_env] computing normalized predicates of `std::clone::Clone`
end of query stack
warning: 9 warnings emitted

error: could not compile `zellij`

To learn more, run the command again with --verbose.
