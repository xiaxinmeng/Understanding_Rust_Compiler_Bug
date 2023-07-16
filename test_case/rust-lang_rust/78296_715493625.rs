plain
   Compiling profiler_builtins v0.0.0 (/checkout/library/profiler_builtins)
error: unused doc comment
    --> library/core/src/ptr/mod.rs:1148:5
     |
1148 | /     /// Calculate multiplicative modular inverse of `x` modulo `m`.
1149 | |     ///
1150 | |     /// This implementation is tailored for `align_offset` and has following preconditions:
...    |
1154 | |     ///
1154 | |     ///
1155 | |     /// Implementation of this function shall not panic. Ever.
     | |______________________________________________________________^
1156 |       #[inline]
1157 | /     unsafe fn mod_inv(x: usize, m: usize) -> usize {
1158 | |         /// Multiplicative modular inverse table modulo 2⁴ = 16.
1159 | |         ///
1160 | |         /// Note, that this table does not contain values where inverse does not exist (i.e., for
1194 | |         }
1195 | |     }
1195 | |     }
     | |_____- rustdoc does not generate documentation for inner items
     |
     = note: `-D unused-doc-comments` implied by `-D warnings`
error: unused doc comment
    --> library/core/src/ptr/mod.rs:1158:9
     |
     |
1158 | /         /// Multiplicative modular inverse table modulo 2⁴ = 16.
1159 | |         ///
1160 | |         /// Note, that this table does not contain values where inverse does not exist (i.e., for
1161 | |         /// `0⁻¹ mod 16`, `2⁻¹ mod 16`, etc.)
     | |_____________________________________________^
1162 |           const INV_TABLE_MOD_16: [u8; 8] = [1, 11, 13, 7, 9, 3, 5, 15];
     |           -------------------------------------------------------------- rustdoc does not generate documentation for inner items
error: unused doc comment
    --> library/core/src/ptr/mod.rs:1163:9
     |
     |
1163 |         /// Modulo for which the `INV_TABLE_MOD_16` is intended.
     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
1164 |         const INV_TABLE_MOD: usize = 16;
     |         -------------------------------- rustdoc does not generate documentation for inner items
error: unused doc comment
    --> library/core/src/ptr/mod.rs:1165:9
     |
     |
1165 |         /// INV_TABLE_MOD²
     |         ^^^^^^^^^^^^^^^^^^
1166 |         const INV_TABLE_MOD_SQUARED: usize = INV_TABLE_MOD * INV_TABLE_MOD;
     |         ------------------------------------------------------------------- rustdoc does not generate documentation for inner items
error: unused doc comment
    --> library/core/src/time.rs:990:9
     |
     |
990  | /         /// Formats a floating point number in decimal notation.
991  | |         ///
992  | |         /// The number is given as the `integer_part` and a fractional part.
993  | |         /// The value of the fractional part is `fractional_part / divisor`. So
...    |
998  | |         /// of 10, everything else doesn't make sense. `fractional_part` has
999  | |         /// to be less than `10 * divisor`!
1000 | /         fn fmt_decimal(
1000 | /         fn fmt_decimal(
1001 | |             f: &mut fmt::Formatter<'_>,
1002 | |             mut integer_part: u64,
1003 | |             mut fractional_part: u32,
1077 | |             }
1078 | |         }
1078 | |         }
     | |_________- rustdoc does not generate documentation for inner items
error: aborting due to 5 previous errors

error: could not compile `core`


To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace profiler compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu --include-default-paths src/tools/build-manifest
Build completed unsuccessfully in 0:14:20
== clock drift check ==
  local time: Fri Oct 23 18:04:05 UTC 2020
  local time: Fri Oct 23 18:04:05 UTC 2020
  network time: Thu, 22 Oct 2020 18:36:22 GMT
== end clock drift check ==
##[error]Process completed with exit code 1.
Terminate orphan process: pid (3385) (python)
