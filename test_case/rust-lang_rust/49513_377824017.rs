plain
Resolving deltas: 100% (613212/613212), completed with 4857 local objects.
---
[00:00:42] configure: rust.quiet-tests     := True
---
[00:40:36] .........................................................................i..........................
[00:40:42] ................i...................................................................................
---
[00:41:17] ............................................................................................i.......
[00:41:23] ................................................................i...................................
---
[00:41:45] - error[E0080]: enums without inhabited variants do not have discriminants
[00:41:45] + error[E0080]: constant evaluation of enum discriminant resulted in non-integer
[00:41:45] 2   --> $DIR/uninhabited_enum_discriminant1.rs:14:9
[00:41:45] 3    |
[00:41:45] 4 LL |     A = X::A as isize, //~ ERROR enums without inhabited variants do not have discriminants
[00:41:45]
[00:41:45]
[00:41:45] The actual stderr differed from the expected stderr.
[00:41:45] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/uninhabited_enum_discriminant1.stderr
[00:41:45] To update references, run this command from build directory:
[00:41:45] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'uninhabited_enum_discriminant1.rs'
[00:41:45]
[00:41:45] error: 1 errors occurred comparing output.
[00:41:45] status: exit code: 101
[00:41:45] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/uninhabited_enum_discriminant1.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/uninhabited_enum_discriminant1.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/uninhabited_enum_discriminant1.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
---
[00:41:45] {"message":"constant evaluation of enum discriminant resulted in non-integer","code":{"code":"E0080","explanation":"\nThis error indicates that the compiler was unable to sensibly evaluate an\nconstant expression that had to be evaluated. Attempting to divide by 0\nor causing integer overflow are two ways to induce this error. For example:\n\n