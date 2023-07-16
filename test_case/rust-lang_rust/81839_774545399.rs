
$ git clone https://github.com/teloxide/teloxide.git
Cloning into 'teloxide'...
remote: Enumerating objects: 114, done.
remote: Counting objects: 100% (114/114), done.
remote: Compressing objects: 100% (77/77), done.
remote: Total 14861 (delta 50), reused 66 (delta 37), pack-reused 14747
Receiving objects: 100% (14861/14861), done.
Resolving deltas: 100% (10379/10379), done.

$ cd teloxide/examples/sqlite_remember_bot/

# >>>> edit src/transitions.rs
# >>>> changing the file with the diff in the first comment above

$ cargo build
    Updating crates.io index
    Updating git repository `https://github.com/teloxide/teloxide-macros`
   Compiling proc-macro2 v1.0.24

...

   Compiling teloxide v0.3.4 (/tmp/teloxide)
   Compiling sqlite_remember_bot v0.1.0 (/tmp/teloxide/examples/sqlite_remember_bot)
error[E0308]: mismatched types
  --> src/transitions.rs:25:9
   |
24 |     match ans {
   |           --- this expression has type `std::string::String`
25 |         "abc" => {
   |         ^^^^^ expected struct `std::string::String`, found `&str`

thread 'rustc' panicked at 'DefId::expect_local: `DefId(22:1034 ~ teloxide[eccd]::dispatching::update_with_cx::{impl#1}::answer_str::{opaque#0})` isn't local', /rustc/e1884a8e3c3e813aada8254edfa120e85bf5ffca/compiler/rustc_span/src/def_id.rs:175:43
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.49.0 (e1884a8e3 2020-12-29) running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [typeck] type-checking `transitions::<impl teloxide::dispatching::dialogue::Subtransition for states::HaveNumberState>::react::have_number`
#1 [mir_built] building MIR for `transitions::<impl teloxide::dispatching::dialogue::Subtransition for states::HaveNumberState>::react::have_number`
end of query stack
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
error: could not compile `sqlite_remember_bot`

