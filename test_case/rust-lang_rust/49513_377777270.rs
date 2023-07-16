plain
Resolving deltas: 100% (613183/613183), completed with 4857 local objects.
---
[00:00:42] configure: rust.quiet-tests     := True
---
[00:38:50] .........................................................................i..........................
[00:38:56] ................i...................................................................................
---
[00:39:30] ............................................................................................i.......
[00:39:37] ................................................................i...................................
[00:39:43] ....................................................................................................
[00:39:50] ....................................................................................................
[00:39:58] .......................................................................................FF...........
---
[00:39:58] - error[E0080]: enums without inhabited variants do not have discriminants
[00:39:58] + error[E0080]: constant evaluation of enum discriminant resulted in non-integer
[00:39:58] 2   --> $DIR/uninhabited_enum_discriminant1.rs:14:9
[00:39:58] 3    |
[00:39:58] 4 LL |     A = X::A as isize, //~ ERROR enums without inhabited variants do not have discriminants
[00:39:58]
[00:39:58]
[00:39:58] The actual stderr differed from the expected stderr.
[00:39:58] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/uninhabited_enum_discriminant1.stderr
[00:39:58] To update references, run this command from build directory:
[00:39:58] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'uninhabited_enum_discriminant1.rs'
[00:39:58]
[00:39:58] error: 1 errors occurred comparing output.
[00:39:58] status: exit code: 101
[00:39:58] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/uninhabited_enum_discriminant1.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/uninhabited_enum_discriminant1.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-on  --> /checkout/src/test/ui/uninhabited_enum_discriminant1.rs:14:9\n   |\nLL |     A = X::A as isize, //~ ERROR enums without inhabited variants do not have discriminants\n   |         ^^^^^^^^^^^^^\n\n"}
[00:39:58] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:39:58] {"message":"For more information about this error, try `rustc --explain E0080`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0080`.\n"}
---
[00:39:58] - error[E0080]: enums without inhabited variants do not have discriminants
[00:39:58] + error[E0080]: constant evaluation of enum discriminant resulted in non-integer
[00:39:58] 2   --> $DIR/uninhabited_enum_discriminant2.rs:14:9
[00:39:58] 3    |
[00:39:58] 4 LL |     B = A, //~ ERROR enums without inhabited variants do not have discriminants
[00:39:58]
[00:39:58]
[00:39:58] The actual stderr differed from the expected stderr.
[00:39:58] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/uninhabited_enum_discriminant2.stderr
[00:39:58] To update references, run this command from build directory:
[00:39:58] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'uninhabited_enum_discriminant2.rs'
[00:39:58]
[00:39:58] error: 1 errors occurred comparing output.
[00:39:58] status: exit code: 101
[00:39:58] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/uninhabited_enum_discriminant2.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/uninhabited_enum_discriminant2.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/uninhabited_enum_discriminant2.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
---
[00:39:58] {"message":"constant evaluation of enum discriminant resulted in non-integer","code":{"code":"E0080","explanation":"\nThis error indicates that the compiler was unable to sensibly evaluate an\nconstant expression that had to be evaluated. Attempting to divide by 0\nor causing integer overflow are two ways to induce this error. For example:\n\n