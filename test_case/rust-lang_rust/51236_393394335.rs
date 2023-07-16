
thread 'rustc' panicked at 'Unable to fulfill trait DefId(2/0:885 ~ core[3db6]::marker[0]::Send[0]) for 'list_list::Owned<T>': [FulfillmentError(Obligation(predicate=Binder(ProjectionPredicate(ProjectionTy { substs: Slice([T, ReStatic]), item_def_id: DefId(0/0:1150 ~ lib[8787]::traits[0]::Owned[0]::Reader[0]) }, _)),depth=2),Ambiguity), FulfillmentError(Obligation(predicate=Binder(TraitPredicate(<_ as std::marker::Send>)),depth=2),Ambiguity)]', librustdoc/clean/auto_trait.rs:401:17
note: Run with `RUST_BACKTRACE=1` for a backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.26.1 (827013a31 2018-05-25) running on x86_64-unknown-linux-gnu

error: Could not document `capnp`.

Caused by:
  process didn't exit successfully: `rustdoc --crate-name capnp /home/qwe/.cargo/registry/src/github.com-1ecc6299db9ec823/capnp-0.8.17/src/lib.rs -o /home/qwe/Desktop/upw/test_rustdoc/target/doc -L dependency=/home/qwe/Desktop/upw/test_rustdoc/target/debug/deps --extern byteorder=/home/qwe/Desktop/upw/test_rustdoc/target/debug/deps/libbyteorder-52361c0693377ac0.rmeta` (exit code: 101)
