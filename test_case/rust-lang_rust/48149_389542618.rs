plain
[01:15:46] ....................................................................................................
[01:15:53] ....................................................................................................
[01:16:01] ..........................................i.........................................................
[01:16:07] ..................i.................................................................................
[01:16:14] .....................................ii.........................F...................................
[01:16:31] ......................................i.............................................................
[01:16:32] ................
[01:16:32] failures:
[01:16:32] 
[01:16:32] 
[01:16:32] ---- [ui] ui/rfc1598-generic-associated-types/parameter_number_and_kind.rs stdout ----
[01:16:32]  diff of stderr:
[01:16:32] 
[01:16:32] - error[E0109]: type parameters are not allowed on this type
[01:16:32] -    |
[01:16:32] -    |
[01:16:32] - LL |     type FOk<T> = Self::E<'static, T>;
[01:16:32] -    |                                    ^ type parameter not allowed
[01:16:32] - 
[01:16:32] 7 error[E0110]: lifetime parameters are not allowed on this type
[01:16:32] 9    |
[01:16:32] 
[01:16:32] 
[01:16:32] 16 LL |     type FErr1 = Self::E<'static, 'static>; // Error
[01:16:32] 17    |                          ^^^^^^^ lifetime parameter not allowed
[01:16:32] 18 
[01:16:32] - error[E0109]: type parameters are not allowed on this type
[01:16:32] -    |
[01:16:32] -    |
[01:16:32] - LL |     type FErr2<T> = Self::E<'static, T, u32>; // Error
[01:16:32] -    |                                      ^ type parameter not allowed
[01:16:32] - 
[01:16:32] 25 error[E0110]: lifetime parameters are not allowed on this type
[01:16:32] 27    |
[01:16:32] 
[01:16:32] 
[01:16:32] 28 LL |     type FErr2<T> = Self::E<'static, T, u32>; // Error
[01:16:32] 29    |                             ^^^^^^^ lifetime parameter not allowed
[01:16:32] - error: aborting due to 5 previous errors
[01:16:32] + error: aborting due to 3 previous errors
[01:16:32] 32 
[01:16:32] - Some errors occurred: E0109, E0110.
---
[01:16:32] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'rfc1598-generic-associated-types/parameter_number_and_kind.rs'
[01:16:32] 
[01:16:32] error: 1 errors occurred comparing output.
[01:16:32] status: exit code: 101
[01:16:32] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc1598-generic-associated-types/parameter_number_and_kind.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc1598-generic-associated-types/parameter_number_and_kind.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc1598-generic-associated-types/parameter_number_and_kind.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[01:16:32] ------------------------------------------
[01:16:32] 
[01:16:32] ------------------------------------------
[01:16:32] stderr:
[01:16:32] stderr:
[01:16:32] ------------------------------------------
[01:16:32] {"message":"lifetime parameters are not allowed on this type","code":{"code":"E0110","explanation":"\nYou tried to give a lifetime parameter to a type which doesn't need it.\nErroneous code example:\n\n