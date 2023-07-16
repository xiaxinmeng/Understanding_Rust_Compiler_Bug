plain

---- [ui] src/test/ui/mismatched_types/ref-pat-suggestions.rs stdout ----
diff of fixed:

13     let _: fn(&u32) = |&_a| (); //~ ERROR mismatched types
14     let _: fn(&mut u32) = |&mut _a| (); //~ ERROR mismatched types
15     let _: fn(&u32) = |&_a| (); //~ ERROR mismatched types
-     let _: fn(&mut u32) = |&mut _a| (); //~ ERROR mismatched types   
+     let _: fn(&mut u32) = |&mut _a| (); //~ ERROR mismatched types
17 
18     let _ = |_a: &u32| (); //~ ERROR mismatched types
19     let _ = |_a: &mut u32| (); //~ ERROR mismatched types

The actual fixed differed from the expected fixed.
The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/ref-pat-suggestions/ref-pat-suggestions.fixed
To only update this specific test, also pass `--test-args mismatched_types/ref-pat-suggestions.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/mismatched_types/ref-pat-suggestions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/ref-pat-suggestions" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/ref-pat-suggestions/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/mismatched_types/ref-pat-suggestions.rs:3:8
   |
   |
LL | fn _f0(&_a: u32) {} //~ ERROR mismatched types
   |        ^^^  --- expected due to this
   |        expected `u32`, found reference
   |
   = note:   expected type `u32`
           found reference `&_`
           found reference `&_`
help: to take parameter `_a` by reference, move `&` to the type
   |
LL - fn _f0(&_a: u32) {} //~ ERROR mismatched types
LL + fn _f0(_a: &u32) {} //~ ERROR mismatched types

error[E0308]: mismatched types
  --> /checkout/src/test/ui/mismatched_types/ref-pat-suggestions.rs:4:8
   |
   |
LL | fn _f1(&mut _a: u32) {} //~ ERROR mismatched types
   |        ^^^^^^^  --- expected due to this
   |        expected `u32`, found `&mut _`
   |
   = note:           expected type `u32`
           found mutable reference `&mut _`
           found mutable reference `&mut _`
help: to take parameter `_a` by reference, move `&mut` to the type
   |
LL - fn _f1(&mut _a: u32) {} //~ ERROR mismatched types
LL + fn _f1(_a: &mut u32) {} //~ ERROR mismatched types

error[E0308]: mismatched types
  --> /checkout/src/test/ui/mismatched_types/ref-pat-suggestions.rs:5:9
   |
   |
LL | fn _f2(&&_a: &u32) {} //~ ERROR mismatched types
   |         ^^^  ---- expected due to this
   |         expected `u32`, found reference
   |
   = note:   expected type `u32`
           found reference `&_`
           found reference `&_`
help: consider removing `&` from the pattern
   |
LL - fn _f2(&&_a: &u32) {} //~ ERROR mismatched types
LL + fn _f2(&_a: &u32) {} //~ ERROR mismatched types

error[E0308]: mismatched types
  --> /checkout/src/test/ui/mismatched_types/ref-pat-suggestions.rs:6:13
   |
   |
LL | fn _f3(&mut &_a: &mut u32) {} //~ ERROR mismatched types
   |             ^^^  -------- expected due to this
   |             expected `u32`, found reference
   |
   = note:   expected type `u32`
           found reference `&_`
           found reference `&_`
help: consider removing `&` from the pattern
   |
LL - fn _f3(&mut &_a: &mut u32) {} //~ ERROR mismatched types
LL + fn _f3(&mut _a: &mut u32) {} //~ ERROR mismatched types

error[E0308]: mismatched types
  --> /checkout/src/test/ui/mismatched_types/ref-pat-suggestions.rs:7:9
   |
   |
LL | fn _f4(&&mut _a: &u32) {} //~ ERROR mismatched types
   |         ^^^^^^^  ---- expected due to this
   |         expected `u32`, found `&mut _`
   |
   = note:           expected type `u32`
           found mutable reference `&mut _`
           found mutable reference `&mut _`
help: consider removing `&mut` from the pattern
   |
LL - fn _f4(&&mut _a: &u32) {} //~ ERROR mismatched types
LL + fn _f4(&_a: &u32) {} //~ ERROR mismatched types

error[E0308]: mismatched types
  --> /checkout/src/test/ui/mismatched_types/ref-pat-suggestions.rs:8:13
   |
   |
LL | fn _f5(&mut &mut _a: &mut u32) {} //~ ERROR mismatched types
   |             ^^^^^^^  -------- expected due to this
   |             expected `u32`, found `&mut _`
   |
   = note:           expected type `u32`
           found mutable reference `&mut _`
           found mutable reference `&mut _`
help: consider removing `&mut` from the pattern
   |
LL - fn _f5(&mut &mut _a: &mut u32) {} //~ ERROR mismatched types
LL + fn _f5(&mut _a: &mut u32) {} //~ ERROR mismatched types

error[E0308]: mismatched types
  --> /checkout/src/test/ui/mismatched_types/ref-pat-suggestions.rs:11:23
   |
   |
LL |     let _: fn(u32) = |&_a| (); //~ ERROR mismatched types
   |                       ^--
   |                       |expected due to this
   |                       expected `u32`, found reference
   |
   = note:   expected type `u32`
   = note:   expected type `u32`
           found reference `&_`
help: consider removing `&` from the pattern
   |
LL -     let _: fn(u32) = |&_a| (); //~ ERROR mismatched types
LL +     let _: fn(u32) = |_a| (); //~ ERROR mismatched types

error[E0308]: mismatched types
  --> /checkout/src/test/ui/mismatched_types/ref-pat-suggestions.rs:12:23
   |
   |
LL |     let _: fn(u32) = |&mut _a| (); //~ ERROR mismatched types
   |                       |    |
   |                       |    expected due to this
   |                       expected `u32`, found `&mut _`
   |
   |
   = note:           expected type `u32`
           found mutable reference `&mut _`
help: consider removing `&mut` from the pattern
   |
LL -     let _: fn(u32) = |&mut _a| (); //~ ERROR mismatched types
LL +     let _: fn(u32) = |_a| (); //~ ERROR mismatched types

error[E0308]: mismatched types
  --> /checkout/src/test/ui/mismatched_types/ref-pat-suggestions.rs:13:25
   |
   |
LL |     let _: fn(&u32) = |&&_a| (); //~ ERROR mismatched types
   |                         ^--
   |                         |expected due to this
   |                         expected `u32`, found reference
   |
   = note:   expected type `u32`
   = note:   expected type `u32`
           found reference `&_`
help: consider removing `&` from the pattern
   |
LL -     let _: fn(&u32) = |&&_a| (); //~ ERROR mismatched types
LL +     let _: fn(&u32) = |&_a| (); //~ ERROR mismatched types

error[E0308]: mismatched types
  --> /checkout/src/test/ui/mismatched_types/ref-pat-suggestions.rs:14:33
   |
   |
LL |     let _: fn(&mut u32) = |&mut &_a| (); //~ ERROR mismatched types
   |                                 ^--
   |                                 |expected due to this
   |                                 expected `u32`, found reference
   |
   = note:   expected type `u32`
   = note:   expected type `u32`
           found reference `&_`
help: consider removing `&` from the pattern
   |
LL -     let _: fn(&mut u32) = |&mut &_a| (); //~ ERROR mismatched types
LL +     let _: fn(&mut u32) = |&mut _a| (); //~ ERROR mismatched types

error[E0308]: mismatched types
  --> /checkout/src/test/ui/mismatched_types/ref-pat-suggestions.rs:15:25
   |
   |
LL |     let _: fn(&u32) = |&&mut _a| (); //~ ERROR mismatched types
   |                         |    |
   |                         |    expected due to this
   |                         expected `u32`, found `&mut _`
   |
   |
   = note:           expected type `u32`
           found mutable reference `&mut _`
help: consider removing `&mut` from the pattern
   |
LL -     let _: fn(&u32) = |&&mut _a| (); //~ ERROR mismatched types
LL +     let _: fn(&u32) = |&_a| (); //~ ERROR mismatched types

error[E0308]: mismatched types
  --> /checkout/src/test/ui/mismatched_types/ref-pat-suggestions.rs:16:33
   |
   |
LL |     let _: fn(&mut u32) = |&mut &mut _a| (); //~ ERROR mismatched types
   |                                 |    |
   |                                 |    expected due to this
   |                                 expected `u32`, found `&mut _`
   |
   |
   = note:           expected type `u32`
           found mutable reference `&mut _`
help: consider removing `&mut` from the pattern
   |
LL -     let _: fn(&mut u32) = |&mut &mut _a| (); //~ ERROR mismatched types
LL +     let _: fn(&mut u32) = |&mut _a| (); //~ ERROR mismatched types

error[E0308]: mismatched types
  --> /checkout/src/test/ui/mismatched_types/ref-pat-suggestions.rs:18:14
   |
   |
LL |     let _ = |&_a: u32| (); //~ ERROR mismatched types
   |              ^^^  --- expected due to this
   |              expected `u32`, found reference
   |
   = note:   expected type `u32`
           found reference `&_`
           found reference `&_`
help: to take parameter `_a` by reference, move `&` to the type
   |
LL -     let _ = |&_a: u32| (); //~ ERROR mismatched types
LL +     let _ = |_a: &u32| (); //~ ERROR mismatched types

error[E0308]: mismatched types
  --> /checkout/src/test/ui/mismatched_types/ref-pat-suggestions.rs:19:14
   |
   |
LL |     let _ = |&mut _a: u32| (); //~ ERROR mismatched types
   |              ^^^^^^^  --- expected due to this
   |              expected `u32`, found `&mut _`
   |
   = note:           expected type `u32`
           found mutable reference `&mut _`
           found mutable reference `&mut _`
help: to take parameter `_a` by reference, move `&mut` to the type
   |
LL -     let _ = |&mut _a: u32| (); //~ ERROR mismatched types
LL +     let _ = |_a: &mut u32| (); //~ ERROR mismatched types

error[E0308]: mismatched types
  --> /checkout/src/test/ui/mismatched_types/ref-pat-suggestions.rs:20:15
   |
   |
LL |     let _ = |&&_a: &u32| (); //~ ERROR mismatched types
   |               ^^^  ---- expected due to this
   |               expected `u32`, found reference
   |
   = note:   expected type `u32`
           found reference `&_`
           found reference `&_`
help: consider removing `&` from the pattern
   |
LL -     let _ = |&&_a: &u32| (); //~ ERROR mismatched types
LL +     let _ = |&_a: &u32| (); //~ ERROR mismatched types

error[E0308]: mismatched types
  --> /checkout/src/test/ui/mismatched_types/ref-pat-suggestions.rs:21:19
   |
   |
LL |     let _ = |&mut &_a: &mut u32| (); //~ ERROR mismatched types
   |                   ^^^  -------- expected due to this
   |                   expected `u32`, found reference
   |
   = note:   expected type `u32`
           found reference `&_`
           found reference `&_`
help: consider removing `&` from the pattern
   |
LL -     let _ = |&mut &_a: &mut u32| (); //~ ERROR mismatched types
LL +     let _ = |&mut _a: &mut u32| (); //~ ERROR mismatched types

error[E0308]: mismatched types
  --> /checkout/src/test/ui/mismatched_types/ref-pat-suggestions.rs:22:15
   |
   |
LL |     let _ = |&&mut _a: &u32| (); //~ ERROR mismatched types
   |               ^^^^^^^  ---- expected due to this
   |               expected `u32`, found `&mut _`
   |
   = note:           expected type `u32`
           found mutable reference `&mut _`
           found mutable reference `&mut _`
help: consider removing `&mut` from the pattern
   |
LL -     let _ = |&&mut _a: &u32| (); //~ ERROR mismatched types
LL +     let _ = |&_a: &u32| (); //~ ERROR mismatched types

error[E0308]: mismatched types
  --> /checkout/src/test/ui/mismatched_types/ref-pat-suggestions.rs:23:19
   |
   |
LL |     let _ = |&mut &mut _a: &mut u32| (); //~ ERROR mismatched types
   |                   ^^^^^^^  -------- expected due to this
   |                   expected `u32`, found `&mut _`
   |
   = note:           expected type `u32`
           found mutable reference `&mut _`
           found mutable reference `&mut _`
help: consider removing `&mut` from the pattern
   |
LL -     let _ = |&mut &mut _a: &mut u32| (); //~ ERROR mismatched types
LL +     let _ = |&mut _a: &mut u32| (); //~ ERROR mismatched types

error: aborting due to 18 previous errors

For more information about this error, try `rustc --explain E0308`.
