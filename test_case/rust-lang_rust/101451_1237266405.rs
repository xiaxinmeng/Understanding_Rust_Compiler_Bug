plain
Some tests failed in compiletest suite=incremental mode=incremental host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
......................................................................
failures:

---- [incremental] src/test/incremental/issue-100521-change-struct-name-assocty.rs stdout ----

error in revision `rpass1`: compilation failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/issue-100521-change-struct-name-assocty.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass1" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-100521-change-struct-name-assocty/issue-100521-change-struct-name-assocty.inc" "-Z" "incremental-verify-ich" "-O" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-100521-change-struct-name-assocty/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-100521-change-struct-name-assocty/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0601]: `main` function not found in crate `issue_100521_change_struct_name_assocty`
  --> /checkout/src/test/incremental/issue-100521-change-struct-name-assocty.rs:63:18
LL | struct Checksum2;
LL | struct Checksum2;
   |                  ^ consider adding a `main` function to `/checkout/src/test/incremental/issue-100521-change-struct-name-assocty.rs`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0601`.
------------------------------------------
------------------------------------------



failures:
    [incremental] src/test/incremental/issue-100521-change-struct-name-assocty.rs
test result: FAILED. 157 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 12.33s

Build completed unsuccessfully in 0:10:44
