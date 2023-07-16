console
error[E0502]: cannot borrow `writer` as immutable because it is also borrowed as mutable
   --> github.com-1ecc6299db9ec823/tracing-subscriber-0.3.3/src/fmt/format/mod.rs:925:17
    |
925 |                 writer.bold().paint(meta.target()),
    |                 ^^^^^^^^^^^^^ immutable borrow occurs here
    |
   ::: library/core/src/lib.rs:268:13
    |
268 |             $x.__rustc_unstable_auto_ref_mut_helper::<{ $crate::autoref::UnstableMethodSeal }>()
    |             ------------------------------------------------------------------------------------ mutable borrow occurs here
    |
   ::: library/core/src/macros/mod.rs:503:34
    |
503 |                 let result = dst.write_fmt($crate::format_args!($($arg)*));
    |                                  --------- mutable borrow later used by call
