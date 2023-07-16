
error[E0514]: found crate `lazy_static` compiled by an incompatible version of rustc
  --> /home/ivan/.cargo/registry/src/github.com-1ecc6299db9ec823/crossbeam-utils-0.8.5/src/sync/sharded_lock.rs:13:5
   |
13 | use lazy_static::lazy_static;
   |     ^^^^^^^^^^^
   |
   = help: please recompile that crate using this compiler (rustc 1.55.0-nightly (32c9b7b09 2021-07-21))
   = note: the following crate versions were found:
           crate `lazy_static` compiled by rustc 1.55.0-nightly (6d820866a 2021-06-29): /home/ivan/src/pickfire/rs/helix/target/debug/deps/liblazy_static-2ed79acc105067d7.rmeta

error: could not compile `crossbeam-utils` due to previous error
warning: build failed, waiting for other jobs to finish...
error[E0514]: found crate `smallvec` compiled by an incompatible version of rustc
  --> /home/ivan/.cargo/registry/src/github.com-1ecc6299db9ec823/parking_lot_core-0.8.3/src/parking_lot.rs:16:5
   |
16 | use smallvec::SmallVec;
   |     ^^^^^^^^
   |
   = help: please recompile that crate using this compiler (rustc 1.55.0-nightly (32c9b7b09 2021-07-21))
   = note: the following crate versions were found:
           crate `smallvec` compiled by rustc 1.55.0-nightly (6d820866a 2021-06-29): /home/ivan/src/pickfire/rs/helix/target/debug/deps/libsmallvec-2a80c8fb8d7317f6.rmeta

error[E0514]: found crate `pin_utils` compiled by an incompatible version of rustc
  --> /home/ivan/.cargo/registry/src/github.com-1ecc6299db9ec823/futures-util-0.3.16/src/lib.rs:34:9
   |
34 | pub use pin_utils::pin_mut;
   |         ^^^^^^^^^
   |
   = help: please recompile that crate using this compiler (rustc 1.55.0-nightly (32c9b7b09 2021-07-21))
   = note: the following crate versions were found:
           crate `pin_utils` compiled by rustc 1.55.0-nightly (6d820866a 2021-06-29): /home/ivan/src/pickfire/rs/helix/target/debug/deps/libpin_utils-849d07e2fcf4214a.rmeta
