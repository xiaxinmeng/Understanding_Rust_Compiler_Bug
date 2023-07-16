
    Checking pixurs v0.1.0 (/Users/mag/prog/pixu.rs)
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', /rustc/25d8a9494ca6d77361e47c1505ecf640b168819e/src/libcore/macros/mod.rs:15:40
stack backtrace:
   0:        0x111291e65 - <unknown>
   1:        0x1112c8ba0 - <unknown>
   2:        0x1112856eb - <unknown>
   3:        0x111296273 - <unknown>
   4:        0x111295f7a - <unknown>
   5:        0x10e6f2bc2 - <unknown>
   6:        0x111296a8c - <unknown>
   7:        0x111296519 - <unknown>
   8:        0x111296419 - <unknown>
   9:        0x1112c224c - <unknown>
  10:        0x1112c21a4 - <unknown>
  11:        0x10efb7eb1 - <unknown>
  12:        0x10efd2b29 - <unknown>
  13:        0x10efd2336 - <unknown>
  14:        0x10efcc64d - <unknown>
  15:        0x10efc93c6 - <unknown>
  16:        0x10f283291 - <unknown>
  17:        0x10efc7620 - <unknown>
  18:        0x10ef52f83 - <unknown>
  19:        0x10f2723da - <unknown>
  20:        0x10f359b05 - <unknown>
  21:        0x10f35e3c0 - <unknown>
  22:        0x10f3515da - <unknown>
  23:        0x10f2c5281 - <unknown>
  24:        0x10efc7fc6 - <unknown>
  25:        0x10f283291 - <unknown>
  26:        0x10efc7620 - <unknown>
  27:        0x10e80ee53 - <unknown>
  28:        0x10e7e558f - <unknown>
  29:        0x10e7a5a72 - <unknown>
  30:        0x10e801489 - <unknown>
  31:        0x10e7e511e - <unknown>
  32:        0x10e818e13 - <unknown>
  33:        0x10e6aaa7f - <unknown>
  34:        0x10e6c812b - <unknown>
  35:        0x10e6fd67a - <unknown>
  36:        0x10e6bfd1c - <unknown>
  37:        0x10e76d54b - <unknown>
  38:        0x10e6bfb3b - <unknown>
  39:        0x10e6f62b7 - <unknown>
  40:        0x10e6ade34 - <unknown>
  41:        0x10e6a7712 - <unknown>
  42:        0x10e6c9455 - <unknown>
  43:        0x10e6a853c - <unknown>
  44:        0x1112a61ff - <unknown>
  45:        0x10e6c1057 - <unknown>
  46:        0x111277cae - <unknown>
  47:        0x1112a4f3e - <unknown>
  48:     0x7fff59cbd2eb - <unknown>
  49:     0x7fff59cc0249 - <unknown>

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.41.0-nightly (25d8a9494 2019-11-29) running on x86_64-apple-darwin

note: compiler flags: -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [mir_borrowck] processing `site::auth::initiate_auth::InitiateAuth::<S>::try_post::{{closure}}#0`
#1 [mir_borrowck] processing `site::auth::initiate_auth::InitiateAuth::<S>::try_post`
#2 [analysis] running analysis passes on this crate
end of query stack
error: could not compile `pixurs`.

To learn more, run the command again with --verbose.
