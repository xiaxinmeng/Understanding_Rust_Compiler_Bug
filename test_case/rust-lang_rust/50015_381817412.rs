plain
[00:00:50] configure: rust.quiet-tests     := True
---
[00:03:25] error[E0597]: `s` does not live long enough
[00:03:25]     --> liballoc/string.rs:1229:51
[00:03:25]      |
[00:03:25] 1229 |             let mut searcher = pat.into_searcher(&s);
[00:03:25]      |                                                   ^ borrowed value does not live long enough
[00:03:25] ...
[00:03:25] 1243 |         }
[00:03:25]      |         - borrowed value only lives until here
[00:03:25]      |
[00:03:25] note: borrowed value must be valid for the lifetime 'a as defined on the method body at 1217:5...
[00:03:25]     --> liballoc/string.rs:1217:5
[00:03:25]      |
[00:03:25] 1217 | /     pub fn remove_matches<'a, P>(&'a mut self, pat: P)
[00:03:25] 1218 | |     where
[00:03:25] 1219 | |         P: Pattern<'a>,
[00:03:25] 1220 | |     {
[00:03:25] ...    |
[00:03:25] 1243 | |         }
[00:03:25] 1244 | |     }
[00:03:25]      | |_____^
[00:03:25]
[00:03:25] error[E0382]: use of moved value: `pat`
[00:03:25]     --> liballoc/string.rs:1229:32
[00:03:25]      |
[00:03:25] 1229 |             let mut searcher = pat.into_searcher(&s);
[00:03:25]      |                                ^^^ value moved here in previous iteration of loop
[00:03:25]      |
[00:03:25]      = note: move occurs because `pat` has type `P`, which does not implement the `Copy` trait
[00:03:25]
[00:03:25] error: aborting due to 2 previous errors
[00:03:25]
[00:03:25] Some errors occurred: E0382, E0597.
[00:03:25] For more information about an error, try `rustc --explain E0382`.
[00:03:25] error: Could not compile `alloc`.
[00:03:25]
[00:03:25] Caused by:
[00:03:25]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name alloc liballoc/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=d3eb5884eb70b0b8 -C extra-filename=-d3eb5884eb70b0b8 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-a2d7f090df844ebf.rlib --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-1b8789e893adb899.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/build/compiler_builtins-6789d0ad544f7553/out` (exit code: 101)
---
$ dmesg | grep -i kill
[   10.758087] init: failsafe main process (1096) killed by TERM signal
