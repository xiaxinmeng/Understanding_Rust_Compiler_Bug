plain
[00:45:36] ....................................................................................................
[00:45:41] ....................................................................................................
[00:45:47] ....................................................................................................
[00:45:53] .........................................i..........................................................
[00:45:59] .................i..................F...............................................................
[00:46:11] ....................................................................................................
[00:46:11] ....................................................................................................
 doesn't implement `std::fmt::Debug`
[00:46:17] 15    |
[00:46:17] 16    = help: the trait `std::fmt::Debug` is not implemented for `no_debug::Bar`
[00:46:17] 17    = note: required by `std::fmt::Debug::fmt`
[00:46:17] 20   --> $DIR/no-debug.rs:21:23
[00:46:17] 21    |
[00:46:17] 21    |
[00:46:17] 22 LL |     println!("{} {}", Foo, Bar);
[00:46:17] -    |                       ^^^ `Foo` cannot be formatted with the default formatter; try using `:?` instead if you are using a format string
[00:46:17] +    |                       ^^^ `Foo` cannot be formatted with the default formatter; try using `{:?}` (or {:#?}) for pretty-print) instead if you are using a format string
[00:46:17] 24    |
[00:46:17] 25    = help: the trait `std::fmt::Display` is not implemented for `Foo`
[00:46:17] 26    = note: required by `std::fmt::Display::fmt`
[00:46:17] 29   --> $DIR/no-debug.rs:21:28
[00:46:17] 30    |
[00:46:17] 30    |
[00:46:17] 31 LL |     println!("{} {}", Foo, Bar);
[00:46:17] -    |                            ^^^ `no_debug::Bar` cannot be formatted with the default formatter; try using `:?` instead if you are using a format string
[00:46:17] +    |                            ^^^ `no_debug::Bar` cannot be formatted with the default formatter; try using `{:?}` (or {:#?}) for pretty-print) instead if you are using a format string
[00:46:17] 33    |
[00:46:17] 34    = help: the trait `std::fmt::Display` is not implemented for `no_debug::Bar`
[00:46:17] 35    = note: required by `std::fmt::Display::fmt`
[00:46:17] 
[00:46:17] The actual stderr differed from the expected stderr.
[00:46:17] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/on-unimplemented/no-debug.stderr
[00:46:17] To update references, run this command from build directory:
[00:46:17] To update references, run this command from build directory:
[00:46:17] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'on-unimplemented/no-debug.rs'
[00:46:17] error: 1 errors occurred comparing output.
[00:46:17] status: exit code: 101
[00:46:17] status: exit code: 101
[00:46:17] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/on-unimplemented/no-debug.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/on-unimplemented/no-debug.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/on-unimplemented/no-debug.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[00:46:17] ------------------------------------------
[00:46:17] 
[00:46:17] ------------------------------------------
[00:46:17] stderr:
[00:46:17] stderr:
[00:46:17] ------------------------------------------
[00:46:17] {"message":"`Foo` doesn't implement `std::fmt::Debug`","code":{"code":"E0277","explanation":"\nYou tried to use a type which doesn't implement some trait in a place which\nexpected that trait. Erroneous code example:\n\n