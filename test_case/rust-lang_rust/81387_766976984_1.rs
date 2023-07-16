
// error: struct fields are initialized with a colon
//    |
//    |
// LL |     let _ = X { f1 = 5 };
//    |                    ^ help: replace equals symbol with a colon: `:`

struct X {
    f1: i32,
}
---
    f3: i32,
}

fn main() {
    let _ = X { f1: 5 };
    //~^ ERROR expected `:`, found `=`
    let f3 = 3;
    let f3 = 3;
    let _ = Y {
        f1: 5,
        //~^ ERROR expected `:`, found `=`
        f2: 4,
        f3,
}




The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-57684/issue-57684.fixed
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args parser/issue-57684.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/issue-57684.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-57684" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-57684/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: expected `:`, found `=`
   |
   |
LL |     let _ = X { f1 = 5 };
   |                   -^
   |                   |
   |                   help: replace equals symbol with a colon: `:`

error: expected `:`, found `=`
   |
LL |         f1 = 5,
   |           -^
   |           |
   |           |
   |           help: replace equals symbol with a colon: `:`
error: aborting due to 2 previous errors


------------------------------------------
------------------------------------------


---- [ui] ui/parser/issue-57819.rs stdout ----
normalized fixed:
// run-rustfix

#![allow(warnings)]

// This test checks that the following error is emitted and the suggestion works:
// 