
   Compiling playground v0.0.1 (file:///playground)
error[E0401]: can't use type parameters from outer function
  --> src/main.rs:24:36
   |
22 | impl<TConfig: NodeConfig> NeuralNetwork<TConfig> {
   |      ------- type variable from outer function
23 |     pub fn new(id: u32) -> Self {
   |            --- try adding a local type parameter in this method instead
24 |         const LAYER_COUNT: usize = TConfig::LAYER_DEPTH + 2;
   |                                    ^^^^^^^^^^^^^^^^^^^^ use of type variable from outer function

error: internal compiler error: librustc/traits/trans/mod.rs:63: Encountered ambiguity selecting `Binder(<[type error] as NodeConfig>)` during trans, presuming due to overflow

thread 'main' panicked at 'Box<Any>', librustc_errors/lib.rs:554:9
note: Run with `RUST_BACKTRACE=1` for a backtrace.
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0401`.

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.27.0-nightly (ac5c0848d 2018-05-14) running on x86_64-unknown-linux-gnu

note: compiler flags: -C codegen-units=1 -C debuginfo=2 --crate-type bin

note: some of the compiler flags provided by cargo are hidden

error: Could not compile `playground`.

To learn more, run the command again with --verbose.
