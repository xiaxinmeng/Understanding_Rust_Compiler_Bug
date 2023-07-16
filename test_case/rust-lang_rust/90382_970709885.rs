
---- [ui] ui/cfg/cfg-target-family.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/cfg/cfg-target-family.rs" "-Zthread
s=1" "--target=wasm32-unknown-emscripten" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no"
 "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cfg/cfg-ta
rget-family" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-
helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cfg/cfg-target-family/auxiliary"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
error[E0428]: the name `main` is defined multiple times
  --> /checkout/src/test/ui/cfg/cfg-target-family.rs:15:1
   |
LL | pub fn main() {
   | ------------- previous definition of the value `main` here
...
LL | pub fn main() {
   | ^^^^^^^^^^^^^ `main` redefined here
   |
   = note: `main` must be defined only once in the value namespace of this module
