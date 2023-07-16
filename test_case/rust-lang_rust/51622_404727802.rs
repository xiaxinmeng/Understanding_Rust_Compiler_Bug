plain
[00:03:38]    Compiling std v0.0.0 (file:///checkout/src/libstd)
[00:03:40]    Compiling rustc_asan v0.0.0 (file:///checkout/src/librustc_asan)
[00:03:41]    Compiling rustc_tsan v0.0.0 (file:///checkout/src/librustc_tsan)
[00:03:41]    Compiling rustc_msan v0.0.0 (file:///checkout/src/librustc_msan)
[00:03:42] error[E0609]: no field `is_iterating` on type `ops::range::RangeInclusive<T>`
[00:03:42]     |
[00:03:42]     |
[00:03:42] 791 |             self.iter.is_iterating = Some(false);
[00:03:42]     |
[00:03:42]     |
[00:03:42]     = note: available fields are: `start`, `end`, `is_empty`
[00:03:42] 
[00:03:42] error[E0609]: no field `is_iterating` on type `ops::range::RangeInclusive<T>`
[00:03:42]     |
[00:03:42]     |
[00:03:42] 797 |             self.iter.is_iterating = Some(n <= self.iter.end);
[00:03:42]     |
[00:03:42]     |
[00:03:42]     = note: available fields are: `start`, `end`, `is_empty`
[00:03:42] 
[00:03:42] error[E0609]: no field `is_iterating` on type `ops::range::RangeInclusive<T>`
[00:03:42]     |
[00:03:42]     |
[00:03:42] 802 |             self.iter.is_iterating = Some(false);
[00:03:42]     |
[00:03:42]     |
[00:03:42]     = note: available fields are: `start`, `end`, `is_empty`
[00:03:42]    Compiling rustc_lsan v0.0.0 (file:///checkout/src/librustc_lsan)
[00:03:44] error: aborting due to 3 previous errors
[00:03:44] 
[00:03:44] For more information about this error, try `rustc --explain E0609`.
