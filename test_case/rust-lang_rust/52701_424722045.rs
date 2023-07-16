
% cat issue-52701.rs
fn rec() -> impl Fn() { rec() }

fn main() {
    let f = rec();
}
% rustup override set nightly-2018-08-19-x86_64-unknown-linux-gnu
info: using existing install for 'nightly-2018-08-19-x86_64-unknown-linux-gnu'
info: override toolchain for '/home/pnkfelix/Dev/Mozilla/issue52701' set to 'nightly-2018-08-19-x86_64-unknown-linux-gnu'

  nightly-2018-08-19-x86_64-unknown-linux-gnu unchanged - rustc 1.30.0-nightly (33b923fd4 2018-08-18)

% rustc --version
rustc 1.30.0-nightly (33b923fd4 2018-08-18)
% rustc issue-52701.rs
warning: unused variable: `f`
 --> issue-52701.rs:4:9
  |
4 |     let f = rec();
  |         ^ help: consider using `_f` instead
  |
  = note: #[warn(unused_variables)] on by default

warning: function cannot return without recurring
 --> issue-52701.rs:1:1
  |
1 | fn rec() -> impl Fn() { rec() }
  | ^^^^^^^^^^^^^^^^^^^^^   ----- recursive call site
  | |
  | cannot return without recurring
  |
  = note: #[warn(unconditional_recursion)] on by default
  = help: a `loop` may express intention better if this is on purpose

error: internal compiler error: librustc/traits/query/normalize.rs:124: infinite recursion generic_ty: impl std::ops::Fn<()>, substs: [], concrete_ty: impl std::ops::Fn<()>, ty: impl std::ops::Fn<()>

thread 'main' panicked at 'Box<Any>', librustc_errors/lib.rs:579:9
note: Run with `RUST_BACKTRACE=1` for a backtrace.
error: aborting due to previous error


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.30.0-nightly (33b923fd4 2018-08-18) running on x86_64-unknown-linux-gnu

% rustup override set nightly-2018-08-24-x86_64-unknown-linux-gnu
info: using existing install for 'nightly-2018-08-24-x86_64-unknown-linux-gnu'
info: override toolchain for '/home/pnkfelix/Dev/Mozilla/issue52701' set to 'nightly-2018-08-24-x86_64-unknown-linux-gnu'

  nightly-2018-08-24-x86_64-unknown-linux-gnu unchanged - rustc 1.30.0-nightly (63d66494a 2018-08-23)

% rustc --version
rustc 1.30.0-nightly (63d66494a 2018-08-23)
% rustc issue-52701.rs
error[E0391]: cycle detected when processing `rec::{{impl-Trait}}`
 --> issue-52701.rs:1:13
  |
1 | fn rec() -> impl Fn() { rec() }
  |             ^^^^^^^^^
  |
note: ...which requires processing `rec`...
 --> issue-52701.rs:1:23
  |
1 | fn rec() -> impl Fn() { rec() }
  |                       ^^^^^^^^^
  = note: ...which again requires processing `rec::{{impl-Trait}}`, completing the cycle

error: aborting due to previous error

For more information about this error, try `rustc --explain E0391`.
%
