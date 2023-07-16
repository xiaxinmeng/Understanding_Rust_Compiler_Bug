plain
.................................................................................................... 2000/12589
.................................................................................................... 2100/12589
.i.................................................................................................. 2200/12589
.................................................................................................... 2300/12589
.......................................................................F...F........................ 2400/12589
.................................................................................................... 2600/12589
.................................................................................................... 2700/12589
.................................................................................................... 2800/12589
.................................................................i.................................. 2900/12589
---
failures:

---- [ui] ui/consts/const-eval/heap/alloc_intrinsic_zero_sized.rs stdout ----
normalized stderr:
warning: unnecessary `unsafe` block
   |
LL |         unsafe {
LL |         unsafe {
   |         ^^^^^^ unnecessary `unsafe` block
   = note: `#[warn(unused_unsafe)]` on by default

warning: 1 warning emitted

---
To only update this specific test, also pass `--test-args consts/const-eval/heap/alloc_intrinsic_zero_sized.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/heap/alloc_intrinsic_zero_sized.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/heap/alloc_intrinsic_zero_sized/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/heap/alloc_intrinsic_zero_sized/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: unnecessary `unsafe` block
   |
LL |         unsafe {
LL |         unsafe {
   |         ^^^^^^ unnecessary `unsafe` block
   = note: `#[warn(unused_unsafe)]` on by default

warning: 1 warning emitted



------------------------------------------


---- [ui] ui/consts/const-eval/heap/dealloc_intrinsic_zero_sized.rs stdout ----
normalized stderr:
warning: unnecessary `unsafe` block
   |
LL |         unsafe {
LL |         unsafe {
   |         ^^^^^^ unnecessary `unsafe` block
   = note: `#[warn(unused_unsafe)]` on by default

warning: 1 warning emitted

---
To only update this specific test, also pass `--test-args consts/const-eval/heap/dealloc_intrinsic_zero_sized.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/heap/dealloc_intrinsic_zero_sized.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/heap/dealloc_intrinsic_zero_sized/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/heap/dealloc_intrinsic_zero_sized/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: unnecessary `unsafe` block
   |
LL |         unsafe {
LL |         unsafe {
   |         ^^^^^^ unnecessary `unsafe` block
   = note: `#[warn(unused_unsafe)]` on by default

warning: 1 warning emitted

