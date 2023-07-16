plain
Resolving deltas: 100% (615406/615406), completed with 4898 local objects.
---
[00:00:46] configure: rust.quiet-tests     := True
---
[00:43:57] ...............................................................................i....................
[00:44:03] ......................i..F..........................................................................
[00:44:07] ....................................................................................................
[00:44:11] ....................................................................................................
[00:44:16] ....................................................................................................
[00:44:20] ....................................................................................................
[00:44:26] .............F...........................................................F........................F.
[00:44:33] .F.F.........................................F......................................................
[00:44:39] ....................................................................................................
[00:44:47] ...................i...........................................................................i....
[00:44:54] ........................................................................F...........................
[00:45:00] .........ii.........................................................................................
---
[00:45:12] - error[E0391]: cyclic dependency detected
[00:45:12] + error[E0391]: cycle detected when computing the supertraits of `B`
[00:45:12] +    |
[00:45:12] + note: ...which requires computing the supertraits of `C`...
[00:45:12] +   --> $DIR/cycle-trait-supertrait-indirect.rs:17:1
[00:45:12] +    |
[00:45:12] + LL | trait B: C {
[00:45:12] +    | ^^^^^^^^^^
[00:45:12] + note: ...which again requires computing the supertraits of `B`, completing the cycle
[00:45:12] 2   --> $DIR/cycle-trait-supertrait-indirect.rs:20:1
[00:45:12] 3    |
[00:45:12] 4 LL | trait C: B { }
[00:45:12]
[00:45:12] -    | ^^^^^^^^^^ cyclic reference
[00:45:12] -    |
[00:45:12] - note: the cycle begins when computing the supertraits of `B`...
[00:45:12] +    | ^^^^^^^^^^
[00:45:12] + note: cycle used when computing the supertraits of `A`
[00:45:12] 8   --> $DIR/cycle-trait-supertrait-indirect.rs:14:1
[00:45:12] 9    |
[00:45:12] 10 LL | trait A: B {
[00:45:12]
[00:45:12] 11    | ^^^^^^^^^^
[00:45:12] - note: ...which then requires computing the supertraits of `C`...
[00:45:12] -   --> $DIR/cycle-trait-supertrait-indirect.rs:17:1
[00:45:12] -    |
[00:45:12] - LL | trait B: C {
[00:45:12] -    | ^^^^^^^^^^
[00:45:12] -    = note: ...which then again requires computing the supertraits of `B`, completing the cycle.
---
[00:45:12] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'cycle-trait-supertrait-indirect.rs'
[00:45:12]
[00:45:12] error: 1 errors occurred comparing output.
[00:45:12] status: exit code: 101
[00:45:12] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/cycle-trait-supertrait-indirect.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cycle-trait-supertrait-indirect.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cycle-trait-supertrait-indirect.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
---
[00:45:12] {"message":"cycle detected when computing the supertraits of `B`","code":{"code":"E0391","explanation":"\nThis error indicates that some types or traits depend on each other\nand therefore cannot be constructed.\n\nThe following example contains a circular dependency between two traits:\n\n