bash
warning: field is never used: `logger`
   --> /home/chris/ext/rust-lightning/lightning/src/chain/chaininterface.rs:303:2
    |
303 |     logger: Arc<Logger>,
    |     ^^^^^^^^^^^^^^^^^^^
    |
    = note: `#[warn(dead_code)]` on by default

warning: field is never used: `logger`
   --> /home/chris/ext/rust-lightning/lightning/src/chain/keysinterface.rs:295:2
    |
295 |     logger: Arc<Logger>,
    |     ^^^^^^^^^^^^^^^^^^^

warning: variant is never constructed: `Watchtower`
   --> /home/chris/ext/rust-lightning/lightning/src/ln/channelmonitor.rs:351:2
    |
351 |       Watchtower {
    |  _____^
352 | |         revocation_base_key: PublicKey,
353 | |         htlc_base_key: PublicKey,
354 | |     }
    | |_____^

   Compiling futures-macro v0.3.1
   Compiling tokio-util v0.2.0
   Compiling pin-project v0.4.8
   Compiling lightning-net-tokio v0.0.3 (/home/chris/ext/rust-lightning/lightning-net-tokio)
warning: unused import: `lightning::ln::peer_handler::SocketDescriptor as LnSocketTrait`
 --> /home/chris/ext/rust-lightning/lightning-net-tokio/src/lib.rs:9:5
  |
9 | use lightning::ln::peer_handler::SocketDescriptor as LnSocketTrait;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: unused `std::result::Result` that must be used
   --> /home/chris/ext/rust-lightning/lightning-net-tokio/src/lib.rs:174:4
    |
174 |             tokio::spawn(Self::schedule_read(peer_manager, us, reader, receiver)).await;
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: `#[warn(unused_must_use)]` on by default
    = note: this `Result` may be an `Err` variant, which should be handled

warning: unused `std::result::Result` that must be used
   --> /home/chris/ext/rust-lightning/lightning-net-tokio/src/lib.rs:195:4
    |
195 |             tokio::spawn(Self::schedule_read(peer_manager, us, reader, receiver)).await;
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: this `Result` may be an `Err` variant, which should be handled

   Compiling futures-util v0.3.1
   Compiling h2 v0.2.1
   Compiling hyper v0.13.2
error[E0658]: procedural macros cannot expand to macro definitions
   --> src/main.rs:326:1
    |
326 | #[tokio::main]
    | ^^^^^^^^^^^^^^
    |
    = note: for more information, see https://github.com/rust-lang/rust/issues/54727
    = help: add `#![feature(proc_macro_hygiene)]` to the crate attributes to enable

thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', src/libcore/option.rs:378:21
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.40.0-nightly (09868a56c 2019-10-06) running on x86_64-unknown-linux-gnu

note: compiler flags: -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

error: aborting due to previous error

For more information about this error, try `rustc --explain E0658`.
error: could not compile `rust-lightning-bitcoinrpc`.

To learn more, run the command again with --verbose.


09868a56c95f7bc7b6ee3ab7611e3ca551031dbd finished with exit code Some(101).
please select an action to take:
tested 09868a56c95f7bc7b6ee3ab7611e3ca551031dbd, got Yes
uninstalling 09868a56c95f7bc7b6ee3ab7611e3ca551031dbd
searched toolchains 421bd77f42c2fe8a2596dbcc1580ec97fb89009f through f3c9cece7b6829e6fd7854a1aee6a1619a81a38c
regression in 09868a56c95f7bc7b6ee3ab7611e3ca551031dbd

