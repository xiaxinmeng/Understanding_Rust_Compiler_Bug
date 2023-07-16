plain
.................................................................................................... 9500/11999
.................................................................................................... 9600/11999
.................................................................................................... 9700/11999
.....................................i.i......i..................................................... 9800/11999
.....................................................................................F............ii 9900/11999
iiii.i..iiiiiiFi.................................................................................... 10000/11999
.................................................................................................... 10200/11999
.................................................................................................... 10300/11999
.................................................................................................... 10400/11999
.................................................................................................... 10500/11999
---
failures:

---- [ui] ui/rust-2021/future-prelude-collision-imported.rs stdout ----
normalized stderr:
warning: trait method `try_into` will become ambiguous in Rust 2021
   |
   |
LL |         let _: u32 = 3u8.try_into().unwrap();
   |                      ^^^^^^^^^^^^^^ help: disambiguate the associated function: `TryIntoU32::try_into(3u8)`
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
note: the lint level is defined here
  --> $DIR/future-prelude-collision-imported.rs:4:9
   |
   |
LL | #![warn(future_prelude_collision)]
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2021 edition!
   = note: for more information, see issue #85684 <https://github.com/rust-lang/rust/issues/85684>


warning: trait method `try_into` will become ambiguous in Rust 2021
   |
   |
LL |         let _: u32 = 3u8.try_into().unwrap();
   |                      ^^^^^^^^^^^^^^ help: disambiguate the associated function: `TryIntoU32::try_into(3u8)`
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2021 edition!
   = note: for more information, see issue #85684 <https://github.com/rust-lang/rust/issues/85684>


warning: trait method `try_into` will become ambiguous in Rust 2021
   |
   |
LL |         let _: u32 = 3u8.try_into().unwrap();
   |                      ^^^^^^^^^^^^^^ help: disambiguate the associated function: `TryIntoU32::try_into(3u8)`
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2021 edition!
   = note: for more information, see issue #85684 <https://github.com/rust-lang/rust/issues/85684>

warning: 3 warnings emitted
warning: 3 warnings emitted




The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2021/future-prelude-collision-imported/future-prelude-collision-imported.stderr
normalized fixed:
// run-rustfix
// edition:2018
// check-pass
#![warn(future_prelude_collision)]
#![allow(dead_code)]
mod m {
    pub trait TryIntoU32 {
    pub trait TryIntoU32 {
        fn try_into(self) -> Result<u32, ()>;


    impl TryIntoU32 for u8 {
        fn try_into(self) -> Result<u32, ()> {
            Ok(self as u32)
    }


    pub trait AnotherTrick {}

mod a {
mod a {
    use crate::m::TryIntoU32;
    fn main() {
    fn main() {
        // In this case, we can just use `TryIntoU32`
        let _: u32 = TryIntoU32::try_into(3u8).unwrap();
}

mod b {
mod b {
    use crate::m::AnotherTrick as TryIntoU32;
    use crate::m::TryIntoU32 as _;
    fn main() {
    fn main() {
        // In this case, a `TryIntoU32::try_into` rewrite will not work, and we need to use
        // the path `crate::m::TryIntoU32` (with which it was imported).
        let _: u32 = TryIntoU32::try_into(3u8).unwrap();
}

mod c {
mod c {
    use super::m::TryIntoU32 as _;
    use crate::m::AnotherTrick as TryIntoU32;
    fn main() {
    fn main() {
        // In this case, a `TryIntoU32::try_into` rewrite will not work, and we need to use
        // the path `super::m::TryIntoU32` (with which it was imported).
        let _: u32 = TryIntoU32::try_into(3u8).unwrap();
}

fn main() {}




The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2021/future-prelude-collision-imported/future-prelude-collision-imported.fixed
To only update this specific test, also pass `--test-args rust-2021/future-prelude-collision-imported.rs`

error: 2 errors occurred comparing output.
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rust-2021/future-prelude-collision-imported.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2021/future-prelude-collision-imported" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2021/future-prelude-collision-imported/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: trait method `try_into` will become ambiguous in Rust 2021
   |
   |
LL |         let _: u32 = 3u8.try_into().unwrap();
   |                      ^^^^^^^^^^^^^^ help: disambiguate the associated function: `TryIntoU32::try_into(3u8)`
note: the lint level is defined here
  --> /checkout/src/test/ui/rust-2021/future-prelude-collision-imported.rs:4:9
   |
   |
LL | #![warn(future_prelude_collision)]
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2021 edition!
   = note: for more information, see issue #85684 <https://github.com/rust-lang/rust/issues/85684>


warning: trait method `try_into` will become ambiguous in Rust 2021
   |
   |
LL |         let _: u32 = 3u8.try_into().unwrap();
   |                      ^^^^^^^^^^^^^^ help: disambiguate the associated function: `TryIntoU32::try_into(3u8)`
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2021 edition!
   = note: for more information, see issue #85684 <https://github.com/rust-lang/rust/issues/85684>


warning: trait method `try_into` will become ambiguous in Rust 2021
   |
   |
LL |         let _: u32 = 3u8.try_into().unwrap();
   |                      ^^^^^^^^^^^^^^ help: disambiguate the associated function: `TryIntoU32::try_into(3u8)`
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2021 edition!
   = note: for more information, see issue #85684 <https://github.com/rust-lang/rust/issues/85684>

warning: 3 warnings emitted
warning: 3 warnings emitted


------------------------------------------


---- [ui] ui/rust-2021/future-prelude-collision-shadow.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rust-2021/future-prelude-collision-shadow.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2021/future-prelude-collision-shadow" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2021/future-prelude-collision-shadow/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0599]: no method named `try_into` found for type `u8` in the current scope
   |
   |
LL |         let _: u32 = 3u8.try_into().unwrap();
   |                          ^^^^^^^^ method not found in `u8`
  ::: /checkout/library/core/src/convert/mod.rs:395:8
   |
   |
LL |     fn try_into(self) -> Result<T, Self::Error>;
   |        |
   |        |
   |        the method is available for `Box<u8>` here
   |        the method is available for `Pin<u8>` here
   |        the method is available for `Arc<u8>` here
   |        the method is available for `Rc<u8>` here
   = help: items from traits can only be used if the trait is in scope
   = note: the following traits are implemented but not in scope; perhaps add a `use` for one of them:
   = note: the following traits are implemented but not in scope; perhaps add a `use` for one of them:
           candidate #1: `use crate::m::TryIntoU32;`
           candidate #2: `use std::convert::TryInto;`
help: consider wrapping the receiver expression with the appropriate type
   |
LL |         let _: u32 = Box::new(3u8).try_into().unwrap();
   |                      ^^^^^^^^^   ^
help: consider wrapping the receiver expression with the appropriate type
   |
LL |         let _: u32 = Pin::new(3u8).try_into().unwrap();
   |                      ^^^^^^^^^   ^
help: consider wrapping the receiver expression with the appropriate type
   |
LL |         let _: u32 = Arc::new(3u8).try_into().unwrap();
   |                      ^^^^^^^^^   ^
help: consider wrapping the receiver expression with the appropriate type
   |
LL |         let _: u32 = Rc::new(3u8).try_into().unwrap();
   |                      ^^^^^^^^   ^
error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.

---
test result: FAILED. 11896 passed; 2 failed; 101 ignored; 0 measured; 0 filtered out; finished in 119.40s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:11:49
