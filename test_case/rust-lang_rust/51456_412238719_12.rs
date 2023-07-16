\n\nWe want to express the constraint that Foo should not outlive `'a`, because\nthe data pointed to by `T` is only valid for that lifetime. The problem is\nthat there are no actual uses of `'a`. It's possible tssue-38293/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-38293/auxiliary" "-A" "unused"
[00:49:33] ------------------------------------------
[00:49:33] 
[00:49:33] ------------------------------------------
[00:49:33] stderr:
[00:49:33] stderr:
[00:49:33] ------------------------------------------
[00:49:33] {"message":"unresolved import `foo::f`","code":{"code":"E0432","explanation":"\nAn import was unresolved.\n\nErroneous code example:\n\n