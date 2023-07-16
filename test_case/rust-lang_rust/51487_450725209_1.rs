\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-20825.rs","byte_start":42,"byte_end"ated_type_defaults)]
[01:03:32] + LL | |
[01:03:32] + LL | | struct S<T = u8>(T);
[01:03:32] + LL | | trait Tr<T = u8> {
[01:03:32] + ...  |
[01:03:32] + LL | |
[01:03:32] + LL | | fn main() {}
[01:03:32] 16 
[01:03:32] 16 
[01:03:32] 17 error[E0391]: cycle detected when processing `<impl at $DIR/resolve-self-in-impl.rs:16:1: 16:13>`
[01:03:32] 
[01:03:32] 21    |      ^^^^
[01:03:32] 22    |
[01:03:32] 22    |
[01:03:32] 23    = note: ...which again requires processing `<impl at $DIR/resolve-self-in-impl.rs:16:1: 16:13>`, completing the cycle
[01:03:32] + note: cycle used when processing ``
[01:03:32] +   --> $DIR/resolve-self-in-impl.rs:1:1
[01:03:32] +    |
[01:03:32] + LL | / #![feature(associated_type_defaults)]
[01:03:32] + LL | |
[01:03:32] + LL | | struct S<T = u8>(T);
[01:03:32] + LL | | trait Tr<T = u8> {
[01:03:32] + ...  |
[01:03:32] + LL | |
[01:03:32] + LL | | fn main() {}
[01:03:32] 24 
[01:03:32] 24 
[01:03:32] 25 error[E0391]: cycle detected when processing `<impl at $DIR/resolve-self-in-impl.rs:17:1: 17:16>`
[01:03:32] 
[01:03:32] 29    |        ^^^^
[01:03:32] 30    |
[01:03:32] 30    |
[01:03:32] 31    = note: ...which again requires processing `<impl at $DIR/resolve-self-in-impl.rs:17:1: 17:16>`, completing the cycle
[01:03:32] + note: cycle used when processing ``
[01:03:32] +   --> $DIR/resolve-self-in-impl.rs:1:1
[01:03:32] +    |
[01:03:32] + LL | / #![feature(associated_type_defaults)]
[01:03:32] + LL | |
[01:03:32] + LL | | struct S<T = u8>(T);
[01:03:32] +lve-self-in-impl.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/resolve-self-in-impl/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/resolve-self-in-impl/auxiliary" "-A" "unused"
[01:03:32] ------------------------------------------
[01:03:32] 
[01:03:32] ------------------------------------------
[01:03:32] stderr:
[01:03:32] stderr:
[01:03:32] ------------------------------------------
[01:03:32] {"message":"cycle detected when processing `<impl at /checkout/src/test/ui/resolve/resolve-self-in-impl.rs:14:1: 14:20>`","code":{"code":"E0391","explanation":"\nThis error indicates that some types or traits depend on each other\nand therefore cannot be constructed.\n\nThe following example contains a circular dependency between two traits:\n\n