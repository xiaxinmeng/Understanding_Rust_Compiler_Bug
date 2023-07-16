plain
[00:00:57] configure: rust.quiet-tests     := True
---
[00:42:55] ...............................................................................i....................
[00:43:00] ......................i.............................................................................
[00:43:04] ....................................................................................................
[00:43:08] ....................................................................................................
[00:43:11] ....................................................................................................
[00:43:15] ..........................................................................F.........................
[00:43:21] ....................................................................................................
[00:43:27] ....................................................................................................
[00:43:33] ....................................................................................................
[00:43:40] ..................i...........................................................................i.....
[00:43:46] ....................................................................................................
[00:43:52] ........ii..........................................................................................
---
[00:44:03] + error[E0597]: `*cell` does not live long enough
[00:44:03] +   --> $DIR/dropck.rs:19:40
[00:44:03] +    |
[00:44:03] + LL |     let ref_ = Box::leak(Box::new(Some(cell.borrow_mut())));
[00:44:03] +    |                                        ^^^^ borrowed value does not live long enough
[00:44:03] + ...
[00:44:03] + LL | }
[00:44:03] +    | - `*cell` dropped here while still borrowed
[00:44:03] +    |
[00:44:03] +    = note: values in a scope are dropped in the opposite order they are created
[00:44:03] +
[00:44:03] 1 error[E0597]: `ref_` does not live long enough
[00:44:03] 2   --> $DIR/dropck.rs:23:18
[00:44:03] 3    |
[00:44:03]
[00:44:03] 12    |
[00:44:03] 13    = note: values in a scope are dropped in the opposite order they are created
---
[00:44:03] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'generator/dropck.rs'
[00:44:03]
[00:44:03] error: 1 errors occurred comparing output.
[00:44:03] status: exit code: 101
[00:44:03] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generator/dropck.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/dropck.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/dropck.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[00:44:03] stdout:
[00:44:03] ------------------------------------------
[00:44:03]
[00:44:03] ------------------------------------------
[00:44:03] stderr:
[00:44:03] -----------------------are created","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0597]: `*cell` does not live long enough\n  --> /checkout/src/test/ui/generator/dropck.rs:19:40\n   |\nLL |     let ref_ = Box::leak(Box::new(Some(cell.borrow_mut())));\n   |                                        ^^^^ borrowed value does not live long enough\n...\nLL | }\n   | - `*cell` dropped here while still borrowed\n   |\n   = note: values in a scope are dropped in the opposite order they are created\n\n"}
[00:44:03] {"message":"`ref_` does not live long enough","code":{"code":"E0597","explanation":"\nThis error occurs because a borrow was made inside a variable which has a\ngreater lifetime than the borrowed one.\n\nExample of erroneous code:\n\n