plain
---- [run-make] tests/run-make/core-no-oom-handling stdout ----

error: make failed
status: exit status: 2
command: cd "/checkout/tests/run-make/core-no-oom-handling" && AR="ar" CC="cc -ffunction-sections -fdata-sections -fPIC -m64" CXX="c++ -ffunction-sections -fdata-sections -fPIC -m64" HOST_RPATH_DIR="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" LD_LIB_PATH_ENVVAR="LD_LIBRARY_PATH" LLVM_BIN_DIR="/usr/lib/llvm-14/bin" LLVM_COMPONENTS="aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgputargetmca amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker dwp engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto m68k m68kasmparser m68kcodegen m68kdesc m68kdisassembler m68kinfo mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils ve veasmparser vecodegen vectorize vedesc vedisassembler veinfo webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo webassemblyutils windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86targetmca xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" LLVM_FILECHECK="/usr/lib/llvm-14/bin/FileCheck" NODE="/usr/bin/node" PYTHON="/usr/bin/python3" RUSTC="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" RUSTDOC="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" RUST_BUILD_STAGE="stage2-x86_64-unknown-linux-gnu" RUST_DEMANGLER="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/rust-demangler" S="/checkout" TARGET="x86_64-unknown-linux-gnu" TARGET_RPATH_DIR="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" TMPDIR="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/core-no-oom-handling/core-no-oom-handling" "make"
--- stdout -------------------------------
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/core-no-oom-handling/core-no-oom-handling:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/core-no-oom-handling/core-no-oom-handling --edition=2021 -Dwarnings --crate-type=rlib ../../../library/core/src/lib.rs --cfg no_global_oom_handling
--- stderr -------------------------------
error[E0773]: attempted to define built-in macro more than once
    --> /checkout/library/core/src/macros/mod.rs:1310:5
     |
     |
1310 |     macro_rules! cfg {
     |     ^^^^^^^^^^^^^^^^
     |
note: previously defined here
    --> ../../../library/core/src/macros/mod.rs:1310:5
     |
1310 | /     macro_rules! cfg {
1311 | |         ($($cfg:tt)*) => {
1313 | |         };
1314 | |     }
     | |_____^


error[E0152]: found duplicate lang item `manually_drop`
  --> ../../../library/core/src/mem/manually_drop.rs:50:1
   |
50 | pub struct ManuallyDrop<T: ?Sized> {
   |
   = note: the lang item is first defined in crate `core`.
   = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
   = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
   = note: second definition in the local crate (`lib`)

error[E0152]: found duplicate lang item `maybe_uninit`
   --> ../../../library/core/src/mem/maybe_uninit.rs:249:1
249 | pub union MaybeUninit<T> {
    | ^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: the lang item is first defined in crate `core`.
    = note: the lang item is first defined in crate `core`.
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: second definition in the local crate (`lib`)

error[E0152]: found duplicate lang item `transmute_trait`
 --> ../../../library/core/src/mem/transmutability.rs:8:1
  |
8 | pub unsafe trait BikeshedIntrinsicFrom<Src, Context, const ASSUME: Assume = { Assume::NOTHING }>
  |
  = note: the lang item is first defined in crate `core`.
  = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
  = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
  = note: second definition in the local crate (`lib`)

error[E0152]: found duplicate lang item `transmute_opts`
  --> ../../../library/core/src/mem/transmutability.rs:18:1
18 | pub struct Assume {
   | ^^^^^^^^^^^^^^^^^
   |
   = note: the lang item is first defined in crate `core`.
   = note: the lang item is first defined in crate `core`.
   = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
   = note: second definition in the local crate (`lib`)

error[E0152]: found duplicate lang item `pointee_trait`
  --> ../../../library/core/src/ptr/metadata.rs:54:1
54 | pub trait Pointee {
   | ^^^^^^^^^^^^^^^^^
   |
   = note: the lang item is first defined in crate `core`.
   = note: the lang item is first defined in crate `core`.
   = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
   = note: second definition in the local crate (`lib`)

error[E0152]: found duplicate lang item `dyn_metadata`
   --> ../../../library/core/src/ptr/metadata.rs:179:1
    |
179 | pub struct DynMetadata<Dyn: ?Sized> {
    |
    = note: the lang item is first defined in crate `core`.
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: second definition in the local crate (`lib`)

error[E0152]: found duplicate lang item `drop_in_place`
   --> ../../../library/core/src/ptr/mod.rs:490:1
    |
490 | pub unsafe fn drop_in_place<T: ?Sized>(to_drop: *mut T) {
    |
    = note: the lang item is first defined in crate `core`.
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: second definition in the local crate (`lib`)

error[E0152]: found duplicate lang item `align_offset`
    --> ../../../library/core/src/ptr/mod.rs:1650:1
     |
1650 | pub(crate) const unsafe fn align_offset<T: Sized>(p: *const T, a: usize) -> usize {
     |
     = note: the lang item is first defined in crate `core`.
     = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
     = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
     = note: second definition in the local crate (`lib`)
error[E0152]: found duplicate lang item `clone`
   --> ../../../library/core/src/clone.rs:107:1
    |
107 | pub trait Clone: Sized {
107 | pub trait Clone: Sized {
    | ^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: the lang item is first defined in crate `core`.
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: second definition in the local crate (`lib`)

error[E0152]: found duplicate lang item `eq`
   --> ../../../library/core/src/cmp.rs:214:1
    |
214 | pub trait PartialEq<Rhs: ?Sized = Self> {
    |
    = note: the lang item is first defined in crate `core`.
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: second definition in the local crate (`lib`)

error[E0152]: found duplicate lang item `partial_ord`
    --> ../../../library/core/src/cmp.rs:1027:1
     |
1027 | pub trait PartialOrd<Rhs: ?Sized = Self>: PartialEq<Rhs> {
     |
     = note: the lang item is first defined in crate `core`.
     = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
     = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
     = note: second definition in the local crate (`lib`)

error[E0152]: found duplicate lang item `sized`
   --> ../../../library/core/src/marker.rs:101:1
101 | pub trait Sized {
    | ^^^^^^^^^^^^^^^
    |
    = note: the lang item is first defined in crate `core`.
    = note: the lang item is first defined in crate `core`.
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: second definition in the local crate (`lib`)

error[E0152]: found duplicate lang item `unsize`
   --> ../../../library/core/src/marker.rs:133:1
    |
133 | pub trait Unsize<T: ?Sized> {
    |
    = note: the lang item is first defined in crate `core`.
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: second definition in the local crate (`lib`)

error[E0152]: found duplicate lang item `structural_peq`
   --> ../../../library/core/src/marker.rs:160:1
160 | pub trait StructuralPartialEq {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: the lang item is first defined in crate `core`.
    = note: the lang item is first defined in crate `core`.
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: second definition in the local crate (`lib`)

error[E0152]: found duplicate lang item `structural_teq`
   --> ../../../library/core/src/marker.rs:213:1
213 | pub trait StructuralEq {
    | ^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: the lang item is first defined in crate `core`.
    = note: the lang item is first defined in crate `core`.
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: second definition in the local crate (`lib`)
error[E0152]: found duplicate lang item `copy`
   --> ../../../library/core/src/marker.rs:392:1
    |
392 | pub trait Copy: Clone {
392 | pub trait Copy: Clone {
    | ^^^^^^^^^^^^^^^^^^^^^
    |
    = note: the lang item is first defined in crate `core`.
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: second definition in the local crate (`lib`)
error[E0152]: found duplicate lang item `sync`
   --> ../../../library/core/src/marker.rs:532:1
    |
532 | pub unsafe auto trait Sync {
532 | pub unsafe auto trait Sync {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: the lang item is first defined in crate `core`.
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: second definition in the local crate (`lib`)

error[E0152]: found duplicate lang item `phantom_data`
   --> ../../../library/core/src/marker.rs:692:1
    |
692 | pub struct PhantomData<T: ?Sized>;
    |
    = note: the lang item is first defined in crate `core`.
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: second definition in the local crate (`lib`)

error[E0152]: found duplicate lang item `discriminant_kind`
   --> ../../../library/core/src/marker.rs:761:1
761 | pub trait DiscriminantKind {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: the lang item is first defined in crate `core`.
    = note: the lang item is first defined in crate `core`.
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: second definition in the local crate (`lib`)

error[E0152]: found duplicate lang item `freeze`
   --> ../../../library/core/src/marker.rs:773:1
773 | pub(crate) unsafe auto trait Freeze {}
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: the lang item is first defined in crate `core`.
    = note: the lang item is first defined in crate `core`.
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: second definition in the local crate (`lib`)

error[E0152]: found duplicate lang item `unpin`
   --> ../../../library/core/src/marker.rs:829:1
    |
829 | pub auto trait Unpin {}
    |
    = note: the lang item is first defined in crate `core`.
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: second definition in the local crate (`lib`)

error[E0152]: found duplicate lang item `destruct`
   --> ../../../library/core/src/marker.rs:862:1
862 | pub trait Destruct {}
    | ^^^^^^^^^^^^^^^^^^
    |
    = note: the lang item is first defined in crate `core`.
    = note: the lang item is first defined in crate `core`.
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: second definition in the local crate (`lib`)

error[E0152]: found duplicate lang item `tuple_trait`
   --> ../../../library/core/src/marker.rs:872:1
872 | pub trait Tuple {}
    | ^^^^^^^^^^^^^^^
    |
    = note: the lang item is first defined in crate `core`.
    = note: the lang item is first defined in crate `core`.
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: second definition in the local crate (`lib`)

error[E0152]: found duplicate lang item `pointer_like`
   --> ../../../library/core/src/marker.rs:884:1
884 | pub trait PointerLike {}
    | ^^^^^^^^^^^^^^^^^^^^^
    |
    = note: the lang item is first defined in crate `core`.
    = note: the lang item is first defined in crate `core`.
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: second definition in the local crate (`lib`)

error[E0152]: found duplicate lang item `fn_ptr_trait`
   --> ../../../library/core/src/marker.rs:934:1
    |
934 | pub trait FnPtr: Copy + Clone {
    |
    = note: the lang item is first defined in crate `core`.
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: second definition in the local crate (`lib`)

error[E0152]: found duplicate lang item `add`
  --> ../../../library/core/src/ops/arith.rs:76:1
   |
76 | pub trait Add<Rhs = Self> {
   |
   = note: the lang item is first defined in crate `core`.
   = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
   = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
   = note: second definition in the local crate (`lib`)

error[E0152]: found duplicate lang item `sub`
   --> ../../../library/core/src/ops/arith.rs:184:1
    |
184 | pub trait Sub<Rhs = Self> {
    |
    = note: the lang item is first defined in crate `core`.
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: second definition in the local crate (`lib`)

error[E0152]: found duplicate lang item `mul`
   --> ../../../library/core/src/ops/arith.rs:313:1
    |
313 | pub trait Mul<Rhs = Self> {
    |
    = note: the lang item is first defined in crate `core`.
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: second definition in the local crate (`lib`)

error[E0152]: found duplicate lang item `div`
   --> ../../../library/core/src/ops/arith.rs:446:1
    |
446 | pub trait Div<Rhs = Self> {
    |
    = note: the lang item is first defined in crate `core`.
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: second definition in the local crate (`lib`)

error[E0152]: found duplicate lang item `rem`
   --> ../../../library/core/src/ops/arith.rs:547:1
    |
547 | pub trait Rem<Rhs = Self> {
    |
    = note: the lang item is first defined in crate `core`.
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: second definition in the local crate (`lib`)

error[E0152]: found duplicate lang item `neg`
   --> ../../../library/core/src/ops/arith.rs:660:1
660 | pub trait Neg {
    | ^^^^^^^^^^^^^
    |
    = note: the lang item is first defined in crate `core`.
    = note: the lang item is first defined in crate `core`.
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: second definition in the local crate (`lib`)

error[E0152]: found duplicate lang item `add_assign`
   --> ../../../library/core/src/ops/arith.rs:733:1
    |
733 | pub trait AddAssign<Rhs = Self> {
    |
    = note: the lang item is first defined in crate `core`.
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: second definition in the local crate (`lib`)

error[E0152]: found duplicate lang item `sub_assign`
   --> ../../../library/core/src/ops/arith.rs:799:1
    |
799 | pub trait SubAssign<Rhs = Self> {
    |
    = note: the lang item is first defined in crate `core`.
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: second definition in the local crate (`lib`)

error[E0152]: found duplicate lang item `mul_assign`
   --> ../../../library/core/src/ops/arith.rs:856:1
    |
856 | pub trait MulAssign<Rhs = Self> {
    |
    = note: the lang item is first defined in crate `core`.
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: second definition in the local crate (`lib`)

error[E0152]: found duplicate lang item `div_assign`
   --> ../../../library/core/src/ops/arith.rs:913:1
    |
913 | pub trait DivAssign<Rhs = Self> {
    |
    = note: the lang item is first defined in crate `core`.
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: second definition in the local crate (`lib`)

error[E0152]: found duplicate lang item `rem_assign`
   --> ../../../library/core/src/ops/arith.rs:973:1
    |
973 | pub trait RemAssign<Rhs = Self> {
    |
    = note: the lang item is first defined in crate `core`.
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: second definition in the local crate (`lib`)

error[E0152]: found duplicate lang item `not`
  --> ../../../library/core/src/ops/bit.rs:34:1
34 | pub trait Not {
   | ^^^^^^^^^^^^^
   |
   = note: the lang item is first defined in crate `core`.
   = note: the lang item is first defined in crate `core`.
   = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
   = note: second definition in the local crate (`lib`)

error[E0152]: found duplicate lang item `bitand`
   --> ../../../library/core/src/ops/bit.rs:144:1
    |
144 | pub trait BitAnd<Rhs = Self> {
    |
    = note: the lang item is first defined in crate `core`.
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: second definition in the local crate (`lib`)

error[E0152]: found duplicate lang item `bitor`
   --> ../../../library/core/src/ops/bit.rs:244:1
    |
244 | pub trait BitOr<Rhs = Self> {
    |
    = note: the lang item is first defined in crate `core`.
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: second definition in the local crate (`lib`)

error[E0152]: found duplicate lang item `bitxor`
   --> ../../../library/core/src/ops/bit.rs:344:1
    |
344 | pub trait BitXor<Rhs = Self> {
    |
    = note: the lang item is first defined in crate `core`.
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: second definition in the local crate (`lib`)

error[E0152]: found duplicate lang item `shl`
   --> ../../../library/core/src/ops/bit.rs:443:1
    |
443 | pub trait Shl<Rhs = Self> {
    |
    = note: the lang item is first defined in crate `core`.
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: second definition in the local crate (`lib`)

error[E0152]: found duplicate lang item `shr`
   --> ../../../library/core/src/ops/bit.rs:561:1
    |
561 | pub trait Shr<Rhs = Self> {
    |
    = note: the lang item is first defined in crate `core`.
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: second definition in the local crate (`lib`)

error[E0152]: found duplicate lang item `bitand_assign`
   --> ../../../library/core/src/ops/bit.rs:688:1
    |
688 | pub trait BitAndAssign<Rhs = Self> {
    |
    = note: the lang item is first defined in crate `core`.
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: second definition in the local crate (`lib`)

error[E0152]: found duplicate lang item `bitor_assign`
   --> ../../../library/core/src/ops/bit.rs:759:1
    |
759 | pub trait BitOrAssign<Rhs = Self> {
    |
    = note: the lang item is first defined in crate `core`.
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: second definition in the local crate (`lib`)

error[E0152]: found duplicate lang item `bitxor_assign`
   --> ../../../library/core/src/ops/bit.rs:830:1
    |
830 | pub trait BitXorAssign<Rhs = Self> {
    |
    = note: the lang item is first defined in crate `core`.
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: second definition in the local crate (`lib`)

error[E0152]: found duplicate lang item `shl_assign`
   --> ../../../library/core/src/ops/bit.rs:899:1
    |
899 | pub trait ShlAssign<Rhs = Self> {
    |
    = note: the lang item is first defined in crate `core`.
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: second definition in the local crate (`lib`)

error[E0152]: found duplicate lang item `shr_assign`
   --> ../../../library/core/src/ops/bit.rs:981:1
    |
981 | pub trait ShrAssign<Rhs = Self> {
    |
    = note: the lang item is first defined in crate `core`.
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: second definition in the local crate (`lib`)
error[E0152]: found duplicate lang item `Continue`
  --> ../../../library/core/src/ops/control_flow.rs:89:5
   |
89 |     Continue(C),
89 |     Continue(C),
   |     ^^^^^^^^
---
...    |
1153 | |         bound_condition = "",
1154 | |     }
     | |_____- in this macro invocation
     = note: candidate #2 is defined in an impl for the type `u128`
     = note: this error originates in the macro `int_module` which comes from the expansion of the macro `uint_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
    --> ../../../library/core/src/num/shells/int_macros.rs:42:33
     |
     |
42   |         pub const MAX: $T = $T::MAX;
     |                                 ^^^ multiple `MAX` found
    ::: ../../../library/core/src/num/shells/u128.rs:11:1
     |
     |
11   | int_module! { u128, #[stable(feature = "i128", since="1.26.0")] }
     |
     |
note: candidate #1 is defined in an impl for the type `u128`
    --> ../../../library/core/src/num/uint_macros.rs:49:9
     |
49   |           pub const MAX: Self = !0;
     |
    ::: ../../../library/core/src/num/mod.rs:1134:5
     |
1134 | /     uint_impl! {
1134 | /     uint_impl! {
1135 | |         Self = u128,
1136 | |         ActualT = u128,
1137 | |         SignedT = i128,
...    |
1153 | |         bound_condition = "",
1154 | |     }
     | |_____- in this macro invocation
     = note: candidate #2 is defined in an impl for the type `u128`
     = note: this error originates in the macro `int_module` which comes from the expansion of the macro `uint_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
    --> ../../../library/core/src/num/shells/int_macros.rs:23:33
     |
     |
23   |         pub const MIN: $T = $T::MIN;
     |                                 ^^^ multiple `MIN` found
    ::: ../../../library/core/src/num/shells/u16.rs:11:1
     |
     |
11   | int_module! { u16 }
     |
     |
note: candidate #1 is defined in an impl for the type `u16`
    --> ../../../library/core/src/num/uint_macros.rs:36:9
36   |           pub const MIN: Self = 0;
     |           ^^^^^^^^^^^^^^^^^^^
     |
    ::: ../../../library/core/src/num/mod.rs:1040:5
---
...    |
1057 | |         bound_condition = "",
1058 | |     }
     | |_____- in this macro invocation
     = note: candidate #2 is defined in an impl for the type `u16`
     = note: this error originates in the macro `int_module` which comes from the expansion of the macro `uint_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
    --> ../../../library/core/src/num/shells/int_macros.rs:42:33
     |
     |
42   |         pub const MAX: $T = $T::MAX;
     |                                 ^^^ multiple `MAX` found
    ::: ../../../library/core/src/num/shells/u16.rs:11:1
     |
     |
11   | int_module! { u16 }
     |
     |
note: candidate #1 is defined in an impl for the type `u16`
    --> ../../../library/core/src/num/uint_macros.rs:49:9
     |
49   |           pub const MAX: Self = !0;
     |
    ::: ../../../library/core/src/num/mod.rs:1040:5
     |
1040 | /     uint_impl! {
1040 | /     uint_impl! {
1041 | |         Self = u16,
1042 | |         ActualT = u16,
1043 | |         SignedT = i16,
...    |
1057 | |         bound_condition = "",
1058 | |     }
     | |_____- in this macro invocation
     = note: candidate #2 is defined in an impl for the type `u16`
     = note: this error originates in the macro `int_module` which comes from the expansion of the macro `uint_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
    --> ../../../library/core/src/num/shells/int_macros.rs:23:33
     |
     |
23   |         pub const MIN: $T = $T::MIN;
     |                                 ^^^ multiple `MIN` found
    ::: ../../../library/core/src/num/shells/u32.rs:11:1
     |
     |
11   | int_module! { u32 }
     |
     |
note: candidate #1 is defined in an impl for the type `u32`
    --> ../../../library/core/src/num/uint_macros.rs:36:9
36   |           pub const MIN: Self = 0;
     |           ^^^^^^^^^^^^^^^^^^^
     |
    ::: ../../../library/core/src/num/mod.rs:1088:5
---
...    |
1105 | |         bound_condition = "",
1106 | |     }
     | |_____- in this macro invocation
     = note: candidate #2 is defined in an impl for the type `u32`
     = note: this error originates in the macro `int_module` which comes from the expansion of the macro `uint_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
    --> ../../../library/core/src/num/shells/int_macros.rs:42:33
     |
     |
42   |         pub const MAX: $T = $T::MAX;
     |                                 ^^^ multiple `MAX` found
    ::: ../../../library/core/src/num/shells/u32.rs:11:1
     |
     |
11   | int_module! { u32 }
     |
     |
note: candidate #1 is defined in an impl for the type `u32`
    --> ../../../library/core/src/num/uint_macros.rs:49:9
     |
49   |           pub const MAX: Self = !0;
     |
    ::: ../../../library/core/src/num/mod.rs:1088:5
     |
1088 | /     uint_impl! {
1088 | /     uint_impl! {
1089 | |         Self = u32,
1090 | |         ActualT = u32,
1091 | |         SignedT = i32,
...    |
1105 | |         bound_condition = "",
1106 | |     }
     | |_____- in this macro invocation
     = note: candidate #2 is defined in an impl for the type `u32`
     = note: this error originates in the macro `int_module` which comes from the expansion of the macro `uint_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
    --> ../../../library/core/src/num/shells/int_macros.rs:23:33
     |
     |
23   |         pub const MIN: $T = $T::MIN;
     |                                 ^^^ multiple `MIN` found
    ::: ../../../library/core/src/num/shells/u64.rs:11:1
     |
     |
11   | int_module! { u64 }
     |
     |
note: candidate #1 is defined in an impl for the type `u64`
    --> ../../../library/core/src/num/uint_macros.rs:36:9
36   |           pub const MIN: Self = 0;
     |           ^^^^^^^^^^^^^^^^^^^
     |
    ::: ../../../library/core/src/num/mod.rs:1111:5
---
...    |
1128 | |         bound_condition = "",
1129 | |     }
     | |_____- in this macro invocation
     = note: candidate #2 is defined in an impl for the type `u64`
     = note: this error originates in the macro `int_module` which comes from the expansion of the macro `uint_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
    --> ../../../library/core/src/num/shells/int_macros.rs:42:33
     |
     |
42   |         pub const MAX: $T = $T::MAX;
     |                                 ^^^ multiple `MAX` found
    ::: ../../../library/core/src/num/shells/u64.rs:11:1
     |
     |
11   | int_module! { u64 }
     |
     |
note: candidate #1 is defined in an impl for the type `u64`
    --> ../../../library/core/src/num/uint_macros.rs:49:9
     |
49   |           pub const MAX: Self = !0;
     |
    ::: ../../../library/core/src/num/mod.rs:1111:5
     |
1111 | /     uint_impl! {
1111 | /     uint_impl! {
1112 | |         Self = u64,
1113 | |         ActualT = u64,
1114 | |         SignedT = i64,
...    |
1128 | |         bound_condition = "",
1129 | |     }
     | |_____- in this macro invocation
     = note: candidate #2 is defined in an impl for the type `u64`
     = note: this error originates in the macro `int_module` which comes from the expansion of the macro `uint_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/num/shells/int_macros.rs:23:33
    |
    |
23  |         pub const MIN: $T = $T::MIN;
    |                                 ^^^ multiple `MIN` found
   ::: ../../../library/core/src/num/shells/u8.rs:11:1
    |
    |
11  | int_module! { u8 }
    |
    |
note: candidate #1 is defined in an impl for the type `u8`
   --> ../../../library/core/src/num/uint_macros.rs:36:9
36  |           pub const MIN: Self = 0;
    |           ^^^^^^^^^^^^^^^^^^^
    |
   ::: ../../../library/core/src/num/mod.rs:438:5
---
...   |
455 | |         bound_condition = "",
456 | |     }
    | |_____- in this macro invocation
    = note: candidate #2 is defined in an impl for the type `u8`
    = note: this error originates in the macro `int_module` which comes from the expansion of the macro `uint_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/num/shells/int_macros.rs:42:33
    |
    |
42  |         pub const MAX: $T = $T::MAX;
    |                                 ^^^ multiple `MAX` found
   ::: ../../../library/core/src/num/shells/u8.rs:11:1
    |
    |
11  | int_module! { u8 }
    |
    |
note: candidate #1 is defined in an impl for the type `u8`
   --> ../../../library/core/src/num/uint_macros.rs:49:9
    |
49  |           pub const MAX: Self = !0;
    |
   ::: ../../../library/core/src/num/mod.rs:438:5
    |
438 | /     uint_impl! {
438 | /     uint_impl! {
439 | |         Self = u8,
440 | |         ActualT = u8,
441 | |         SignedT = i8,
...   |
455 | |         bound_condition = "",
456 | |     }
    | |_____- in this macro invocation
    = note: candidate #2 is defined in an impl for the type `u8`
    = note: this error originates in the macro `int_module` which comes from the expansion of the macro `uint_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
    --> ../../../library/core/src/num/shells/int_macros.rs:23:33
     |
     |
23   |         pub const MIN: $T = $T::MIN;
     |                                 ^^^ multiple `MIN` found
    ::: ../../../library/core/src/num/shells/usize.rs:11:1
     |
     |
11   | int_module! { usize }
     |
     |
note: candidate #1 is defined in an impl for the type `usize`
    --> ../../../library/core/src/num/uint_macros.rs:36:9
36   |           pub const MIN: Self = 0;
     |           ^^^^^^^^^^^^^^^^^^^
     |
    ::: ../../../library/core/src/num/mod.rs:1207:5
    ::: ../../../library/core/src/num/mod.rs:1207:5
     |
1207 | /     uint_impl! {
1208 | |         Self = usize,
1209 | |         ActualT = u64,
1210 | |         SignedT = isize,
...    |
1224 | |         bound_condition = " on 64-bit targets",
     | |_____- in this macro invocation
     | |_____- in this macro invocation
     = note: candidate #2 is defined in an impl for the type `usize`
     = note: this error originates in the macro `int_module` which comes from the expansion of the macro `uint_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
    --> ../../../library/core/src/num/shells/int_macros.rs:42:33
     |
     |
42   |         pub const MAX: $T = $T::MAX;
     |                                 ^^^ multiple `MAX` found
    ::: ../../../library/core/src/num/shells/usize.rs:11:1
     |
     |
11   | int_module! { usize }
     |
     |
note: candidate #1 is defined in an impl for the type `usize`
    --> ../../../library/core/src/num/uint_macros.rs:49:9
     |
49   |           pub const MAX: Self = !0;
     |
    ::: ../../../library/core/src/num/mod.rs:1207:5
     |
1207 | /     uint_impl! {
1207 | /     uint_impl! {
1208 | |         Self = usize,
1209 | |         ActualT = u64,
1210 | |         SignedT = isize,
...    |
1224 | |         bound_condition = " on 64-bit targets",
     | |_____- in this macro invocation
     | |_____- in this macro invocation
     = note: candidate #2 is defined in an impl for the type `usize`
     = note: this error originates in the macro `int_module` which comes from the expansion of the macro `uint_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/num/f32.rs:35:29
    |
    |
35  | pub const RADIX: u32 = f32::RADIX;
    |                             ^^^^^ multiple `RADIX` found
    |
note: candidate #1 is defined in an impl for the type `f32`
   --> ../../../library/core/src/num/f32.rs:353:5
353 |     pub const RADIX: u32 = 2;
    |     ^^^^^^^^^^^^^^^^^^^^
    |     ^^^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl for the type `f32`
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/num/f32.rs:55:39
    |
    |
55  | pub const MANTISSA_DIGITS: u32 = f32::MANTISSA_DIGITS;
    |                                       ^^^^^^^^^^^^^^^ multiple `MANTISSA_DIGITS` found
    |
note: candidate #1 is defined in an impl for the type `f32`
   --> ../../../library/core/src/num/f32.rs:357:5
357 |     pub const MANTISSA_DIGITS: u32 = 24;
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl for the type `f32`
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/num/f32.rs:72:30
    |
    |
72  | pub const DIGITS: u32 = f32::DIGITS;
    |                              ^^^^^^ multiple `DIGITS` found
    |
note: candidate #1 is defined in an impl for the type `f32`
   --> ../../../library/core/src/num/f32.rs:361:5
361 |     pub const DIGITS: u32 = 6;
    |     ^^^^^^^^^^^^^^^^^^^^^
    |     ^^^^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl for the type `f32`
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/num/f32.rs:93:31
    |
    |
93  | pub const EPSILON: f32 = f32::EPSILON;
    |                               ^^^^^^^ multiple `EPSILON` found
    |
note: candidate #1 is defined in an impl for the type `f32`
   --> ../../../library/core/src/num/f32.rs:369:5
    |
369 |     pub const EPSILON: f32 = 1.19209290e-07_f32;
    |     ^^^^^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl for the type `f32`
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/num/f32.rs:110:27
    |
110 | pub const MIN: f32 = f32::MIN;
110 | pub const MIN: f32 = f32::MIN;
    |                           ^^^ multiple `MIN` found
    |
note: candidate #1 is defined in an impl for the type `f32`
   --> ../../../library/core/src/num/f32.rs:373:5
    |
373 |     pub const MIN: f32 = -3.40282347e+38_f32;
    |     ^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl for the type `f32`
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/num/f32.rs:127:36
    |
127 | pub const MIN_POSITIVE: f32 = f32::MIN_POSITIVE;
127 | pub const MIN_POSITIVE: f32 = f32::MIN_POSITIVE;
    |                                    ^^^^^^^^^^^^ multiple `MIN_POSITIVE` found
    |
note: candidate #1 is defined in an impl for the type `f32`
   --> ../../../library/core/src/num/f32.rs:376:5
    |
376 |     pub const MIN_POSITIVE: f32 = 1.17549435e-38_f32;
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl for the type `f32`
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/num/f32.rs:144:27
    |
144 | pub const MAX: f32 = f32::MAX;
144 | pub const MAX: f32 = f32::MAX;
    |                           ^^^ multiple `MAX` found
    |
note: candidate #1 is defined in an impl for the type `f32`
   --> ../../../library/core/src/num/f32.rs:379:5
    |
379 |     pub const MAX: f32 = 3.40282347e+38_f32;
    |     ^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl for the type `f32`
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/num/f32.rs:161:31
    |
    |
161 | pub const MIN_EXP: i32 = f32::MIN_EXP;
    |                               ^^^^^^^ multiple `MIN_EXP` found
    |
note: candidate #1 is defined in an impl for the type `f32`
   --> ../../../library/core/src/num/f32.rs:383:5
    |
383 |     pub const MIN_EXP: i32 = -125;
    |     ^^^^^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl for the type `f32`
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/num/f32.rs:178:31
    |
    |
178 | pub const MAX_EXP: i32 = f32::MAX_EXP;
    |                               ^^^^^^^ multiple `MAX_EXP` found
    |
note: candidate #1 is defined in an impl for the type `f32`
   --> ../../../library/core/src/num/f32.rs:386:5
386 |     pub const MAX_EXP: i32 = 128;
    |     ^^^^^^^^^^^^^^^^^^^^^^
    |     ^^^^^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl for the type `f32`
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/num/f32.rs:195:34
    |
    |
195 | pub const MIN_10_EXP: i32 = f32::MIN_10_EXP;
    |                                  ^^^^^^^^^^ multiple `MIN_10_EXP` found
    |
note: candidate #1 is defined in an impl for the type `f32`
   --> ../../../library/core/src/num/f32.rs:390:5
    |
390 |     pub const MIN_10_EXP: i32 = -37;
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl for the type `f32`
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/num/f32.rs:212:34
    |
    |
212 | pub const MAX_10_EXP: i32 = f32::MAX_10_EXP;
    |                                  ^^^^^^^^^^ multiple `MAX_10_EXP` found
    |
note: candidate #1 is defined in an impl for the type `f32`
   --> ../../../library/core/src/num/f32.rs:393:5
393 |     pub const MAX_10_EXP: i32 = 38;
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl for the type `f32`
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/num/f32.rs:229:27
    |
    |
229 | pub const NAN: f32 = f32::NAN;
    |                           ^^^ multiple `NAN` found
    |
note: candidate #1 is defined in an impl for the type `f32`
   --> ../../../library/core/src/num/f32.rs:406:5
    |
406 |     pub const NAN: f32 = 0.0_f32 / 0.0_f32;
    |     ^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl for the type `f32`
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/num/f32.rs:246:32
    |
246 | pub const INFINITY: f32 = f32::INFINITY;
246 | pub const INFINITY: f32 = f32::INFINITY;
    |                                ^^^^^^^^ multiple `INFINITY` found
    |
note: candidate #1 is defined in an impl for the type `f32`
   --> ../../../library/core/src/num/f32.rs:409:5
    |
409 |     pub const INFINITY: f32 = 1.0_f32 / 0.0_f32;
    |     ^^^^^^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl for the type `f32`
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/num/f32.rs:263:36
    |
263 | pub const NEG_INFINITY: f32 = f32::NEG_INFINITY;
263 | pub const NEG_INFINITY: f32 = f32::NEG_INFINITY;
    |                                    ^^^^^^^^^^^^ multiple `NEG_INFINITY` found
    |
note: candidate #1 is defined in an impl for the type `f32`
   --> ../../../library/core/src/num/f32.rs:412:5
    |
412 |     pub const NEG_INFINITY: f32 = -1.0_f32 / 0.0_f32;
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl for the type `f32`
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/num/f64.rs:35:29
    |
    |
35  | pub const RADIX: u32 = f64::RADIX;
    |                             ^^^^^ multiple `RADIX` found
    |
note: candidate #1 is defined in an impl for the type `f64`
   --> ../../../library/core/src/num/f64.rs:353:5
353 |     pub const RADIX: u32 = 2;
    |     ^^^^^^^^^^^^^^^^^^^^
    |     ^^^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl for the type `f64`
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/num/f64.rs:55:39
    |
    |
55  | pub const MANTISSA_DIGITS: u32 = f64::MANTISSA_DIGITS;
    |                                       ^^^^^^^^^^^^^^^ multiple `MANTISSA_DIGITS` found
    |
note: candidate #1 is defined in an impl for the type `f64`
   --> ../../../library/core/src/num/f64.rs:357:5
357 |     pub const MANTISSA_DIGITS: u32 = 53;
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl for the type `f64`
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/num/f64.rs:72:30
    |
    |
72  | pub const DIGITS: u32 = f64::DIGITS;
    |                              ^^^^^^ multiple `DIGITS` found
    |
note: candidate #1 is defined in an impl for the type `f64`
   --> ../../../library/core/src/num/f64.rs:360:5
360 |     pub const DIGITS: u32 = 15;
    |     ^^^^^^^^^^^^^^^^^^^^^
    |     ^^^^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl for the type `f64`
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/num/f64.rs:93:31
    |
    |
93  | pub const EPSILON: f64 = f64::EPSILON;
    |                               ^^^^^^^ multiple `EPSILON` found
    |
note: candidate #1 is defined in an impl for the type `f64`
   --> ../../../library/core/src/num/f64.rs:368:5
    |
368 |     pub const EPSILON: f64 = 2.2204460492503131e-16_f64;
    |     ^^^^^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl for the type `f64`
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/num/f64.rs:110:27
    |
110 | pub const MIN: f64 = f64::MIN;
110 | pub const MIN: f64 = f64::MIN;
    |                           ^^^ multiple `MIN` found
    |
note: candidate #1 is defined in an impl for the type `f64`
   --> ../../../library/core/src/num/f64.rs:372:5
    |
372 |     pub const MIN: f64 = -1.7976931348623157e+308_f64;
    |     ^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl for the type `f64`
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/num/f64.rs:127:36
    |
127 | pub const MIN_POSITIVE: f64 = f64::MIN_POSITIVE;
127 | pub const MIN_POSITIVE: f64 = f64::MIN_POSITIVE;
    |                                    ^^^^^^^^^^^^ multiple `MIN_POSITIVE` found
    |
note: candidate #1 is defined in an impl for the type `f64`
   --> ../../../library/core/src/num/f64.rs:375:5
    |
375 |     pub const MIN_POSITIVE: f64 = 2.2250738585072014e-308_f64;
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl for the type `f64`
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/num/f64.rs:144:27
    |
144 | pub const MAX: f64 = f64::MAX;
144 | pub const MAX: f64 = f64::MAX;
    |                           ^^^ multiple `MAX` found
    |
note: candidate #1 is defined in an impl for the type `f64`
   --> ../../../library/core/src/num/f64.rs:378:5
    |
378 |     pub const MAX: f64 = 1.7976931348623157e+308_f64;
    |     ^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl for the type `f64`
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/num/f64.rs:161:31
    |
    |
161 | pub const MIN_EXP: i32 = f64::MIN_EXP;
    |                               ^^^^^^^ multiple `MIN_EXP` found
    |
note: candidate #1 is defined in an impl for the type `f64`
   --> ../../../library/core/src/num/f64.rs:382:5
    |
382 |     pub const MIN_EXP: i32 = -1021;
    |     ^^^^^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl for the type `f64`
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/num/f64.rs:178:31
    |
    |
178 | pub const MAX_EXP: i32 = f64::MAX_EXP;
    |                               ^^^^^^^ multiple `MAX_EXP` found
    |
note: candidate #1 is defined in an impl for the type `f64`
   --> ../../../library/core/src/num/f64.rs:385:5
385 |     pub const MAX_EXP: i32 = 1024;
    |     ^^^^^^^^^^^^^^^^^^^^^^
    |     ^^^^^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl for the type `f64`
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/num/f64.rs:195:34
    |
    |
195 | pub const MIN_10_EXP: i32 = f64::MIN_10_EXP;
    |                                  ^^^^^^^^^^ multiple `MIN_10_EXP` found
    |
note: candidate #1 is defined in an impl for the type `f64`
   --> ../../../library/core/src/num/f64.rs:389:5
    |
389 |     pub const MIN_10_EXP: i32 = -307;
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl for the type `f64`
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/num/f64.rs:212:34
    |
    |
212 | pub const MAX_10_EXP: i32 = f64::MAX_10_EXP;
    |                                  ^^^^^^^^^^ multiple `MAX_10_EXP` found
    |
note: candidate #1 is defined in an impl for the type `f64`
   --> ../../../library/core/src/num/f64.rs:392:5
392 |     pub const MAX_10_EXP: i32 = 308;
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl for the type `f64`
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/num/f64.rs:229:27
    |
    |
229 | pub const NAN: f64 = f64::NAN;
    |                           ^^^ multiple `NAN` found
    |
note: candidate #1 is defined in an impl for the type `f64`
   --> ../../../library/core/src/num/f64.rs:405:5
    |
405 |     pub const NAN: f64 = 0.0_f64 / 0.0_f64;
    |     ^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl for the type `f64`
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/num/f64.rs:246:32
    |
246 | pub const INFINITY: f64 = f64::INFINITY;
246 | pub const INFINITY: f64 = f64::INFINITY;
    |                                ^^^^^^^^ multiple `INFINITY` found
    |
note: candidate #1 is defined in an impl for the type `f64`
   --> ../../../library/core/src/num/f64.rs:408:5
    |
408 |     pub const INFINITY: f64 = 1.0_f64 / 0.0_f64;
    |     ^^^^^^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl for the type `f64`
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/num/f64.rs:263:36
    |
263 | pub const NEG_INFINITY: f64 = f64::NEG_INFINITY;
263 | pub const NEG_INFINITY: f64 = f64::NEG_INFINITY;
    |                                    ^^^^^^^^^^^^ multiple `NEG_INFINITY` found
    |
note: candidate #1 is defined in an impl for the type `f64`
   --> ../../../library/core/src/num/f64.rs:411:5
    |
411 |     pub const NEG_INFINITY: f64 = -1.0_f64 / 0.0_f64;
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl for the type `f64`
error[E0034]: multiple applicable items in scope
  --> ../../../library/core/src/char/mod.rs:91:29
   |
   |
91 | pub const MAX: char = char::MAX;
   |                             ^^^ multiple `MAX` found
   |
note: candidate #1 is defined in an impl for the type `char`
  --> ../../../library/core/src/char/methods.rs:25:5
   |
25 |     pub const MAX: char = '\u{10ffff}';
   |     ^^^^^^^^^^^^^^^^^^^
   = note: candidate #2 is defined in an impl for the type `char`
error[E0034]: multiple applicable items in scope
  --> ../../../library/core/src/char/mod.rs:96:47
   |
96 | pub const REPLACEMENT_CHARACTER: char = char::REPLACEMENT_CHARACTER;
96 | pub const REPLACEMENT_CHARACTER: char = char::REPLACEMENT_CHARACTER;
   |                                               ^^^^^^^^^^^^^^^^^^^^^ multiple `REPLACEMENT_CHARACTER` found
   |
note: candidate #1 is defined in an impl for the type `char`
  --> ../../../library/core/src/char/methods.rs:33:5
   |
33 |     pub const REPLACEMENT_CHARACTER: char = '\u{FFFD}';
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: candidate #2 is defined in an impl for the type `char`
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/char/mod.rs:101:49
    |
    |
101 | pub const UNICODE_VERSION: (u8, u8, u8) = char::UNICODE_VERSION;
    |                                                 ^^^^^^^^^^^^^^^ multiple `UNICODE_VERSION` found
    |
note: candidate #1 is defined in an impl for the type `char`
   --> ../../../library/core/src/char/methods.rs:46:5
    |
46  |     pub const UNICODE_VERSION: (u8, u8, u8) = crate::unicode::UNICODE_VERSION;
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl for the type `char`
error[E0034]: multiple applicable items in scope
---
...    |
1057 | |         bound_condition = "",
1058 | |     }
     | |_____- in this macro invocation
     = note: candidate #2 is defined in an impl for the type `u16`
     = note: this error originates in the macro `uint_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
    --> ../../../library/core/src/str/pattern.rs:1906:34
     |
     |
1906 |         let (pxend, pyend) = (px.add(x.len() - 4), py.add(y.len() - 4));
     |                                  ^^^ multiple `add` found
     |
note: candidate #1 is defined in an impl for the type `*const T`
    --> ../../../library/core/src/ptr/const_ptr.rs:915:5
915  | /     pub const unsafe fn add(self, count: usize) -> Self
916  | |     where
917  | |         T: Sized,
     | |_________________^
     | |_________________^
     = note: candidate #2 is defined in an impl for the type `*const T`
error[E0034]: multiple applicable items in scope
    --> ../../../library/core/src/str/pattern.rs:1906:55
     |
     |
1906 |         let (pxend, pyend) = (px.add(x.len() - 4), py.add(y.len() - 4));
     |                                                       ^^^ multiple `add` found
     |
note: candidate #1 is defined in an impl for the type `*const T`
    --> ../../../library/core/src/ptr/const_ptr.rs:915:5
915  | /     pub const unsafe fn add(self, count: usize) -> Self
916  | |     where
917  | |         T: Sized,
     | |_________________^
     | |_________________^
     = note: candidate #2 is defined in an impl for the type `*const T`
error[E0034]: multiple applicable items in scope
    --> ../../../library/core/src/str/pattern.rs:1908:41
     |
     |
1908 |             let vx = (px as *const u32).read_unaligned();
     |                                         ^^^^^^^^^^^^^^ multiple `read_unaligned` found
     |
note: candidate #1 is defined in an impl for the type `*const T`
    --> ../../../library/core/src/ptr/const_ptr.rs:1234:5
1234 | /     pub const unsafe fn read_unaligned(self) -> T
1235 | |     where
1236 | |         T: Sized,
     | |_________________^
     | |_________________^
     = note: candidate #2 is defined in an impl for the type `*const T`
error[E0034]: multiple applicable items in scope
    --> ../../../library/core/src/str/pattern.rs:1909:41
     |
     |
1909 |             let vy = (py as *const u32).read_unaligned();
     |                                         ^^^^^^^^^^^^^^ multiple `read_unaligned` found
     |
note: candidate #1 is defined in an impl for the type `*const T`
    --> ../../../library/core/src/ptr/const_ptr.rs:1234:5
1234 | /     pub const unsafe fn read_unaligned(self) -> T
1235 | |     where
1236 | |         T: Sized,
     | |_________________^
     | |_________________^
     = note: candidate #2 is defined in an impl for the type `*const T`
error[E0034]: multiple applicable items in scope
    --> ../../../library/core/src/str/pattern.rs:1913:21
     |
     |
1913 |             px = px.add(4);
     |                     ^^^ multiple `add` found
     |
note: candidate #1 is defined in an impl for the type `*const T`
    --> ../../../library/core/src/ptr/const_ptr.rs:915:5
915  | /     pub const unsafe fn add(self, count: usize) -> Self
916  | |     where
917  | |         T: Sized,
     | |_________________^
     | |_________________^
     = note: candidate #2 is defined in an impl for the type `*const T`
error[E0034]: multiple applicable items in scope
    --> ../../../library/core/src/str/pattern.rs:1914:21
     |
     |
1914 |             py = py.add(4);
     |                     ^^^ multiple `add` found
     |
note: candidate #1 is defined in an impl for the type `*const T`
    --> ../../../library/core/src/ptr/const_ptr.rs:915:5
915  | /     pub const unsafe fn add(self, count: usize) -> Self
916  | |     where
917  | |         T: Sized,
     | |_________________^
     | |_________________^
     = note: candidate #2 is defined in an impl for the type `*const T`
error[E0034]: multiple applicable items in scope
    --> ../../../library/core/src/str/pattern.rs:1916:40
     |
     |
1916 |         let vx = (pxend as *const u32).read_unaligned();
     |                                        ^^^^^^^^^^^^^^ multiple `read_unaligned` found
     |
note: candidate #1 is defined in an impl for the type `*const T`
    --> ../../../library/core/src/ptr/const_ptr.rs:1234:5
1234 | /     pub const unsafe fn read_unaligned(self) -> T
1235 | |     where
1236 | |         T: Sized,
     | |_________________^
     | |_________________^
     = note: candidate #2 is defined in an impl for the type `*const T`
error[E0034]: multiple applicable items in scope
    --> ../../../library/core/src/str/pattern.rs:1917:40
     |
     |
1917 |         let vy = (pyend as *const u32).read_unaligned();
     |                                        ^^^^^^^^^^^^^^ multiple `read_unaligned` found
     |
note: candidate #1 is defined in an impl for the type `*const T`
    --> ../../../library/core/src/ptr/const_ptr.rs:1234:5
1234 | /     pub const unsafe fn read_unaligned(self) -> T
1235 | |     where
1236 | |         T: Sized,
     | |_________________^
     | |_________________^
     = note: candidate #2 is defined in an impl for the type `*const T`
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/str/lossy.rs:87:37
    |
    |
87  |                 for (i, c) in valid.char_indices() {
    |                                     ^^^^^^^^^^^^ multiple `char_indices` found
    |
note: candidate #1 is defined in an impl for the type `str`
   --> ../../../library/core/src/str/mod.rs:831:5
831 |     pub fn char_indices(&self) -> CharIndices<'_> {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl for the type `str`
error: a method with this name may be added to the standard library in the future
   --> ../../../library/core/src/str/mod.rs:102:23
    |
    |
102 |     let trunc_len = s.floor_char_boundary(MAX_DISPLAY_LENGTH);
    |
    = warning: once this associated item is added to the standard library, the ambiguity may cause an error or change in behavior!
    = note: for more information, see issue #48919 <https://github.com/rust-lang/rust/issues/48919>
    = note: for more information, see issue #48919 <https://github.com/rust-lang/rust/issues/48919>
    = help: call with fully qualified syntax `str::<impl str>::floor_char_boundary(...)` to keep using the current method
    = help: add `#![feature(round_char_boundary)]` to the crate attributes to enable `core::str::<impl str>::floor_char_boundary`
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/str/mod.rs:104:37
    |
    |
104 |     let ellipsis = if trunc_len < s.len() { "[...]" } else { "" };
    |                                     ^^^ multiple `len` found
    |
note: candidate #1 is defined in an impl for the type `str`
   --> ../../../library/core/src/str/mod.rs:159:5
    |
159 |     pub const fn len(&self) -> usize {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl for the type `str`
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/str/mod.rs:107:18
    |
    |
107 |     if begin > s.len() || end > s.len() {
    |                  ^^^ multiple `len` found
    |
note: candidate #1 is defined in an impl for the type `str`
   --> ../../../library/core/src/str/mod.rs:159:5
    |
159 |     pub const fn len(&self) -> usize {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl for the type `str`
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/str/mod.rs:107:35
    |
    |
107 |     if begin > s.len() || end > s.len() {
    |                                   ^^^ multiple `len` found
    |
note: candidate #1 is defined in an impl for the type `str`
   --> ../../../library/core/src/str/mod.rs:159:5
    |
159 |     pub const fn len(&self) -> usize {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl for the type `str`
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/str/mod.rs:108:38
    |
    |
108 |         let oob_index = if begin > s.len() { begin } else { end };
    |                                      ^^^ multiple `len` found
    |
note: candidate #1 is defined in an impl for the type `str`
   --> ../../../library/core/src/str/mod.rs:159:5
    |
159 |     pub const fn len(&self) -> usize {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl for the type `str`
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/str/mod.rs:123:23
    |
    |
123 |     let index = if !s.is_char_boundary(begin) { begin } else { end };
    |                       ^^^^^^^^^^^^^^^^ multiple `is_char_boundary` found
    |
note: candidate #1 is defined in an impl for the type `str`
   --> ../../../library/core/src/str/mod.rs:210:5
    |
210 |     pub fn is_char_boundary(&self, index: usize) -> bool {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl for the type `str`
error: a method with this name may be added to the standard library in the future
   --> ../../../library/core/src/str/mod.rs:125:24
    |
    |
125 |     let char_start = s.floor_char_boundary(index);
    |
    = warning: once this associated item is added to the standard library, the ambiguity may cause an error or change in behavior!
    = note: for more information, see issue #48919 <https://github.com/rust-lang/rust/issues/48919>
    = note: for more information, see issue #48919 <https://github.com/rust-lang/rust/issues/48919>
    = help: call with fully qualified syntax `str::<impl str>::floor_char_boundary(...)` to keep using the current method
    = help: add `#![feature(round_char_boundary)]` to the crate attributes to enable `core::str::<impl str>::floor_char_boundary`
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/str/mod.rs:127:30
    |
    |
127 |     let ch = s[char_start..].chars().next().unwrap();
    |                              ^^^^^ multiple `chars` found
    |
note: candidate #1 is defined in an impl for the type `str`
   --> ../../../library/core/src/str/mod.rs:774:5
    |
774 |     pub fn chars(&self) -> Chars<'_> {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl for the type `str`
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/str/mod.rs:160:14
    |
160 |         self.as_bytes().len()
160 |         self.as_bytes().len()
    |              ^^^^^^^^ multiple `as_bytes` found
    |
note: candidate #1 is defined in an impl for the type `str`
   --> ../../../library/core/src/str/mod.rs:324:5
    |
324 |     pub const fn as_bytes(&self) -> &[u8] {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl for the type `str`
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/str/mod.rs:181:14
    |
181 |         self.len() == 0
181 |         self.len() == 0
    |              ^^^ multiple `len` found
    |
note: candidate #1 is defined in an impl for the type `str`
   --> ../../../library/core/src/str/mod.rs:159:5
    |
159 |     pub const fn len(&self) -> usize {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl for the type `str`
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/str/mod.rs:219:20
    |
219 |         match self.as_bytes().get(index) {
219 |         match self.as_bytes().get(index) {
    |                    ^^^^^^^^ multiple `as_bytes` found
    |
note: candidate #1 is defined in an impl for the type `str`
   --> ../../../library/core/src/str/mod.rs:324:5
    |
324 |     pub const fn as_bytes(&self) -> &[u8] {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl for the type `str`
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/str/mod.rs:229:35
    |
    |
229 |             None => index == self.len(),
    |                                   ^^^ multiple `len` found
    |
note: candidate #1 is defined in an impl for the type `str`
   --> ../../../library/core/src/str/mod.rs:159:5
    |
159 |     pub const fn len(&self) -> usize {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl for the type `str`
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/str/mod.rs:258:26
    |
    |
258 |         if index >= self.len() {
    |                          ^^^ multiple `len` found
    |
note: candidate #1 is defined in an impl for the type `str`
   --> ../../../library/core/src/str/mod.rs:159:5
    |
159 |     pub const fn len(&self) -> usize {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl for the type `str`
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/str/mod.rs:259:18
    |
259 |             self.len()
259 |             self.len()
    |                  ^^^ multiple `len` found
    |
note: candidate #1 is defined in an impl for the type `str`
   --> ../../../library/core/src/str/mod.rs:159:5
    |
159 |     pub const fn len(&self) -> usize {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl for the type `str`
error[E0034]: multiple applicable items in scope
    --> ../../../library/core/src/str/mod.rs:261:37
     |
     |
261  |             let lower_bound = index.saturating_sub(3);
     |                                     ^^^^^^^^^^^^^^ multiple `saturating_sub` found
     |
note: candidate #1 is defined in an impl for the type `usize`
    --> ../../../library/core/src/num/uint_macros.rs:1096:9
     |
1096 |           pub const fn saturating_sub(self, rhs: Self) -> Self {
     |
    ::: ../../../library/core/src/num/mod.rs:1207:5
     |
1207 | /     uint_impl! {
1207 | /     uint_impl! {
1208 | |         Self = usize,
1209 | |         ActualT = u64,
1210 | |         SignedT = isize,
...    |
1224 | |         bound_condition = " on 64-bit targets",
     | |_____- in this macro invocation
     | |_____- in this macro invocation
     = note: candidate #2 is defined in an impl for the type `usize`
     = note: this error originates in the macro `uint_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/str/mod.rs:262:34
    |
    |
262 |             let new_index = self.as_bytes()[lower_bound..=index]
    |                                  ^^^^^^^^ multiple `as_bytes` found
    |
note: candidate #1 is defined in an impl for the type `str`
   --> ../../../library/core/src/str/mod.rs:324:5
    |
324 |     pub const fn as_bytes(&self) -> &[u8] {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl for the type `str`
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/str/mod.rs:297:25
    |
    |
297 |         if index > self.len() {
    |                         ^^^ multiple `len` found
    |
note: candidate #1 is defined in an impl for the type `str`
   --> ../../../library/core/src/str/mod.rs:159:5
    |
159 |     pub const fn len(&self) -> usize {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl for the type `str`
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/str/mod.rs:300:56
    |
    |
300 |             let upper_bound = Ord::min(index + 4, self.len());
    |                                                        ^^^ multiple `len` found
    |
note: candidate #1 is defined in an impl for the type `str`
   --> ../../../library/core/src/str/mod.rs:159:5
    |
159 |     pub const fn len(&self) -> usize {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl for the type `str`
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/str/mod.rs:301:18
    |
    |
301 |             self.as_bytes()[index..upper_bound]
    |                  ^^^^^^^^ multiple `as_bytes` found
    |
note: candidate #1 is defined in an impl for the type `str`
   --> ../../../library/core/src/str/mod.rs:324:5
    |
324 |     pub const fn as_bytes(&self) -> &[u8] {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl for the type `str`
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/str/mod.rs:666:17
    |
    |
666 |         if self.is_char_boundary(mid) {
    |                 ^^^^^^^^^^^^^^^^ multiple `is_char_boundary` found
    |
note: candidate #1 is defined in an impl for the type `str`
   --> ../../../library/core/src/str/mod.rs:210:5
    |
210 |     pub fn is_char_boundary(&self, index: usize) -> bool {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl for the type `str`
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/str/mod.rs:668:28
    |
    |
668 |             unsafe { (self.get_unchecked(0..mid), self.get_unchecked(mid..self.len())) }
    |                            ^^^^^^^^^^^^^ multiple `get_unchecked` found
    |
note: candidate #1 is defined in an impl for the type `str`
   --> ../../../library/core/src/str/mod.rs:503:5
    |
503 |     pub unsafe fn get_unchecked<I: SliceIndex<str>>(&self, i: I) -> &I::Output {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl for the type `str`
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/str/mod.rs:668:56
    |
    |
668 |             unsafe { (self.get_unchecked(0..mid), self.get_unchecked(mid..self.len())) }
    |                                                        ^^^^^^^^^^^^^ multiple `get_unchecked` found
    |
note: candidate #1 is defined in an impl for the type `str`
   --> ../../../library/core/src/str/mod.rs:503:5
    |
503 |     pub unsafe fn get_unchecked<I: SliceIndex<str>>(&self, i: I) -> &I::Output {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl for the type `str`
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/str/mod.rs:668:80
    |
    |
668 |             unsafe { (self.get_unchecked(0..mid), self.get_unchecked(mid..self.len())) }
    |                                                                                ^^^ multiple `len` found
    |
note: candidate #1 is defined in an impl for the type `str`
   --> ../../../library/core/src/str/mod.rs:159:5
    |
159 |     pub const fn len(&self) -> usize {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl for the type `str`
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/str/mod.rs:710:17
    |
    |
710 |         if self.is_char_boundary(mid) {
    |                 ^^^^^^^^^^^^^^^^ multiple `is_char_boundary` found
    |
note: candidate #1 is defined in an impl for the type `str`
   --> ../../../library/core/src/str/mod.rs:210:5
    |
210 |     pub fn is_char_boundary(&self, index: usize) -> bool {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl for the type `str`
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/str/mod.rs:711:28
    |
711 |             let len = self.len();
711 |             let len = self.len();
    |                            ^^^ multiple `len` found
    |
note: candidate #1 is defined in an impl for the type `str`
   --> ../../../library/core/src/str/mod.rs:159:5
    |
159 |     pub const fn len(&self) -> usize {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl for the type `str`
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/str/mod.rs:712:28
    |
712 |             let ptr = self.as_mut_ptr();
712 |             let ptr = self.as_mut_ptr();
    |                            ^^^^^^^^^^ multiple `as_mut_ptr` found
    |
note: candidate #1 is defined in an impl for the type `str`
   --> ../../../library/core/src/str/mod.rs:414:5
    |
414 |     pub fn as_mut_ptr(&mut self) -> *mut u8 {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl for the type `str`
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/str/mod.rs:775:28
    |
    |
775 |         Chars { iter: self.as_bytes().iter() }
    |                            ^^^^^^^^ multiple `as_bytes` found
    |
note: candidate #1 is defined in an impl for the type `str`
   --> ../../../library/core/src/str/mod.rs:324:5
    |
324 |     pub const fn as_bytes(&self) -> &[u8] {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl for the type `str`
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/str/mod.rs:832:51
---
...   |
310 | |         bound_condition = "",
311 | |     }
    | |_____- in this macro invocation
    = note: candidate #2 is defined in an impl for the type `i32`
    = note: this error originates in the macro `int_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
error[E0034]: multiple applicable items in scope
    --> ../../../library/core/src/../../stdarch/crates/core_arch/src/x86/abm.rs:33:7
33   |     x.leading_zeros()
33   |     x.leading_zeros()
     |       ^^^^^^^^^^^^^ multiple `leading_zeros` found
     |
note: candidate #1 is defined in an impl for the type `u32`
    --> ../../../library/core/src/num/uint_macros.rs:148:9
148  |           pub const fn leading_zeros(self) -> u32 {
     |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
    ::: ../../../library/core/src/num/mod.rs:1088:5
---
...    |
1105 | |         bound_condition = "",
1106 | |     }
     | |_____- in this macro invocation
     = note: candidate #2 is defined in an impl for the type `u32`
     = note: this error originates in the macro `uint_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/../../stdarch/crates/core_arch/src/x86/abm.rs:44:7
44  |     x.count_ones() as i32
    |       ^^^^^^^^^^ multiple `count_ones` found
    |
    |
note: candidate #1 is defined in an impl for the type `i32`
   --> ../../../library/core/src/num/int_macros.rs:108:9
    |
108 |           pub const fn count_ones(self) -> u32 { (self as $UnsignedT).count_ones() }
    |
   ::: ../../../library/core/src/num/mod.rs:292:5
    |
292 | /     int_impl! {
292 | /     int_impl! {
293 | |         Self = i32,
294 | |         ActualT = i32,
295 | |         UnsignedT = u32,
...   |
310 | |         bound_condition = "",
311 | |     }
    | |_____- in this macro invocation
    = note: candidate #2 is defined in an impl for the type `i32`
    = note: this error originates in the macro `int_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
error[E0034]: multiple applicable items in scope
    --> ../../../library/core/src/../../stdarch/crates/core_arch/src/x86/bmi1.rs:61:11
     |
61   |     x & x.wrapping_neg()
     |           ^^^^^^^^^^^^ multiple `wrapping_neg` found
     |
note: candidate #1 is defined in an impl for the type `u32`
    --> ../../../library/core/src/num/uint_macros.rs:1377:9
1377 |           pub const fn wrapping_neg(self) -> Self {
     |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
    ::: ../../../library/core/src/num/mod.rs:1088:5
---
...    |
1105 | |         bound_condition = "",
1106 | |     }
     | |_____- in this macro invocation
     = note: candidate #2 is defined in an impl for the type `u32`
     = note: this error originates in the macro `uint_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
error[E0034]: multiple applicable items in scope
    --> ../../../library/core/src/../../stdarch/crates/core_arch/src/x86/bmi1.rs:72:12
     |
72   |     x ^ (x.wrapping_sub(1_u32))
     |            ^^^^^^^^^^^^ multiple `wrapping_sub` found
     |
note: candidate #1 is defined in an impl for the type `u32`
    --> ../../../library/core/src/num/uint_macros.rs:1229:9
     |
1229 |           pub const fn wrapping_sub(self, rhs: Self) -> Self {
     |
    ::: ../../../library/core/src/num/mod.rs:1088:5
     |
1088 | /     uint_impl! {
1088 | /     uint_impl! {
1089 | |         Self = u32,
1090 | |         ActualT = u32,
1091 | |         SignedT = i32,
...    |
1105 | |         bound_condition = "",
1106 | |     }
     | |_____- in this macro invocation
     = note: candidate #2 is defined in an impl for the type `u32`
     = note: this error originates in the macro `uint_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
error[E0034]: multiple applicable items in scope
    --> ../../../library/core/src/../../stdarch/crates/core_arch/src/x86/bmi1.rs:85:12
     |
85   |     x & (x.wrapping_sub(1))
     |            ^^^^^^^^^^^^ multiple `wrapping_sub` found
     |
note: candidate #1 is defined in an impl for the type `u32`
    --> ../../../library/core/src/num/uint_macros.rs:1229:9
     |
1229 |           pub const fn wrapping_sub(self, rhs: Self) -> Self {
     |
    ::: ../../../library/core/src/num/mod.rs:1088:5
     |
1088 | /     uint_impl! {
1088 | /     uint_impl! {
1089 | |         Self = u32,
1090 | |         ActualT = u32,
1091 | |         SignedT = i32,
...    |
1105 | |         bound_condition = "",
1106 | |     }
     | |_____- in this macro invocation
     = note: candidate #2 is defined in an impl for the type `u32`
     = note: this error originates in the macro `uint_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
error[E0034]: multiple applicable items in scope
    --> ../../../library/core/src/../../stdarch/crates/core_arch/src/x86/bmi1.rs:98:7
98   |     x.trailing_zeros()
98   |     x.trailing_zeros()
     |       ^^^^^^^^^^^^^^ multiple `trailing_zeros` found
     |
note: candidate #1 is defined in an impl for the type `u32`
    --> ../../../library/core/src/num/uint_macros.rs:169:9
169  |           pub const fn trailing_zeros(self) -> u32 {
     |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
    ::: ../../../library/core/src/num/mod.rs:1088:5
---
...    |
1105 | |         bound_condition = "",
1106 | |     }
     | |_____- in this macro invocation
     = note: candidate #2 is defined in an impl for the type `u32`
     = note: this error originates in the macro `uint_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
error[E0034]: multiple applicable items in scope
    --> ../../../library/core/src/../../stdarch/crates/core_arch/src/x86/bmi1.rs:111:7
111  |     x.trailing_zeros() as i32
111  |     x.trailing_zeros() as i32
     |       ^^^^^^^^^^^^^^ multiple `trailing_zeros` found
     |
note: candidate #1 is defined in an impl for the type `u32`
    --> ../../../library/core/src/num/uint_macros.rs:169:9
169  |           pub const fn trailing_zeros(self) -> u32 {
     |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
    ::: ../../../library/core/src/num/mod.rs:1088:5
---
...    |
1105 | |         bound_condition = "",
1106 | |     }
     | |_____- in this macro invocation
     = note: candidate #2 is defined in an impl for the type `u32`
     = note: this error originates in the macro `uint_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
error[E0034]: multiple applicable items in scope
    --> ../../../library/core/src/../../stdarch/crates/core_arch/src/x86/tbm.rs:75:12
     |
75   |     x & (x.wrapping_add(1))
     |            ^^^^^^^^^^^^ multiple `wrapping_add` found
     |
note: candidate #1 is defined in an impl for the type `u32`
    --> ../../../library/core/src/num/uint_macros.rs:1188:9
     |
1188 |           pub const fn wrapping_add(self, rhs: Self) -> Self {
     |
    ::: ../../../library/core/src/num/mod.rs:1088:5
     |
1088 | /     uint_impl! {
1088 | /     uint_impl! {
1089 | |         Self = u32,
1090 | |         ActualT = u32,
1091 | |         SignedT = i32,
...    |
1105 | |         bound_condition = "",
1106 | |     }
     | |_____- in this macro invocation
     = note: candidate #2 is defined in an impl for the type `u32`
     = note: this error originates in the macro `uint_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
error[E0034]: multiple applicable items in scope
    --> ../../../library/core/src/../../stdarch/crates/core_arch/src/x86/tbm.rs:87:12
     |
87   |     x & (x.wrapping_add(1))
     |            ^^^^^^^^^^^^ multiple `wrapping_add` found
     |
note: candidate #1 is defined in an impl for the type `u64`
    --> ../../../library/core/src/num/uint_macros.rs:1188:9
     |
1188 |           pub const fn wrapping_add(self, rhs: Self) -> Self {
     |
    ::: ../../../library/core/src/num/mod.rs:1111:5
     |
1111 | /     uint_impl! {
1111 | /     uint_impl! {
1112 | |         Self = u64,
1113 | |         ActualT = u64,
1114 | |         SignedT = i64,
...    |
1128 | |         bound_condition = "",
1129 | |     }
     | |_____- in this macro invocation
     = note: candidate #2 is defined in an impl for the type `u64`
     = note: this error originates in the macro `uint_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
error[E0034]: multiple applicable items in scope
    --> ../../../library/core/src/../../stdarch/crates/core_arch/src/x86/tbm.rs:98:13
     |
98   |     x | !(x.wrapping_add(1))
     |             ^^^^^^^^^^^^ multiple `wrapping_add` found
     |
note: candidate #1 is defined in an impl for the type `u32`
    --> ../../../library/core/src/num/uint_macros.rs:1188:9
     |
1188 |           pub const fn wrapping_add(self, rhs: Self) -> Self {
     |
    ::: ../../../library/core/src/num/mod.rs:1088:5
     |
1088 | /     uint_impl! {
1088 | /     uint_impl! {
1089 | |         Self = u32,
1090 | |         ActualT = u32,
1091 | |         SignedT = i32,
...    |
1105 | |         bound_condition = "",
1106 | |     }
     | |_____- in this macro invocation
     = note: candidate #2 is defined in an impl for the type `u32`
     = note: this error originates in the macro `uint_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
error[E0034]: multiple applicable items in scope
    --> ../../../library/core/src/../../stdarch/crates/core_arch/src/x86/tbm.rs:110:13
     |
110  |     x | !(x.wrapping_add(1))
     |             ^^^^^^^^^^^^ multiple `wrapping_add` found
     |
note: candidate #1 is defined in an impl for the type `u64`
    --> ../../../library/core/src/num/uint_macros.rs:1188:9
     |
1188 |           pub const fn wrapping_add(self, rhs: Self) -> Self {
     |
    ::: ../../../library/core/src/num/mod.rs:1111:5
     |
1111 | /     uint_impl! {
1111 | /     uint_impl! {
1112 | |         Self = u64,
1113 | |         ActualT = u64,
1114 | |         SignedT = i64,
...    |
1128 | |         bound_condition = "",
1129 | |     }
     | |_____- in this macro invocation
     = note: candidate #2 is defined in an impl for the type `u64`
     = note: this error originates in the macro `uint_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
error[E0034]: multiple applicable items in scope
    --> ../../../library/core/src/../../stdarch/crates/core_arch/src/x86/tbm.rs:121:13
     |
121  |     !x & (x.wrapping_add(1))
     |             ^^^^^^^^^^^^ multiple `wrapping_add` found
     |
note: candidate #1 is defined in an impl for the type `u32`
    --> ../../../library/core/src/num/uint_macros.rs:1188:9
     |
1188 |           pub const fn wrapping_add(self, rhs: Self) -> Self {
     |
    ::: ../../../library/core/src/num/mod.rs:1088:5
     |
1088 | /     uint_impl! {
1088 | /     uint_impl! {
1089 | |         Self = u32,
1090 | |         ActualT = u32,
1091 | |         SignedT = i32,
...    |
1105 | |         bound_condition = "",
1106 | |     }
     | |_____- in this macro invocation
     = note: candidate #2 is defined in an impl for the type `u32`
     = note: this error originates in the macro `uint_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
error[E0034]: multiple applicable items in scope
    --> ../../../library/core/src/../../stdarch/crates/core_arch/src/x86/tbm.rs:133:13
     |
133  |     !x & (x.wrapping_add(1))
     |             ^^^^^^^^^^^^ multiple `wrapping_add` found
     |
note: candidate #1 is defined in an impl for the type `u64`
    --> ../../../library/core/src/num/uint_macros.rs:1188:9
     |
1188 |           pub const fn wrapping_add(self, rhs: Self) -> Self {
     |
    ::: ../../../library/core/src/num/mod.rs:1111:5
     |
1111 | /     uint_impl! {
1111 | /     uint_impl! {
1112 | |         Self = u64,
1113 | |         ActualT = u64,
1114 | |         SignedT = i64,
...    |
1128 | |         bound_condition = "",
1129 | |     }
     | |_____- in this macro invocation
     = note: candidate #2 is defined in an impl for the type `u64`
     = note: this error originates in the macro `uint_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
error[E0034]: multiple applicable items in scope
    --> ../../../library/core/src/../../stdarch/crates/core_arch/src/x86/tbm.rs:145:12
     |
145  |     x ^ (x.wrapping_add(1))
     |            ^^^^^^^^^^^^ multiple `wrapping_add` found
     |
note: candidate #1 is defined in an impl for the type `u32`
    --> ../../../library/core/src/num/uint_macros.rs:1188:9
     |
1188 |           pub const fn wrapping_add(self, rhs: Self) -> Self {
     |
    ::: ../../../library/core/src/num/mod.rs:1088:5
     |
1088 | /     uint_impl! {
1088 | /     uint_impl! {
1089 | |         Self = u32,
1090 | |         ActualT = u32,
1091 | |         SignedT = i32,
...    |
1105 | |         bound_condition = "",
1106 | |     }
     | |_____- in this macro invocation
     = note: candidate #2 is defined in an impl for the type `u32`
     = note: this error originates in the macro `uint_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
error[E0034]: multiple applicable items in scope
    --> ../../../library/core/src/../../stdarch/crates/core_arch/src/x86/tbm.rs:158:12
     |
158  |     x ^ (x.wrapping_add(1))
     |            ^^^^^^^^^^^^ multiple `wrapping_add` found
     |
note: candidate #1 is defined in an impl for the type `u64`
    --> ../../../library/core/src/num/uint_macros.rs:1188:9
     |
1188 |           pub const fn wrapping_add(self, rhs: Self) -> Self {
     |
    ::: ../../../library/core/src/num/mod.rs:1111:5
     |
1111 | /     uint_impl! {
1111 | /     uint_impl! {
1112 | |         Self = u64,
1113 | |         ActualT = u64,
1114 | |         SignedT = i64,
...    |
1128 | |         bound_condition = "",
1129 | |     }
     | |_____- in this macro invocation
     = note: candidate #2 is defined in an impl for the type `u64`
     = note: this error originates in the macro `uint_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
error[E0034]: multiple applicable items in scope
    --> ../../../library/core/src/../../stdarch/crates/core_arch/src/x86/tbm.rs:169:12
     |
169  |     x | (x.wrapping_add(1))
     |            ^^^^^^^^^^^^ multiple `wrapping_add` found
     |
note: candidate #1 is defined in an impl for the type `u32`
    --> ../../../library/core/src/num/uint_macros.rs:1188:9
     |
1188 |           pub const fn wrapping_add(self, rhs: Self) -> Self {
     |
    ::: ../../../library/core/src/num/mod.rs:1088:5
     |
1088 | /     uint_impl! {
1088 | /     uint_impl! {
1089 | |         Self = u32,
1090 | |         ActualT = u32,
1091 | |         SignedT = i32,
...    |
1105 | |         bound_condition = "",
1106 | |     }
     | |_____- in this macro invocation
     = note: candidate #2 is defined in an impl for the type `u32`
     = note: this error originates in the macro `uint_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
error[E0034]: multiple applicable items in scope
    --> ../../../library/core/src/../../stdarch/crates/core_arch/src/x86/tbm.rs:181:11
     |
181  |     x | x.wrapping_add(1)
     |           ^^^^^^^^^^^^ multiple `wrapping_add` found
     |
note: candidate #1 is defined in an impl for the type `u64`
    --> ../../../library/core/src/num/uint_macros.rs:1188:9
     |
1188 |           pub const fn wrapping_add(self, rhs: Self) -> Self {
     |
    ::: ../../../library/core/src/num/mod.rs:1111:5
     |
1111 | /     uint_impl! {
1111 | /     uint_impl! {
1112 | |         Self = u64,
1113 | |         ActualT = u64,
1114 | |         SignedT = i64,
...    |
1128 | |         bound_condition = "",
1129 | |     }
     | |_____- in this macro invocation
     = note: candidate #2 is defined in an impl for the type `u64`
     = note: this error originates in the macro `uint_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
error[E0034]: multiple applicable items in scope
    --> ../../../library/core/src/../../stdarch/crates/core_arch/src/x86/tbm.rs:192:12
     |
192  |     x | (x.wrapping_sub(1))
     |            ^^^^^^^^^^^^ multiple `wrapping_sub` found
     |
note: candidate #1 is defined in an impl for the type `u32`
    --> ../../../library/core/src/num/uint_macros.rs:1229:9
     |
1229 |           pub const fn wrapping_sub(self, rhs: Self) -> Self {
     |
    ::: ../../../library/core/src/num/mod.rs:1088:5
     |
1088 | /     uint_impl! {
1088 | /     uint_impl! {
1089 | |         Self = u32,
1090 | |         ActualT = u32,
1091 | |         SignedT = i32,
...    |
1105 | |         bound_condition = "",
1106 | |     }
     | |_____- in this macro invocation
     = note: candidate #2 is defined in an impl for the type `u32`
     = note: this error originates in the macro `uint_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
error[E0034]: multiple applicable items in scope
    --> ../../../library/core/src/../../stdarch/crates/core_arch/src/x86/tbm.rs:204:12
     |
204  |     x | (x.wrapping_sub(1))
     |            ^^^^^^^^^^^^ multiple `wrapping_sub` found
     |
note: candidate #1 is defined in an impl for the type `u64`
    --> ../../../library/core/src/num/uint_macros.rs:1229:9
     |
1229 |           pub const fn wrapping_sub(self, rhs: Self) -> Self {
     |
    ::: ../../../library/core/src/num/mod.rs:1111:5
     |
1111 | /     uint_impl! {
1111 | /     uint_impl! {
1112 | |         Self = u64,
1113 | |         ActualT = u64,
1114 | |         SignedT = i64,
...    |
1128 | |         bound_condition = "",
1129 | |     }
     | |_____- in this macro invocation
     = note: candidate #2 is defined in an impl for the type `u64`
     = note: this error originates in the macro `uint_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
error[E0034]: multiple applicable items in scope
    --> ../../../library/core/src/../../stdarch/crates/core_arch/src/x86/tbm.rs:215:13
     |
215  |     !x | (x.wrapping_sub(1))
     |             ^^^^^^^^^^^^ multiple `wrapping_sub` found
     |
note: candidate #1 is defined in an impl for the type `u32`
    --> ../../../library/core/src/num/uint_macros.rs:1229:9
     |
1229 |           pub const fn wrapping_sub(self, rhs: Self) -> Self {
     |
    ::: ../../../library/core/src/num/mod.rs:1088:5
     |
1088 | /     uint_impl! {
1088 | /     uint_impl! {
1089 | |         Self = u32,
1090 | |         ActualT = u32,
1091 | |         SignedT = i32,
...    |
1105 | |         bound_condition = "",
1106 | |     }
     | |_____- in this macro invocation
     = note: candidate #2 is defined in an impl for the type `u32`
     = note: this error originates in the macro `uint_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
error[E0034]: multiple applicable items in scope
    --> ../../../library/core/src/../../stdarch/crates/core_arch/src/x86/tbm.rs:227:13
     |
227  |     !x | (x.wrapping_sub(1))
     |             ^^^^^^^^^^^^ multiple `wrapping_sub` found
     |
note: candidate #1 is defined in an impl for the type `u64`
    --> ../../../library/core/src/num/uint_macros.rs:1229:9
     |
1229 |           pub const fn wrapping_sub(self, rhs: Self) -> Self {
     |
    ::: ../../../library/core/src/num/mod.rs:1111:5
     |
1111 | /     uint_impl! {
1111 | /     uint_impl! {
1112 | |         Self = u64,
1113 | |         ActualT = u64,
1114 | |         SignedT = i64,
...    |
1128 | |         bound_condition = "",
1129 | |     }
     | |_____- in this macro invocation
     = note: candidate #2 is defined in an impl for the type `u64`
     = note: this error originates in the macro `uint_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
error[E0034]: multiple applicable items in scope
    --> ../../../library/core/src/../../stdarch/crates/core_arch/src/x86/tbm.rs:239:13
     |
239  |     !x | (x.wrapping_add(1))
     |             ^^^^^^^^^^^^ multiple `wrapping_add` found
     |
note: candidate #1 is defined in an impl for the type `u32`
    --> ../../../library/core/src/num/uint_macros.rs:1188:9
     |
1188 |           pub const fn wrapping_add(self, rhs: Self) -> Self {
     |
    ::: ../../../library/core/src/num/mod.rs:1088:5
     |
1088 | /     uint_impl! {
1088 | /     uint_impl! {
1089 | |         Self = u32,
1090 | |         ActualT = u32,
1091 | |         SignedT = i32,
...    |
1105 | |         bound_condition = "",
1106 | |     }
     | |_____- in this macro invocation
     = note: candidate #2 is defined in an impl for the type `u32`
     = note: this error originates in the macro `uint_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
error[E0034]: multiple applicable items in scope
    --> ../../../library/core/src/../../stdarch/crates/core_arch/src/x86/tbm.rs:252:13
     |
252  |     !x | (x.wrapping_add(1))
     |             ^^^^^^^^^^^^ multiple `wrapping_add` found
     |
note: candidate #1 is defined in an impl for the type `u64`
    --> ../../../library/core/src/num/uint_macros.rs:1188:9
     |
1188 |           pub const fn wrapping_add(self, rhs: Self) -> Self {
     |
    ::: ../../../library/core/src/num/mod.rs:1111:5
     |
1111 | /     uint_impl! {
1111 | /     uint_impl! {
1112 | |         Self = u64,
1113 | |         ActualT = u64,
1114 | |         SignedT = i64,
...    |
1128 | |         bound_condition = "",
1129 | |     }
     | |_____- in this macro invocation
     = note: candidate #2 is defined in an impl for the type `u64`
     = note: this error originates in the macro `uint_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
error[E0034]: multiple applicable items in scope
    --> ../../../library/core/src/../../stdarch/crates/core_arch/src/x86/tbm.rs:264:13
     |
264  |     !x & (x.wrapping_sub(1))
     |             ^^^^^^^^^^^^ multiple `wrapping_sub` found
     |
note: candidate #1 is defined in an impl for the type `u32`
    --> ../../../library/core/src/num/uint_macros.rs:1229:9
     |
1229 |           pub const fn wrapping_sub(self, rhs: Self) -> Self {
     |
    ::: ../../../library/core/src/num/mod.rs:1088:5
     |
1088 | /     uint_impl! {
1088 | /     uint_impl! {
1089 | |         Self = u32,
1090 | |         ActualT = u32,
1091 | |         SignedT = i32,
...    |
1105 | |         bound_condition = "",
1106 | |     }
     | |_____- in this macro invocation
     = note: candidate #2 is defined in an impl for the type `u32`
     = note: this error originates in the macro `uint_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
error[E0034]: multiple applicable items in scope
    --> ../../../library/core/src/../../stdarch/crates/core_arch/src/x86/tbm.rs:277:13
     |
277  |     !x & (x.wrapping_sub(1))
     |             ^^^^^^^^^^^^ multiple `wrapping_sub` found
     |
note: candidate #1 is defined in an impl for the type `u64`
    --> ../../../library/core/src/num/uint_macros.rs:1229:9
     |
1229 |           pub const fn wrapping_sub(self, rhs: Self) -> Self {
     |
    ::: ../../../library/core/src/num/mod.rs:1111:5
     |
1111 | /     uint_impl! {
1111 | /     uint_impl! {
1112 | |         Self = u64,
1113 | |         ActualT = u64,
1114 | |         SignedT = i64,
...    |
1128 | |         bound_condition = "",
1129 | |     }
     | |_____- in this macro invocation
     = note: candidate #2 is defined in an impl for the type `u64`
     = note: this error originates in the macro `uint_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
     --> ../../../library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:25471:65
      |
      |
25471 |     _mm512_and_epi32(_mm512_xor_epi32(a, _mm512_set1_epi32(u32::MAX as i32)), b)
      |                                                                 ^^^ multiple `MAX` found
      |
note: candidate #1 is defined in an impl for the type `u32`
     --> ../../../library/core/src/num/uint_macros.rs:49:9
      |
49    |           pub const MAX: Self = !0;
      |
     ::: ../../../library/core/src/num/mod.rs:1088:5
      |
1088  | /     uint_impl! {
1088  | /     uint_impl! {
1089  | |         Self = u32,
1090  | |         ActualT = u32,
1091  | |         SignedT = i32,
...     |
1105  | |         bound_condition = "",
1106  | |     }
      | |_____- in this macro invocation
      = note: candidate #2 is defined in an impl for the type `u32`
      = note: this error originates in the macro `uint_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
     --> ../../../library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:25514:58
      |
      |
25514 |     let not = _mm256_xor_epi32(a, _mm256_set1_epi32(u32::MAX as i32));
      |                                                          ^^^ multiple `MAX` found
      |
note: candidate #1 is defined in an impl for the type `u32`
     --> ../../../library/core/src/num/uint_macros.rs:49:9
      |
49    |           pub const MAX: Self = !0;
      |
     ::: ../../../library/core/src/num/mod.rs:1088:5
      |
1088  | /     uint_impl! {
1088  | /     uint_impl! {
1089  | |         Self = u32,
1090  | |         ActualT = u32,
1091  | |         SignedT = i32,
---
...    |
1128 | |         bound_condition = "",
1129 | |     }
     | |_____- in this macro invocation
     = note: candidate #2 is defined in an impl for the type `u64`
     = note: this error originates in the macro `uint_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/../../stdarch/crates/core_arch/src/x86_64/abm.rs:44:7
44  |     x.count_ones() as i32
    |       ^^^^^^^^^^ multiple `count_ones` found
    |
    |
note: candidate #1 is defined in an impl for the type `i64`
   --> ../../../library/core/src/num/int_macros.rs:108:9
    |
108 |           pub const fn count_ones(self) -> u32 { (self as $UnsignedT).count_ones() }
    |
   ::: ../../../library/core/src/num/mod.rs:315:5
    |
315 | /     int_impl! {
315 | /     int_impl! {
316 | |         Self = i64,
317 | |         ActualT = i64,
318 | |         UnsignedT = u64,
...   |
333 | |         bound_condition = "",
334 | |     }
    | |_____- in this macro invocation
    = note: candidate #2 is defined in an impl for the type `i64`
    = note: this error originates in the macro `int_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
error[E0034]: multiple applicable items in scope
    --> ../../../library/core/src/../../stdarch/crates/core_arch/src/x86_64/bmi.rs:64:11
     |
64   |     x & x.wrapping_neg()
     |           ^^^^^^^^^^^^ multiple `wrapping_neg` found
     |
note: candidate #1 is defined in an impl for the type `u64`
    --> ../../../library/core/src/num/uint_macros.rs:1377:9
1377 |           pub const fn wrapping_neg(self) -> Self {
     |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
    ::: ../../../library/core/src/num/mod.rs:1111:5
---
...    |
1128 | |         bound_condition = "",
1129 | |     }
     | |_____- in this macro invocation
     = note: candidate #2 is defined in an impl for the type `u64`
     = note: this error originates in the macro `uint_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
error[E0034]: multiple applicable items in scope
    --> ../../../library/core/src/../../stdarch/crates/core_arch/src/x86_64/bmi.rs:76:12
     |
76   |     x ^ (x.wrapping_sub(1_u64))
     |            ^^^^^^^^^^^^ multiple `wrapping_sub` found
     |
note: candidate #1 is defined in an impl for the type `u64`
    --> ../../../library/core/src/num/uint_macros.rs:1229:9
     |
1229 |           pub const fn wrapping_sub(self, rhs: Self) -> Self {
     |
    ::: ../../../library/core/src/num/mod.rs:1111:5
     |
1111 | /     uint_impl! {
1111 | /     uint_impl! {
1112 | |         Self = u64,
1113 | |         ActualT = u64,
1114 | |         SignedT = i64,
...    |
1128 | |         bound_condition = "",
1129 | |     }
     | |_____- in this macro invocation
     = note: candidate #2 is defined in an impl for the type `u64`
     = note: this error originates in the macro `uint_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
error[E0034]: multiple applicable items in scope
    --> ../../../library/core/src/../../stdarch/crates/core_arch/src/x86_64/bmi.rs:90:12
     |
90   |     x & (x.wrapping_sub(1))
     |            ^^^^^^^^^^^^ multiple `wrapping_sub` found
     |
note: candidate #1 is defined in an impl for the type `u64`
    --> ../../../library/core/src/num/uint_macros.rs:1229:9
     |
1229 |           pub const fn wrapping_sub(self, rhs: Self) -> Self {
     |
    ::: ../../../library/core/src/num/mod.rs:1111:5
     |
1111 | /     uint_impl! {
1111 | /     uint_impl! {
1112 | |         Self = u64,
1113 | |         ActualT = u64,
1114 | |         SignedT = i64,
...    |
1128 | |         bound_condition = "",
1129 | |     }
     | |_____- in this macro invocation
     = note: candidate #2 is defined in an impl for the type `u64`
     = note: this error originates in the macro `uint_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
error[E0034]: multiple applicable items in scope
    --> ../../../library/core/src/../../stdarch/crates/core_arch/src/x86_64/bmi.rs:103:7
103  |     x.trailing_zeros() as u64
103  |     x.trailing_zeros() as u64
     |       ^^^^^^^^^^^^^^ multiple `trailing_zeros` found
     |
note: candidate #1 is defined in an impl for the type `u64`
    --> ../../../library/core/src/num/uint_macros.rs:169:9
169  |           pub const fn trailing_zeros(self) -> u32 {
     |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
    ::: ../../../library/core/src/num/mod.rs:1111:5
---
...    |
1128 | |         bound_condition = "",
1129 | |     }
     | |_____- in this macro invocation
     = note: candidate #2 is defined in an impl for the type `u64`
     = note: this error originates in the macro `uint_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
error[E0034]: multiple applicable items in scope
    --> ../../../library/core/src/../../stdarch/crates/core_arch/src/x86_64/bmi.rs:116:7
116  |     x.trailing_zeros() as i64
116  |     x.trailing_zeros() as i64
     |       ^^^^^^^^^^^^^^ multiple `trailing_zeros` found
     |
note: candidate #1 is defined in an impl for the type `u64`
    --> ../../../library/core/src/num/uint_macros.rs:169:9
169  |           pub const fn trailing_zeros(self) -> u32 {
     |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
    ::: ../../../library/core/src/num/mod.rs:1111:5
---
...    |
1128 | |         bound_condition = "",
1129 | |     }
     | |_____- in this macro invocation
     = note: candidate #2 is defined in an impl for the type `u64`
     = note: this error originates in the macro `uint_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/../../stdarch/crates/core_arch/src/x86_64/bswap.rs:15:7
    |
15  |     x.swap_bytes()
15  |     x.swap_bytes()
    |       ^^^^^^^^^^ multiple `swap_bytes` found
    |
note: candidate #1 is defined in an impl for the type `i64`
   --> ../../../library/core/src/num/int_macros.rs:279:9
279 |           pub const fn swap_bytes(self) -> Self {
    |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
   ::: ../../../library/core/src/num/mod.rs:315:5
---
...   |
333 | |         bound_condition = "",
334 | |     }
    | |_____- in this macro invocation
    = note: candidate #2 is defined in an impl for the type `i64`
    = note: this error originates in the macro `int_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/../../portable-simd/crates/core_simd/src/elements/float.rs:238:50
    |
    |
238 |                 self * Self::splat(Self::Scalar::to_degrees(1.))
    |                                                  ^^^^^^^^^^ multiple `to_degrees` found
...
357 | impl_trait! { f32 { bits: u32, mask: i32 }, f64 { bits: u64, mask: i64 } }
    |
    |
note: candidate #1 is defined in an impl for the type `f32`
   --> ../../../library/core/src/num/f32.rs:809:5
    |
809 |     pub fn to_degrees(self) -> f32 {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl for the type `f32`
    = note: this error originates in the macro `impl_trait` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/../../portable-simd/crates/core_simd/src/elements/float.rs:243:50
    |
    |
243 |                 self * Self::splat(Self::Scalar::to_radians(1.))
    |                                                  ^^^^^^^^^^ multiple `to_radians` found
...
357 | impl_trait! { f32 { bits: u32, mask: i32 }, f64 { bits: u64, mask: i64 } }
    |
    |
note: candidate #1 is defined in an impl for the type `f32`
   --> ../../../library/core/src/num/f32.rs:828:5
    |
828 |     pub fn to_radians(self) -> f32 {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl for the type `f32`
    = note: this error originates in the macro `impl_trait` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/../../portable-simd/crates/core_simd/src/elements/float.rs:264:62
    |
    |
264 |                 self.abs().simd_eq(Self::splat(Self::Scalar::INFINITY))
    |                                                              ^^^^^^^^ multiple `INFINITY` found
...
357 | impl_trait! { f32 { bits: u32, mask: i32 }, f64 { bits: u64, mask: i64 } }
    |
    |
note: candidate #1 is defined in an impl for the type `f32`
   --> ../../../library/core/src/num/f32.rs:409:5
    |
409 |     pub const INFINITY: f32 = 1.0_f32 / 0.0_f32;
    |     ^^^^^^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl for the type `f32`
    = note: this error originates in the macro `impl_trait` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/../../portable-simd/crates/core_simd/src/elements/float.rs:269:62
    |
    |
269 |                 self.abs().simd_lt(Self::splat(Self::Scalar::INFINITY))
    |                                                              ^^^^^^^^ multiple `INFINITY` found
...
357 | impl_trait! { f32 { bits: u32, mask: i32 }, f64 { bits: u64, mask: i64 } }
    |
    |
note: candidate #1 is defined in an impl for the type `f32`
   --> ../../../library/core/src/num/f32.rs:409:5
    |
409 |     pub const INFINITY: f32 = 1.0_f32 / 0.0_f32;
    |     ^^^^^^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl for the type `f32`
    = note: this error originates in the macro `impl_trait` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/../../portable-simd/crates/core_simd/src/elements/float.rs:274:100
    |
    |
274 |                 self.abs().simd_ne(Self::splat(0.0)) & (self.to_bits() & Self::splat(Self::Scalar::INFINITY).to_bits()).simd_eq(Simd::spl...
    |                                                                                                    ^^^^^^^^ multiple `INFINITY` found
...
357 | impl_trait! { f32 { bits: u32, mask: i32 }, f64 { bits: u64, mask: i64 } }
    |
    |
note: candidate #1 is defined in an impl for the type `f32`
   --> ../../../library/core/src/num/f32.rs:409:5
    |
409 |     pub const INFINITY: f32 = 1.0_f32 / 0.0_f32;
    |     ^^^^^^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl for the type `f32`
    = note: this error originates in the macro `impl_trait` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/../../portable-simd/crates/core_simd/src/elements/float.rs:285:64
    |
    |
285 |                 self.is_nan().select(Self::splat(Self::Scalar::NAN), Self::splat(1.0).copysign(self))
    |                                                                ^^^ multiple `NAN` found
...
357 | impl_trait! { f32 { bits: u32, mask: i32 }, f64 { bits: u64, mask: i64 } }
    |
    |
note: candidate #1 is defined in an impl for the type `f32`
   --> ../../../library/core/src/num/f32.rs:406:5
    |
406 |     pub const NAN: f32 = 0.0_f32 / 0.0_f32;
    |     ^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl for the type `f32`
    = note: this error originates in the macro `impl_trait` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/../../portable-simd/crates/core_simd/src/elements/float.rs:238:50
    |
    |
238 |                 self * Self::splat(Self::Scalar::to_degrees(1.))
    |                                                  ^^^^^^^^^^ multiple `to_degrees` found
...
357 | impl_trait! { f32 { bits: u32, mask: i32 }, f64 { bits: u64, mask: i64 } }
    |
    |
note: candidate #1 is defined in an impl for the type `f64`
   --> ../../../library/core/src/num/f64.rs:819:5
    |
819 |     pub fn to_degrees(self) -> f64 {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl for the type `f64`
    = note: this error originates in the macro `impl_trait` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/../../portable-simd/crates/core_simd/src/elements/float.rs:243:50
    |
    |
243 |                 self * Self::splat(Self::Scalar::to_radians(1.))
    |                                                  ^^^^^^^^^^ multiple `to_radians` found
...
357 | impl_trait! { f32 { bits: u32, mask: i32 }, f64 { bits: u64, mask: i64 } }
    |
    |
note: candidate #1 is defined in an impl for the type `f64`
   --> ../../../library/core/src/num/f64.rs:839:5
    |
839 |     pub fn to_radians(self) -> f64 {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl for the type `f64`
    = note: this error originates in the macro `impl_trait` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/../../portable-simd/crates/core_simd/src/elements/float.rs:264:62
    |
    |
264 |                 self.abs().simd_eq(Self::splat(Self::Scalar::INFINITY))
    |                                                              ^^^^^^^^ multiple `INFINITY` found
...
357 | impl_trait! { f32 { bits: u32, mask: i32 }, f64 { bits: u64, mask: i64 } }
    |
    |
note: candidate #1 is defined in an impl for the type `f64`
   --> ../../../library/core/src/num/f64.rs:408:5
    |
408 |     pub const INFINITY: f64 = 1.0_f64 / 0.0_f64;
    |     ^^^^^^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl for the type `f64`
    = note: this error originates in the macro `impl_trait` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/../../portable-simd/crates/core_simd/src/elements/float.rs:269:62
    |
    |
269 |                 self.abs().simd_lt(Self::splat(Self::Scalar::INFINITY))
    |                                                              ^^^^^^^^ multiple `INFINITY` found
...
357 | impl_trait! { f32 { bits: u32, mask: i32 }, f64 { bits: u64, mask: i64 } }
    |
    |
note: candidate #1 is defined in an impl for the type `f64`
   --> ../../../library/core/src/num/f64.rs:408:5
    |
408 |     pub const INFINITY: f64 = 1.0_f64 / 0.0_f64;
    |     ^^^^^^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl for the type `f64`
    = note: this error originates in the macro `impl_trait` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/../../portable-simd/crates/core_simd/src/elements/float.rs:274:100
    |
    |
274 |                 self.abs().simd_ne(Self::splat(0.0)) & (self.to_bits() & Self::splat(Self::Scalar::INFINITY).to_bits()).simd_eq(Simd::spl...
    |                                                                                                    ^^^^^^^^ multiple `INFINITY` found
...
357 | impl_trait! { f32 { bits: u32, mask: i32 }, f64 { bits: u64, mask: i64 } }
    |
    |
note: candidate #1 is defined in an impl for the type `f64`
   --> ../../../library/core/src/num/f64.rs:408:5
    |
408 |     pub const INFINITY: f64 = 1.0_f64 / 0.0_f64;
    |     ^^^^^^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl for the type `f64`
    = note: this error originates in the macro `impl_trait` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/../../portable-simd/crates/core_simd/src/elements/float.rs:285:64
    |
    |
285 |                 self.is_nan().select(Self::splat(Self::Scalar::NAN), Self::splat(1.0).copysign(self))
    |                                                                ^^^ multiple `NAN` found
...
357 | impl_trait! { f32 { bits: u32, mask: i32 }, f64 { bits: u64, mask: i64 } }
    |
    |
note: candidate #1 is defined in an impl for the type `f64`
   --> ../../../library/core/src/num/f64.rs:405:5
    |
405 |     pub const NAN: f64 = 0.0_f64 / 0.0_f64;
    |     ^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl for the type `f64`
    = note: this error originates in the macro `impl_trait` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/../../portable-simd/crates/core_simd/src/masks/full_masks.rs:85:35
    |
    |
85  |                 let rev = <$int>::reverse_bits(self);
    |                                   ^^^^^^^^^^^^ multiple `reverse_bits` found
...
99  | impl_reverse_bits! { u8, u16, u32, u64 }
    |
    |
note: candidate #1 is defined in an impl for the type `u8`
   --> ../../../library/core/src/num/uint_macros.rs:303:9
303 |           pub const fn reverse_bits(self) -> Self {
    |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
   ::: ../../../library/core/src/num/mod.rs:438:5
---
...   |
455 | |         bound_condition = "",
456 | |     }
    | |_____- in this macro invocation
    = note: candidate #2 is defined in an impl for the type `u8`
    = note: this error originates in the macro `impl_reverse_bits` which comes from the expansion of the macro `uint_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
    --> ../../../library/core/src/../../portable-simd/crates/core_simd/src/masks/full_masks.rs:85:35
     |
     |
85   |                 let rev = <$int>::reverse_bits(self);
     |                                   ^^^^^^^^^^^^ multiple `reverse_bits` found
...
99   | impl_reverse_bits! { u8, u16, u32, u64 }
     |
     |
note: candidate #1 is defined in an impl for the type `u16`
    --> ../../../library/core/src/num/uint_macros.rs:303:9
303  |           pub const fn reverse_bits(self) -> Self {
     |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
    ::: ../../../library/core/src/num/mod.rs:1040:5
---
...    |
1057 | |         bound_condition = "",
1058 | |     }
     | |_____- in this macro invocation
     = note: candidate #2 is defined in an impl for the type `u16`
     = note: this error originates in the macro `impl_reverse_bits` which comes from the expansion of the macro `uint_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
    --> ../../../library/core/src/../../portable-simd/crates/core_simd/src/masks/full_masks.rs:85:35
     |
     |
85   |                 let rev = <$int>::reverse_bits(self);
     |                                   ^^^^^^^^^^^^ multiple `reverse_bits` found
...
99   | impl_reverse_bits! { u8, u16, u32, u64 }
     |
     |
note: candidate #1 is defined in an impl for the type `u32`
    --> ../../../library/core/src/num/uint_macros.rs:303:9
303  |           pub const fn reverse_bits(self) -> Self {
     |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
    ::: ../../../library/core/src/num/mod.rs:1088:5
---
...    |
1105 | |         bound_condition = "",
1106 | |     }
     | |_____- in this macro invocation
     = note: candidate #2 is defined in an impl for the type `u32`
     = note: this error originates in the macro `impl_reverse_bits` which comes from the expansion of the macro `uint_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
    --> ../../../library/core/src/../../portable-simd/crates/core_simd/src/masks/full_masks.rs:85:35
     |
     |
85   |                 let rev = <$int>::reverse_bits(self);
     |                                   ^^^^^^^^^^^^ multiple `reverse_bits` found
...
99   | impl_reverse_bits! { u8, u16, u32, u64 }
     |
     |
note: candidate #1 is defined in an impl for the type `u64`
    --> ../../../library/core/src/num/uint_macros.rs:303:9
303  |           pub const fn reverse_bits(self) -> Self {
     |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
    ::: ../../../library/core/src/num/mod.rs:1111:5
---
...    |
1128 | |         bound_condition = "",
1129 | |     }
     | |_____- in this macro invocation
     = note: candidate #2 is defined in an impl for the type `u64`
     = note: this error originates in the macro `impl_reverse_bits` which comes from the expansion of the macro `uint_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/../../portable-simd/crates/core_simd/src/ops.rs:84:34
    |
    |
84  |               let rhs = if <$int>::MIN != 0 {
    |                                    ^^^ multiple `MIN` found
168 | / for_base_ops! {
168 | / for_base_ops! {
169 | |     T = (i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);
170 | |     type Lhs = Simd<T, N>;
171 | |     type Rhs = Simd<T, N>;
224 | |     }
225 | | }
    | |_- in this macro invocation
    |
    |
note: candidate #1 is defined in an impl for the type `i8`
   --> ../../../library/core/src/num/int_macros.rs:38:9
    |
38  |           pub const MIN: Self = !Self::MAX;
    |
   ::: ../../../library/core/src/num/mod.rs:246:5
    |
246 | /     int_impl! {
246 | /     int_impl! {
247 | |         Self = i8,
248 | |         ActualT = i8,
249 | |         UnsignedT = u8,
...   |
264 | |         bound_condition = "",
265 | |     }
    | |_____- in this macro invocation
    = note: candidate #2 is defined in an impl for the type `i8`
    = note: this error originates in the macro `int_divrem_guard` which comes from the expansion of the macro `int_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/../../portable-simd/crates/core_simd/src/ops.rs:88:51
    |
    |
88  |                   ($lhs.simd_eq(Simd::splat(<$int>::MIN))
    |                                                     ^^^ multiple `MIN` found
168 | / for_base_ops! {
168 | / for_base_ops! {
169 | |     T = (i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);
170 | |     type Lhs = Simd<T, N>;
171 | |     type Rhs = Simd<T, N>;
224 | |     }
225 | | }
    | |_- in this macro invocation
    |
    |
note: candidate #1 is defined in an impl for the type `i8`
   --> ../../../library/core/src/num/int_macros.rs:38:9
    |
38  |           pub const MIN: Self = !Self::MAX;
    |
   ::: ../../../library/core/src/num/mod.rs:246:5
    |
246 | /     int_impl! {
246 | /     int_impl! {
247 | |         Self = i8,
248 | |         ActualT = i8,
249 | |         UnsignedT = u8,
...   |
264 | |         bound_condition = "",
265 | |     }
    | |_____- in this macro invocation
    = note: candidate #2 is defined in an impl for the type `i8`
    = note: this error originates in the macro `int_divrem_guard` which comes from the expansion of the macro `int_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/../../portable-simd/crates/core_simd/src/ops.rs:84:34
    |
    |
84  |               let rhs = if <$int>::MIN != 0 {
    |                                    ^^^ multiple `MIN` found
168 | / for_base_ops! {
168 | / for_base_ops! {
169 | |     T = (i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);
170 | |     type Lhs = Simd<T, N>;
171 | |     type Rhs = Simd<T, N>;
224 | |     }
225 | | }
    | |_- in this macro invocation
    |
    |
note: candidate #1 is defined in an impl for the type `i16`
   --> ../../../library/core/src/num/int_macros.rs:38:9
    |
38  |           pub const MIN: Self = !Self::MAX;
    |
   ::: ../../../library/core/src/num/mod.rs:269:5
    |
269 | /     int_impl! {
269 | /     int_impl! {
270 | |         Self = i16,
271 | |         ActualT = i16,
272 | |         UnsignedT = u16,
...   |
287 | |         bound_condition = "",
288 | |     }
    | |_____- in this macro invocation
    = note: candidate #2 is defined in an impl for the type `i16`
    = note: this error originates in the macro `int_divrem_guard` which comes from the expansion of the macro `int_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/../../portable-simd/crates/core_simd/src/ops.rs:88:51
    |
    |
88  |                   ($lhs.simd_eq(Simd::splat(<$int>::MIN))
    |                                                     ^^^ multiple `MIN` found
168 | / for_base_ops! {
168 | / for_base_ops! {
169 | |     T = (i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);
170 | |     type Lhs = Simd<T, N>;
171 | |     type Rhs = Simd<T, N>;
224 | |     }
225 | | }
    | |_- in this macro invocation
    |
    |
note: candidate #1 is defined in an impl for the type `i16`
   --> ../../../library/core/src/num/int_macros.rs:38:9
    |
38  |           pub const MIN: Self = !Self::MAX;
    |
   ::: ../../../library/core/src/num/mod.rs:269:5
    |
269 | /     int_impl! {
269 | /     int_impl! {
270 | |         Self = i16,
271 | |         ActualT = i16,
272 | |         UnsignedT = u16,
...   |
287 | |         bound_condition = "",
288 | |     }
    | |_____- in this macro invocation
    = note: candidate #2 is defined in an impl for the type `i16`
    = note: this error originates in the macro `int_divrem_guard` which comes from the expansion of the macro `int_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/../../portable-simd/crates/core_simd/src/ops.rs:84:34
    |
    |
84  |               let rhs = if <$int>::MIN != 0 {
    |                                    ^^^ multiple `MIN` found
168 | / for_base_ops! {
168 | / for_base_ops! {
169 | |     T = (i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);
170 | |     type Lhs = Simd<T, N>;
171 | |     type Rhs = Simd<T, N>;
224 | |     }
225 | | }
    | |_- in this macro invocation
    |
    |
note: candidate #1 is defined in an impl for the type `i32`
   --> ../../../library/core/src/num/int_macros.rs:38:9
    |
38  |           pub const MIN: Self = !Self::MAX;
    |
   ::: ../../../library/core/src/num/mod.rs:292:5
    |
292 | /     int_impl! {
292 | /     int_impl! {
293 | |         Self = i32,
294 | |         ActualT = i32,
295 | |         UnsignedT = u32,
...   |
310 | |         bound_condition = "",
311 | |     }
    | |_____- in this macro invocation
    = note: candidate #2 is defined in an impl for the type `i32`
    = note: this error originates in the macro `int_divrem_guard` which comes from the expansion of the macro `int_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/../../portable-simd/crates/core_simd/src/ops.rs:88:51
    |
    |
88  |                   ($lhs.simd_eq(Simd::splat(<$int>::MIN))
    |                                                     ^^^ multiple `MIN` found
168 | / for_base_ops! {
168 | / for_base_ops! {
169 | |     T = (i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);
170 | |     type Lhs = Simd<T, N>;
171 | |     type Rhs = Simd<T, N>;
224 | |     }
225 | | }
    | |_- in this macro invocation
    |
    |
note: candidate #1 is defined in an impl for the type `i32`
   --> ../../../library/core/src/num/int_macros.rs:38:9
    |
38  |           pub const MIN: Self = !Self::MAX;
    |
   ::: ../../../library/core/src/num/mod.rs:292:5
    |
292 | /     int_impl! {
292 | /     int_impl! {
293 | |         Self = i32,
294 | |         ActualT = i32,
295 | |         UnsignedT = u32,
...   |
310 | |         bound_condition = "",
311 | |     }
    | |_____- in this macro invocation
    = note: candidate #2 is defined in an impl for the type `i32`
    = note: this error originates in the macro `int_divrem_guard` which comes from the expansion of the macro `int_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/../../portable-simd/crates/core_simd/src/ops.rs:84:34
    |
    |
84  |               let rhs = if <$int>::MIN != 0 {
    |                                    ^^^ multiple `MIN` found
168 | / for_base_ops! {
168 | / for_base_ops! {
169 | |     T = (i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);
170 | |     type Lhs = Simd<T, N>;
171 | |     type Rhs = Simd<T, N>;
224 | |     }
225 | | }
    | |_- in this macro invocation
    |
    |
note: candidate #1 is defined in an impl for the type `i64`
   --> ../../../library/core/src/num/int_macros.rs:38:9
    |
38  |           pub const MIN: Self = !Self::MAX;
    |
   ::: ../../../library/core/src/num/mod.rs:315:5
    |
315 | /     int_impl! {
315 | /     int_impl! {
316 | |         Self = i64,
317 | |         ActualT = i64,
318 | |         UnsignedT = u64,
...   |
333 | |         bound_condition = "",
334 | |     }
    | |_____- in this macro invocation
    = note: candidate #2 is defined in an impl for the type `i64`
    = note: this error originates in the macro `int_divrem_guard` which comes from the expansion of the macro `int_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/../../portable-simd/crates/core_simd/src/ops.rs:88:51
    |
    |
88  |                   ($lhs.simd_eq(Simd::splat(<$int>::MIN))
    |                                                     ^^^ multiple `MIN` found
168 | / for_base_ops! {
168 | / for_base_ops! {
169 | |     T = (i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);
170 | |     type Lhs = Simd<T, N>;
171 | |     type Rhs = Simd<T, N>;
224 | |     }
225 | | }
    | |_- in this macro invocation
    |
    |
note: candidate #1 is defined in an impl for the type `i64`
   --> ../../../library/core/src/num/int_macros.rs:38:9
    |
38  |           pub const MIN: Self = !Self::MAX;
    |
   ::: ../../../library/core/src/num/mod.rs:315:5
    |
315 | /     int_impl! {
315 | /     int_impl! {
316 | |         Self = i64,
317 | |         ActualT = i64,
318 | |         UnsignedT = u64,
...   |
333 | |         bound_condition = "",
334 | |     }
    | |_____- in this macro invocation
    = note: candidate #2 is defined in an impl for the type `i64`
    = note: this error originates in the macro `int_divrem_guard` which comes from the expansion of the macro `int_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/../../portable-simd/crates/core_simd/src/ops.rs:84:34
    |
    |
84  |               let rhs = if <$int>::MIN != 0 {
    |                                    ^^^ multiple `MIN` found
168 | / for_base_ops! {
168 | / for_base_ops! {
169 | |     T = (i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);
170 | |     type Lhs = Simd<T, N>;
171 | |     type Rhs = Simd<T, N>;
224 | |     }
225 | | }
    | |_- in this macro invocation
    |
    |
note: candidate #1 is defined in an impl for the type `isize`
   --> ../../../library/core/src/num/int_macros.rs:38:9
    |
38  |           pub const MIN: Self = !Self::MAX;
    |
   ::: ../../../library/core/src/num/mod.rs:412:5
    |
412 | /     int_impl! {
412 | /     int_impl! {
413 | |         Self = isize,
414 | |         ActualT = i64,
415 | |         UnsignedT = usize,
...   |
430 | |         bound_condition = " on 64-bit targets",
    | |_____- in this macro invocation
    | |_____- in this macro invocation
    = note: candidate #2 is defined in an impl for the type `isize`
    = note: this error originates in the macro `int_divrem_guard` which comes from the expansion of the macro `int_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/../../portable-simd/crates/core_simd/src/ops.rs:88:51
    |
    |
88  |                   ($lhs.simd_eq(Simd::splat(<$int>::MIN))
    |                                                     ^^^ multiple `MIN` found
168 | / for_base_ops! {
168 | / for_base_ops! {
169 | |     T = (i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);
170 | |     type Lhs = Simd<T, N>;
171 | |     type Rhs = Simd<T, N>;
224 | |     }
225 | | }
    | |_- in this macro invocation
    |
    |
note: candidate #1 is defined in an impl for the type `isize`
   --> ../../../library/core/src/num/int_macros.rs:38:9
    |
38  |           pub const MIN: Self = !Self::MAX;
    |
   ::: ../../../library/core/src/num/mod.rs:412:5
    |
412 | /     int_impl! {
412 | /     int_impl! {
413 | |         Self = isize,
414 | |         ActualT = i64,
415 | |         UnsignedT = usize,
...   |
430 | |         bound_condition = " on 64-bit targets",
    | |_____- in this macro invocation
    | |_____- in this macro invocation
    = note: candidate #2 is defined in an impl for the type `isize`
    = note: this error originates in the macro `int_divrem_guard` which comes from the expansion of the macro `int_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/../../portable-simd/crates/core_simd/src/ops.rs:84:34
    |
    |
84  |               let rhs = if <$int>::MIN != 0 {
    |                                    ^^^ multiple `MIN` found
168 | / for_base_ops! {
168 | / for_base_ops! {
169 | |     T = (i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);
170 | |     type Lhs = Simd<T, N>;
171 | |     type Rhs = Simd<T, N>;
224 | |     }
225 | | }
    | |_- in this macro invocation
    |
    |
note: candidate #1 is defined in an impl for the type `u8`
   --> ../../../library/core/src/num/uint_macros.rs:36:9
36  |           pub const MIN: Self = 0;
    |           ^^^^^^^^^^^^^^^^^^^
    |
   ::: ../../../library/core/src/num/mod.rs:438:5
---
...   |
455 | |         bound_condition = "",
456 | |     }
    | |_____- in this macro invocation
    = note: candidate #2 is defined in an impl for the type `u8`
    = note: this error originates in the macro `int_divrem_guard` which comes from the expansion of the macro `uint_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/../../portable-simd/crates/core_simd/src/ops.rs:88:51
    |
    |
88  |                   ($lhs.simd_eq(Simd::splat(<$int>::MIN))
    |                                                     ^^^ multiple `MIN` found
168 | / for_base_ops! {
168 | / for_base_ops! {
169 | |     T = (i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);
170 | |     type Lhs = Simd<T, N>;
171 | |     type Rhs = Simd<T, N>;
224 | |     }
225 | | }
    | |_- in this macro invocation
    |
    |
note: candidate #1 is defined in an impl for the type `u8`
   --> ../../../library/core/src/num/uint_macros.rs:36:9
36  |           pub const MIN: Self = 0;
    |           ^^^^^^^^^^^^^^^^^^^
    |
   ::: ../../../library/core/src/num/mod.rs:438:5
---
...   |
455 | |         bound_condition = "",
456 | |     }
    | |_____- in this macro invocation
    = note: candidate #2 is defined in an impl for the type `u8`
    = note: this error originates in the macro `int_divrem_guard` which comes from the expansion of the macro `uint_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
    --> ../../../library/core/src/../../portable-simd/crates/core_simd/src/ops.rs:84:34
     |
     |
84   |               let rhs = if <$int>::MIN != 0 {
     |                                    ^^^ multiple `MIN` found
168  | / for_base_ops! {
168  | / for_base_ops! {
169  | |     T = (i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);
170  | |     type Lhs = Simd<T, N>;
171  | |     type Rhs = Simd<T, N>;
224  | |     }
225  | | }
     | |_- in this macro invocation
     |
     |
note: candidate #1 is defined in an impl for the type `u16`
    --> ../../../library/core/src/num/uint_macros.rs:36:9
36   |           pub const MIN: Self = 0;
     |           ^^^^^^^^^^^^^^^^^^^
     |
    ::: ../../../library/core/src/num/mod.rs:1040:5
---
...    |
1057 | |         bound_condition = "",
1058 | |     }
     | |_____- in this macro invocation
     = note: candidate #2 is defined in an impl for the type `u16`
     = note: this error originates in the macro `int_divrem_guard` which comes from the expansion of the macro `uint_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
    --> ../../../library/core/src/../../portable-simd/crates/core_simd/src/ops.rs:88:51
     |
     |
88   |                   ($lhs.simd_eq(Simd::splat(<$int>::MIN))
     |                                                     ^^^ multiple `MIN` found
168  | / for_base_ops! {
168  | / for_base_ops! {
169  | |     T = (i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);
170  | |     type Lhs = Simd<T, N>;
171  | |     type Rhs = Simd<T, N>;
224  | |     }
225  | | }
     | |_- in this macro invocation
     |
     |
note: candidate #1 is defined in an impl for the type `u16`
    --> ../../../library/core/src/num/uint_macros.rs:36:9
36   |           pub const MIN: Self = 0;
     |           ^^^^^^^^^^^^^^^^^^^
     |
    ::: ../../../library/core/src/num/mod.rs:1040:5
---
...    |
1057 | |         bound_condition = "",
1058 | |     }
     | |_____- in this macro invocation
     = note: candidate #2 is defined in an impl for the type `u16`
     = note: this error originates in the macro `int_divrem_guard` which comes from the expansion of the macro `uint_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
    --> ../../../library/core/src/../../portable-simd/crates/core_simd/src/ops.rs:84:34
     |
     |
84   |               let rhs = if <$int>::MIN != 0 {
     |                                    ^^^ multiple `MIN` found
168  | / for_base_ops! {
168  | / for_base_ops! {
169  | |     T = (i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);
170  | |     type Lhs = Simd<T, N>;
171  | |     type Rhs = Simd<T, N>;
224  | |     }
225  | | }
     | |_- in this macro invocation
     |
     |
note: candidate #1 is defined in an impl for the type `u32`
    --> ../../../library/core/src/num/uint_macros.rs:36:9
36   |           pub const MIN: Self = 0;
     |           ^^^^^^^^^^^^^^^^^^^
     |
    ::: ../../../library/core/src/num/mod.rs:1088:5
---
...    |
1105 | |         bound_condition = "",
1106 | |     }
     | |_____- in this macro invocation
     = note: candidate #2 is defined in an impl for the type `u32`
     = note: this error originates in the macro `int_divrem_guard` which comes from the expansion of the macro `uint_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
    --> ../../../library/core/src/../../portable-simd/crates/core_simd/src/ops.rs:88:51
     |
     |
88   |                   ($lhs.simd_eq(Simd::splat(<$int>::MIN))
     |                                                     ^^^ multiple `MIN` found
168  | / for_base_ops! {
168  | / for_base_ops! {
169  | |     T = (i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);
170  | |     type Lhs = Simd<T, N>;
171  | |     type Rhs = Simd<T, N>;
224  | |     }
225  | | }
     | |_- in this macro invocation
     |
     |
note: candidate #1 is defined in an impl for the type `u32`
    --> ../../../library/core/src/num/uint_macros.rs:36:9
36   |           pub const MIN: Self = 0;
     |           ^^^^^^^^^^^^^^^^^^^
     |
    ::: ../../../library/core/src/num/mod.rs:1088:5
---
...    |
1105 | |         bound_condition = "",
1106 | |     }
     | |_____- in this macro invocation
     = note: candidate #2 is defined in an impl for the type `u32`
     = note: this error originates in the macro `int_divrem_guard` which comes from the expansion of the macro `uint_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
    --> ../../../library/core/src/../../portable-simd/crates/core_simd/src/ops.rs:84:34
     |
     |
84   |               let rhs = if <$int>::MIN != 0 {
     |                                    ^^^ multiple `MIN` found
168  | / for_base_ops! {
168  | / for_base_ops! {
169  | |     T = (i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);
170  | |     type Lhs = Simd<T, N>;
171  | |     type Rhs = Simd<T, N>;
224  | |     }
225  | | }
     | |_- in this macro invocation
     |
     |
note: candidate #1 is defined in an impl for the type `u64`
    --> ../../../library/core/src/num/uint_macros.rs:36:9
36   |           pub const MIN: Self = 0;
     |           ^^^^^^^^^^^^^^^^^^^
     |
    ::: ../../../library/core/src/num/mod.rs:1111:5
---
...    |
1128 | |         bound_condition = "",
1129 | |     }
     | |_____- in this macro invocation
     = note: candidate #2 is defined in an impl for the type `u64`
     = note: this error originates in the macro `int_divrem_guard` which comes from the expansion of the macro `uint_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
    --> ../../../library/core/src/../../portable-simd/crates/core_simd/src/ops.rs:88:51
     |
     |
88   |                   ($lhs.simd_eq(Simd::splat(<$int>::MIN))
     |                                                     ^^^ multiple `MIN` found
168  | / for_base_ops! {
168  | / for_base_ops! {
169  | |     T = (i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);
170  | |     type Lhs = Simd<T, N>;
171  | |     type Rhs = Simd<T, N>;
224  | |     }
225  | | }
     | |_- in this macro invocation
     |
     |
note: candidate #1 is defined in an impl for the type `u64`
    --> ../../../library/core/src/num/uint_macros.rs:36:9
36   |           pub const MIN: Self = 0;
     |           ^^^^^^^^^^^^^^^^^^^
     |
    ::: ../../../library/core/src/num/mod.rs:1111:5
---
...    |
1128 | |         bound_condition = "",
1129 | |     }
     | |_____- in this macro invocation
     = note: candidate #2 is defined in an impl for the type `u64`
     = note: this error originates in the macro `int_divrem_guard` which comes from the expansion of the macro `uint_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
    --> ../../../library/core/src/../../portable-simd/crates/core_simd/src/ops.rs:84:34
     |
     |
84   |               let rhs = if <$int>::MIN != 0 {
     |                                    ^^^ multiple `MIN` found
168  | / for_base_ops! {
168  | / for_base_ops! {
169  | |     T = (i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);
170  | |     type Lhs = Simd<T, N>;
171  | |     type Rhs = Simd<T, N>;
224  | |     }
225  | | }
     | |_- in this macro invocation
     |
     |
note: candidate #1 is defined in an impl for the type `usize`
    --> ../../../library/core/src/num/uint_macros.rs:36:9
36   |           pub const MIN: Self = 0;
     |           ^^^^^^^^^^^^^^^^^^^
     |
    ::: ../../../library/core/src/num/mod.rs:1207:5
    ::: ../../../library/core/src/num/mod.rs:1207:5
     |
1207 | /     uint_impl! {
1208 | |         Self = usize,
1209 | |         ActualT = u64,
1210 | |         SignedT = isize,
...    |
1224 | |         bound_condition = " on 64-bit targets",
     | |_____- in this macro invocation
     | |_____- in this macro invocation
     = note: candidate #2 is defined in an impl for the type `usize`
     = note: this error originates in the macro `int_divrem_guard` which comes from the expansion of the macro `uint_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
    --> ../../../library/core/src/../../portable-simd/crates/core_simd/src/ops.rs:88:51
     |
     |
88   |                   ($lhs.simd_eq(Simd::splat(<$int>::MIN))
     |                                                     ^^^ multiple `MIN` found
168  | / for_base_ops! {
168  | / for_base_ops! {
169  | |     T = (i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);
170  | |     type Lhs = Simd<T, N>;
171  | |     type Rhs = Simd<T, N>;
224  | |     }
225  | | }
     | |_- in this macro invocation
     |
     |
note: candidate #1 is defined in an impl for the type `usize`
    --> ../../../library/core/src/num/uint_macros.rs:36:9
36   |           pub const MIN: Self = 0;
     |           ^^^^^^^^^^^^^^^^^^^
     |
    ::: ../../../library/core/src/num/mod.rs:1207:5
    ::: ../../../library/core/src/num/mod.rs:1207:5
     |
1207 | /     uint_impl! {
1208 | |         Self = usize,
1209 | |         ActualT = u64,
1210 | |         SignedT = isize,
...    |
1224 | |         bound_condition = " on 64-bit targets",
     | |_____- in this macro invocation
     | |_____- in this macro invocation
     = note: candidate #2 is defined in an impl for the type `usize`
     = note: this error originates in the macro `int_divrem_guard` which comes from the expansion of the macro `uint_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/../../portable-simd/crates/core_simd/src/ops.rs:84:34
    |
    |
84  |               let rhs = if <$int>::MIN != 0 {
    |                                    ^^^ multiple `MIN` found
168 | / for_base_ops! {
168 | / for_base_ops! {
169 | |     T = (i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);
170 | |     type Lhs = Simd<T, N>;
171 | |     type Rhs = Simd<T, N>;
224 | |     }
225 | | }
    | |_- in this macro invocation
    |
    |
note: candidate #1 is defined in an impl for the type `i8`
   --> ../../../library/core/src/num/int_macros.rs:38:9
    |
38  |           pub const MIN: Self = !Self::MAX;
    |
   ::: ../../../library/core/src/num/mod.rs:246:5
    |
246 | /     int_impl! {
246 | /     int_impl! {
247 | |         Self = i8,
248 | |         ActualT = i8,
249 | |         UnsignedT = u8,
...   |
264 | |         bound_condition = "",
265 | |     }
    | |_____- in this macro invocation
    = note: candidate #2 is defined in an impl for the type `i8`
    = note: this error originates in the macro `int_divrem_guard` which comes from the expansion of the macro `int_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/../../portable-simd/crates/core_simd/src/ops.rs:88:51
    |
    |
88  |                   ($lhs.simd_eq(Simd::splat(<$int>::MIN))
    |                                                     ^^^ multiple `MIN` found
168 | / for_base_ops! {
168 | / for_base_ops! {
169 | |     T = (i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);
170 | |     type Lhs = Simd<T, N>;
171 | |     type Rhs = Simd<T, N>;
224 | |     }
225 | | }
    | |_- in this macro invocation
    |
    |
note: candidate #1 is defined in an impl for the type `i8`
   --> ../../../library/core/src/num/int_macros.rs:38:9
    |
38  |           pub const MIN: Self = !Self::MAX;
    |
   ::: ../../../library/core/src/num/mod.rs:246:5
    |
246 | /     int_impl! {
246 | /     int_impl! {
247 | |         Self = i8,
248 | |         ActualT = i8,
249 | |         UnsignedT = u8,
...   |
264 | |         bound_condition = "",
265 | |     }
    | |_____- in this macro invocation
    = note: candidate #2 is defined in an impl for the type `i8`
    = note: this error originates in the macro `int_divrem_guard` which comes from the expansion of the macro `int_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/../../portable-simd/crates/core_simd/src/ops.rs:84:34
    |
    |
84  |               let rhs = if <$int>::MIN != 0 {
    |                                    ^^^ multiple `MIN` found
168 | / for_base_ops! {
168 | / for_base_ops! {
169 | |     T = (i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);
170 | |     type Lhs = Simd<T, N>;
171 | |     type Rhs = Simd<T, N>;
224 | |     }
225 | | }
    | |_- in this macro invocation
    |
    |
note: candidate #1 is defined in an impl for the type `i16`
   --> ../../../library/core/src/num/int_macros.rs:38:9
    |
38  |           pub const MIN: Self = !Self::MAX;
    |
   ::: ../../../library/core/src/num/mod.rs:269:5
    |
269 | /     int_impl! {
269 | /     int_impl! {
270 | |         Self = i16,
271 | |         ActualT = i16,
272 | |         UnsignedT = u16,
...   |
287 | |         bound_condition = "",
288 | |     }
    | |_____- in this macro invocation
    = note: candidate #2 is defined in an impl for the type `i16`
    = note: this error originates in the macro `int_divrem_guard` which comes from the expansion of the macro `int_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/../../portable-simd/crates/core_simd/src/ops.rs:88:51
    |
    |
88  |                   ($lhs.simd_eq(Simd::splat(<$int>::MIN))
    |                                                     ^^^ multiple `MIN` found
168 | / for_base_ops! {
168 | / for_base_ops! {
169 | |     T = (i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);
170 | |     type Lhs = Simd<T, N>;
171 | |     type Rhs = Simd<T, N>;
224 | |     }
225 | | }
    | |_- in this macro invocation
    |
    |
note: candidate #1 is defined in an impl for the type `i16`
   --> ../../../library/core/src/num/int_macros.rs:38:9
    |
38  |           pub const MIN: Self = !Self::MAX;
    |
   ::: ../../../library/core/src/num/mod.rs:269:5
    |
269 | /     int_impl! {
269 | /     int_impl! {
270 | |         Self = i16,
271 | |         ActualT = i16,
272 | |         UnsignedT = u16,
...   |
287 | |         bound_condition = "",
288 | |     }
    | |_____- in this macro invocation
    = note: candidate #2 is defined in an impl for the type `i16`
    = note: this error originates in the macro `int_divrem_guard` which comes from the expansion of the macro `int_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/../../portable-simd/crates/core_simd/src/ops.rs:84:34
    |
    |
84  |               let rhs = if <$int>::MIN != 0 {
    |                                    ^^^ multiple `MIN` found
168 | / for_base_ops! {
168 | / for_base_ops! {
169 | |     T = (i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);
170 | |     type Lhs = Simd<T, N>;
171 | |     type Rhs = Simd<T, N>;
224 | |     }
225 | | }
    | |_- in this macro invocation
    |
    |
note: candidate #1 is defined in an impl for the type `i32`
   --> ../../../library/core/src/num/int_macros.rs:38:9
    |
38  |           pub const MIN: Self = !Self::MAX;
    |
   ::: ../../../library/core/src/num/mod.rs:292:5
    |
292 | /     int_impl! {
292 | /     int_impl! {
293 | |         Self = i32,
294 | |         ActualT = i32,
295 | |         UnsignedT = u32,
...   |
310 | |         bound_condition = "",
311 | |     }
    | |_____- in this macro invocation
    = note: candidate #2 is defined in an impl for the type `i32`
    = note: this error originates in the macro `int_divrem_guard` which comes from the expansion of the macro `int_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/../../portable-simd/crates/core_simd/src/ops.rs:88:51
    |
    |
88  |                   ($lhs.simd_eq(Simd::splat(<$int>::MIN))
    |                                                     ^^^ multiple `MIN` found
168 | / for_base_ops! {
168 | / for_base_ops! {
169 | |     T = (i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);
170 | |     type Lhs = Simd<T, N>;
171 | |     type Rhs = Simd<T, N>;
224 | |     }
225 | | }
    | |_- in this macro invocation
    |
    |
note: candidate #1 is defined in an impl for the type `i32`
   --> ../../../library/core/src/num/int_macros.rs:38:9
    |
38  |           pub const MIN: Self = !Self::MAX;
    |
   ::: ../../../library/core/src/num/mod.rs:292:5
    |
292 | /     int_impl! {
292 | /     int_impl! {
293 | |         Self = i32,
294 | |         ActualT = i32,
295 | |         UnsignedT = u32,
...   |
310 | |         bound_condition = "",
311 | |     }
    | |_____- in this macro invocation
    = note: candidate #2 is defined in an impl for the type `i32`
    = note: this error originates in the macro `int_divrem_guard` which comes from the expansion of the macro `int_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/../../portable-simd/crates/core_simd/src/ops.rs:84:34
    |
    |
84  |               let rhs = if <$int>::MIN != 0 {
    |                                    ^^^ multiple `MIN` found
168 | / for_base_ops! {
168 | / for_base_ops! {
169 | |     T = (i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);
170 | |     type Lhs = Simd<T, N>;
171 | |     type Rhs = Simd<T, N>;
224 | |     }
225 | | }
    | |_- in this macro invocation
    |
    |
note: candidate #1 is defined in an impl for the type `i64`
   --> ../../../library/core/src/num/int_macros.rs:38:9
    |
38  |           pub const MIN: Self = !Self::MAX;
    |
   ::: ../../../library/core/src/num/mod.rs:315:5
    |
315 | /     int_impl! {
315 | /     int_impl! {
316 | |         Self = i64,
317 | |         ActualT = i64,
318 | |         UnsignedT = u64,
...   |
333 | |         bound_condition = "",
334 | |     }
    | |_____- in this macro invocation
    = note: candidate #2 is defined in an impl for the type `i64`
    = note: this error originates in the macro `int_divrem_guard` which comes from the expansion of the macro `int_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/../../portable-simd/crates/core_simd/src/ops.rs:88:51
    |
    |
88  |                   ($lhs.simd_eq(Simd::splat(<$int>::MIN))
    |                                                     ^^^ multiple `MIN` found
168 | / for_base_ops! {
168 | / for_base_ops! {
169 | |     T = (i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);
170 | |     type Lhs = Simd<T, N>;
171 | |     type Rhs = Simd<T, N>;
224 | |     }
225 | | }
    | |_- in this macro invocation
    |
    |
note: candidate #1 is defined in an impl for the type `i64`
   --> ../../../library/core/src/num/int_macros.rs:38:9
    |
38  |           pub const MIN: Self = !Self::MAX;
    |
   ::: ../../../library/core/src/num/mod.rs:315:5
    |
315 | /     int_impl! {
315 | /     int_impl! {
316 | |         Self = i64,
317 | |         ActualT = i64,
318 | |         UnsignedT = u64,
...   |
333 | |         bound_condition = "",
334 | |     }
    | |_____- in this macro invocation
    = note: candidate #2 is defined in an impl for the type `i64`
    = note: this error originates in the macro `int_divrem_guard` which comes from the expansion of the macro `int_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/../../portable-simd/crates/core_simd/src/ops.rs:84:34
    |
    |
84  |               let rhs = if <$int>::MIN != 0 {
    |                                    ^^^ multiple `MIN` found
168 | / for_base_ops! {
168 | / for_base_ops! {
169 | |     T = (i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);
170 | |     type Lhs = Simd<T, N>;
171 | |     type Rhs = Simd<T, N>;
224 | |     }
225 | | }
    | |_- in this macro invocation
    |
    |
note: candidate #1 is defined in an impl for the type `isize`
   --> ../../../library/core/src/num/int_macros.rs:38:9
    |
38  |           pub const MIN: Self = !Self::MAX;
    |
   ::: ../../../library/core/src/num/mod.rs:412:5
    |
412 | /     int_impl! {
412 | /     int_impl! {
413 | |         Self = isize,
414 | |         ActualT = i64,
415 | |         UnsignedT = usize,
...   |
430 | |         bound_condition = " on 64-bit targets",
    | |_____- in this macro invocation
    | |_____- in this macro invocation
    = note: candidate #2 is defined in an impl for the type `isize`
    = note: this error originates in the macro `int_divrem_guard` which comes from the expansion of the macro `int_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/../../portable-simd/crates/core_simd/src/ops.rs:88:51
    |
    |
88  |                   ($lhs.simd_eq(Simd::splat(<$int>::MIN))
    |                                                     ^^^ multiple `MIN` found
168 | / for_base_ops! {
168 | / for_base_ops! {
169 | |     T = (i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);
170 | |     type Lhs = Simd<T, N>;
171 | |     type Rhs = Simd<T, N>;
224 | |     }
225 | | }
    | |_- in this macro invocation
    |
    |
note: candidate #1 is defined in an impl for the type `isize`
   --> ../../../library/core/src/num/int_macros.rs:38:9
    |
38  |           pub const MIN: Self = !Self::MAX;
    |
   ::: ../../../library/core/src/num/mod.rs:412:5
    |
412 | /     int_impl! {
412 | /     int_impl! {
413 | |         Self = isize,
414 | |         ActualT = i64,
415 | |         UnsignedT = usize,
...   |
430 | |         bound_condition = " on 64-bit targets",
    | |_____- in this macro invocation
    | |_____- in this macro invocation
    = note: candidate #2 is defined in an impl for the type `isize`
    = note: this error originates in the macro `int_divrem_guard` which comes from the expansion of the macro `int_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/../../portable-simd/crates/core_simd/src/ops.rs:84:34
    |
    |
84  |               let rhs = if <$int>::MIN != 0 {
    |                                    ^^^ multiple `MIN` found
168 | / for_base_ops! {
168 | / for_base_ops! {
169 | |     T = (i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);
170 | |     type Lhs = Simd<T, N>;
171 | |     type Rhs = Simd<T, N>;
224 | |     }
225 | | }
    | |_- in this macro invocation
    |
    |
note: candidate #1 is defined in an impl for the type `u8`
   --> ../../../library/core/src/num/uint_macros.rs:36:9
36  |           pub const MIN: Self = 0;
    |           ^^^^^^^^^^^^^^^^^^^
    |
   ::: ../../../library/core/src/num/mod.rs:438:5
---
...   |
455 | |         bound_condition = "",
456 | |     }
    | |_____- in this macro invocation
    = note: candidate #2 is defined in an impl for the type `u8`
    = note: this error originates in the macro `int_divrem_guard` which comes from the expansion of the macro `uint_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/../../portable-simd/crates/core_simd/src/ops.rs:88:51
    |
    |
88  |                   ($lhs.simd_eq(Simd::splat(<$int>::MIN))
    |                                                     ^^^ multiple `MIN` found
168 | / for_base_ops! {
168 | / for_base_ops! {
169 | |     T = (i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);
170 | |     type Lhs = Simd<T, N>;
171 | |     type Rhs = Simd<T, N>;
224 | |     }
225 | | }
    | |_- in this macro invocation
    |
    |
note: candidate #1 is defined in an impl for the type `u8`
   --> ../../../library/core/src/num/uint_macros.rs:36:9
36  |           pub const MIN: Self = 0;
    |           ^^^^^^^^^^^^^^^^^^^
    |
   ::: ../../../library/core/src/num/mod.rs:438:5
---
...   |
455 | |         bound_condition = "",
456 | |     }
    | |_____- in this macro invocation
    = note: candidate #2 is defined in an impl for the type `u8`
    = note: this error originates in the macro `int_divrem_guard` which comes from the expansion of the macro `uint_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
    --> ../../../library/core/src/../../portable-simd/crates/core_simd/src/ops.rs:84:34
     |
     |
84   |               let rhs = if <$int>::MIN != 0 {
     |                                    ^^^ multiple `MIN` found
168  | / for_base_ops! {
168  | / for_base_ops! {
169  | |     T = (i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);
170  | |     type Lhs = Simd<T, N>;
171  | |     type Rhs = Simd<T, N>;
224  | |     }
225  | | }
     | |_- in this macro invocation
     |
     |
note: candidate #1 is defined in an impl for the type `u16`
    --> ../../../library/core/src/num/uint_macros.rs:36:9
36   |           pub const MIN: Self = 0;
     |           ^^^^^^^^^^^^^^^^^^^
     |
    ::: ../../../library/core/src/num/mod.rs:1040:5
---
...    |
1057 | |         bound_condition = "",
1058 | |     }
     | |_____- in this macro invocation
     = note: candidate #2 is defined in an impl for the type `u16`
     = note: this error originates in the macro `int_divrem_guard` which comes from the expansion of the macro `uint_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
    --> ../../../library/core/src/../../portable-simd/crates/core_simd/src/ops.rs:88:51
     |
     |
88   |                   ($lhs.simd_eq(Simd::splat(<$int>::MIN))
     |                                                     ^^^ multiple `MIN` found
168  | / for_base_ops! {
168  | / for_base_ops! {
169  | |     T = (i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);
170  | |     type Lhs = Simd<T, N>;
171  | |     type Rhs = Simd<T, N>;
224  | |     }
225  | | }
     | |_- in this macro invocation
     |
     |
note: candidate #1 is defined in an impl for the type `u16`
    --> ../../../library/core/src/num/uint_macros.rs:36:9
36   |           pub const MIN: Self = 0;
     |           ^^^^^^^^^^^^^^^^^^^
     |
    ::: ../../../library/core/src/num/mod.rs:1040:5
---
...    |
1057 | |         bound_condition = "",
1058 | |     }
     | |_____- in this macro invocation
     = note: candidate #2 is defined in an impl for the type `u16`
     = note: this error originates in the macro `int_divrem_guard` which comes from the expansion of the macro `uint_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
    --> ../../../library/core/src/../../portable-simd/crates/core_simd/src/ops.rs:84:34
     |
     |
84   |               let rhs = if <$int>::MIN != 0 {
     |                                    ^^^ multiple `MIN` found
168  | / for_base_ops! {
168  | / for_base_ops! {
169  | |     T = (i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);
170  | |     type Lhs = Simd<T, N>;
171  | |     type Rhs = Simd<T, N>;
224  | |     }
225  | | }
     | |_- in this macro invocation
     |
     |
note: candidate #1 is defined in an impl for the type `u32`
    --> ../../../library/core/src/num/uint_macros.rs:36:9
36   |           pub const MIN: Self = 0;
     |           ^^^^^^^^^^^^^^^^^^^
     |
    ::: ../../../library/core/src/num/mod.rs:1088:5
---
...    |
1105 | |         bound_condition = "",
1106 | |     }
     | |_____- in this macro invocation
     = note: candidate #2 is defined in an impl for the type `u32`
     = note: this error originates in the macro `int_divrem_guard` which comes from the expansion of the macro `uint_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
    --> ../../../library/core/src/../../portable-simd/crates/core_simd/src/ops.rs:88:51
     |
     |
88   |                   ($lhs.simd_eq(Simd::splat(<$int>::MIN))
     |                                                     ^^^ multiple `MIN` found
168  | / for_base_ops! {
168  | / for_base_ops! {
169  | |     T = (i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);
170  | |     type Lhs = Simd<T, N>;
171  | |     type Rhs = Simd<T, N>;
224  | |     }
225  | | }
     | |_- in this macro invocation
     |
     |
note: candidate #1 is defined in an impl for the type `u32`
    --> ../../../library/core/src/num/uint_macros.rs:36:9
36   |           pub const MIN: Self = 0;
     |           ^^^^^^^^^^^^^^^^^^^
     |
    ::: ../../../library/core/src/num/mod.rs:1088:5
---
...    |
1105 | |         bound_condition = "",
1106 | |     }
     | |_____- in this macro invocation
     = note: candidate #2 is defined in an impl for the type `u32`
     = note: this error originates in the macro `int_divrem_guard` which comes from the expansion of the macro `uint_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
    --> ../../../library/core/src/../../portable-simd/crates/core_simd/src/ops.rs:84:34
     |
     |
84   |               let rhs = if <$int>::MIN != 0 {
     |                                    ^^^ multiple `MIN` found
168  | / for_base_ops! {
168  | / for_base_ops! {
169  | |     T = (i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);
170  | |     type Lhs = Simd<T, N>;
171  | |     type Rhs = Simd<T, N>;
224  | |     }
225  | | }
     | |_- in this macro invocation
     |
     |
note: candidate #1 is defined in an impl for the type `u64`
    --> ../../../library/core/src/num/uint_macros.rs:36:9
36   |           pub const MIN: Self = 0;
     |           ^^^^^^^^^^^^^^^^^^^
     |
    ::: ../../../library/core/src/num/mod.rs:1111:5
---
...    |
1128 | |         bound_condition = "",
1129 | |     }
     | |_____- in this macro invocation
     = note: candidate #2 is defined in an impl for the type `u64`
     = note: this error originates in the macro `int_divrem_guard` which comes from the expansion of the macro `uint_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
    --> ../../../library/core/src/../../portable-simd/crates/core_simd/src/ops.rs:88:51
     |
     |
88   |                   ($lhs.simd_eq(Simd::splat(<$int>::MIN))
     |                                                     ^^^ multiple `MIN` found
168  | / for_base_ops! {
168  | / for_base_ops! {
169  | |     T = (i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);
170  | |     type Lhs = Simd<T, N>;
171  | |     type Rhs = Simd<T, N>;
224  | |     }
225  | | }
     | |_- in this macro invocation
     |
     |
note: candidate #1 is defined in an impl for the type `u64`
    --> ../../../library/core/src/num/uint_macros.rs:36:9
36   |           pub const MIN: Self = 0;
     |           ^^^^^^^^^^^^^^^^^^^
     |
    ::: ../../../library/core/src/num/mod.rs:1111:5
---
...    |
1128 | |         bound_condition = "",
1129 | |     }
     | |_____- in this macro invocation
     = note: candidate #2 is defined in an impl for the type `u64`
     = note: this error originates in the macro `int_divrem_guard` which comes from the expansion of the macro `uint_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
    --> ../../../library/core/src/../../portable-simd/crates/core_simd/src/ops.rs:84:34
     |
     |
84   |               let rhs = if <$int>::MIN != 0 {
     |                                    ^^^ multiple `MIN` found
168  | / for_base_ops! {
168  | / for_base_ops! {
169  | |     T = (i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);
170  | |     type Lhs = Simd<T, N>;
171  | |     type Rhs = Simd<T, N>;
224  | |     }
225  | | }
     | |_- in this macro invocation
     |
     |
note: candidate #1 is defined in an impl for the type `usize`
    --> ../../../library/core/src/num/uint_macros.rs:36:9
36   |           pub const MIN: Self = 0;
     |           ^^^^^^^^^^^^^^^^^^^
     |
    ::: ../../../library/core/src/num/mod.rs:1207:5
    ::: ../../../library/core/src/num/mod.rs:1207:5
     |
1207 | /     uint_impl! {
1208 | |         Self = usize,
1209 | |         ActualT = u64,
1210 | |         SignedT = isize,
...    |
1224 | |         bound_condition = " on 64-bit targets",
     | |_____- in this macro invocation
     | |_____- in this macro invocation
     = note: candidate #2 is defined in an impl for the type `usize`
     = note: this error originates in the macro `int_divrem_guard` which comes from the expansion of the macro `uint_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
    --> ../../../library/core/src/../../portable-simd/crates/core_simd/src/ops.rs:88:51
     |
     |
88   |                   ($lhs.simd_eq(Simd::splat(<$int>::MIN))
     |                                                     ^^^ multiple `MIN` found
168  | / for_base_ops! {
168  | / for_base_ops! {
169  | |     T = (i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);
170  | |     type Lhs = Simd<T, N>;
171  | |     type Rhs = Simd<T, N>;
224  | |     }
225  | | }
     | |_- in this macro invocation
     |
     |
note: candidate #1 is defined in an impl for the type `usize`
    --> ../../../library/core/src/num/uint_macros.rs:36:9
36   |           pub const MIN: Self = 0;
     |           ^^^^^^^^^^^^^^^^^^^
     |
    ::: ../../../library/core/src/num/mod.rs:1207:5
    ::: ../../../library/core/src/num/mod.rs:1207:5
     |
1207 | /     uint_impl! {
1208 | |         Self = usize,
1209 | |         ActualT = u64,
1210 | |         SignedT = isize,
...    |
1224 | |         bound_condition = " on 64-bit targets",
     | |_____- in this macro invocation
     | |_____- in this macro invocation
     = note: candidate #2 is defined in an impl for the type `usize`
     = note: this error originates in the macro `int_divrem_guard` which comes from the expansion of the macro `uint_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/../../portable-simd/crates/core_simd/src/ops.rs:57:49
    |
    |
57  |                   $rhs.bitand(Simd::splat(<$int>::BITS as $int - 1)),
    |                                                   ^^^^ multiple `BITS` found
168 | / for_base_ops! {
168 | / for_base_ops! {
169 | |     T = (i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);
170 | |     type Lhs = Simd<T, N>;
171 | |     type Rhs = Simd<T, N>;
224 | |     }
225 | | }
    | |_- in this macro invocation
    |
    |
note: candidate #1 is defined in an impl for the type `i8`
   --> ../../../library/core/src/num/int_macros.rs:61:9
    |
61  |           pub const BITS: u32 = <$UnsignedT>::BITS;
    |
   ::: ../../../library/core/src/num/mod.rs:246:5
    |
246 | /     int_impl! {
246 | /     int_impl! {
247 | |         Self = i8,
248 | |         ActualT = i8,
249 | |         UnsignedT = u8,
...   |
264 | |         bound_condition = "",
265 | |     }
    | |_____- in this macro invocation
    = note: candidate #2 is defined in an impl for the type `i8`
    = note: this error originates in the macro `wrap_bitshift` which comes from the expansion of the macro `int_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/../../portable-simd/crates/core_simd/src/ops.rs:57:49
    |
    |
57  |                   $rhs.bitand(Simd::splat(<$int>::BITS as $int - 1)),
    |                                                   ^^^^ multiple `BITS` found
168 | / for_base_ops! {
168 | / for_base_ops! {
169 | |     T = (i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);
170 | |     type Lhs = Simd<T, N>;
171 | |     type Rhs = Simd<T, N>;
224 | |     }
225 | | }
    | |_- in this macro invocation
    |
    |
note: candidate #1 is defined in an impl for the type `i16`
   --> ../../../library/core/src/num/int_macros.rs:61:9
    |
61  |           pub const BITS: u32 = <$UnsignedT>::BITS;
    |
   ::: ../../../library/core/src/num/mod.rs:269:5
    |
269 | /     int_impl! {
269 | /     int_impl! {
270 | |         Self = i16,
271 | |         ActualT = i16,
272 | |         UnsignedT = u16,
...   |
287 | |         bound_condition = "",
288 | |     }
    | |_____- in this macro invocation
    = note: candidate #2 is defined in an impl for the type `i16`
    = note: this error originates in the macro `wrap_bitshift` which comes from the expansion of the macro `int_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/../../portable-simd/crates/core_simd/src/ops.rs:57:49
    |
    |
57  |                   $rhs.bitand(Simd::splat(<$int>::BITS as $int - 1)),
    |                                                   ^^^^ multiple `BITS` found
168 | / for_base_ops! {
168 | / for_base_ops! {
169 | |     T = (i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);
170 | |     type Lhs = Simd<T, N>;
171 | |     type Rhs = Simd<T, N>;
224 | |     }
225 | | }
    | |_- in this macro invocation
    |
    |
note: candidate #1 is defined in an impl for the type `i32`
   --> ../../../library/core/src/num/int_macros.rs:61:9
    |
61  |           pub const BITS: u32 = <$UnsignedT>::BITS;
    |
   ::: ../../../library/core/src/num/mod.rs:292:5
    |
292 | /     int_impl! {
292 | /     int_impl! {
293 | |         Self = i32,
294 | |         ActualT = i32,
295 | |         UnsignedT = u32,
...   |
310 | |         bound_condition = "",
311 | |     }
    | |_____- in this macro invocation
    = note: candidate #2 is defined in an impl for the type `i32`
    = note: this error originates in the macro `wrap_bitshift` which comes from the expansion of the macro `int_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/../../portable-simd/crates/core_simd/src/ops.rs:57:49
    |
    |
57  |                   $rhs.bitand(Simd::splat(<$int>::BITS as $int - 1)),
    |                                                   ^^^^ multiple `BITS` found
168 | / for_base_ops! {
168 | / for_base_ops! {
169 | |     T = (i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);
170 | |     type Lhs = Simd<T, N>;
171 | |     type Rhs = Simd<T, N>;
224 | |     }
225 | | }
    | |_- in this macro invocation
    |
    |
note: candidate #1 is defined in an impl for the type `i64`
   --> ../../../library/core/src/num/int_macros.rs:61:9
    |
61  |           pub const BITS: u32 = <$UnsignedT>::BITS;
    |
   ::: ../../../library/core/src/num/mod.rs:315:5
    |
315 | /     int_impl! {
315 | /     int_impl! {
316 | |         Self = i64,
317 | |         ActualT = i64,
318 | |         UnsignedT = u64,
...   |
333 | |         bound_condition = "",
334 | |     }
    | |_____- in this macro invocation
    = note: candidate #2 is defined in an impl for the type `i64`
    = note: this error originates in the macro `wrap_bitshift` which comes from the expansion of the macro `int_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/../../portable-simd/crates/core_simd/src/ops.rs:57:49
    |
    |
57  |                   $rhs.bitand(Simd::splat(<$int>::BITS as $int - 1)),
    |                                                   ^^^^ multiple `BITS` found
168 | / for_base_ops! {
168 | / for_base_ops! {
169 | |     T = (i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);
170 | |     type Lhs = Simd<T, N>;
171 | |     type Rhs = Simd<T, N>;
224 | |     }
225 | | }
    | |_- in this macro invocation
    |
    |
note: candidate #1 is defined in an impl for the type `isize`
   --> ../../../library/core/src/num/int_macros.rs:61:9
    |
61  |           pub const BITS: u32 = <$UnsignedT>::BITS;
    |
   ::: ../../../library/core/src/num/mod.rs:412:5
    |
412 | /     int_impl! {
412 | /     int_impl! {
413 | |         Self = isize,
414 | |         ActualT = i64,
415 | |         UnsignedT = usize,
...   |
430 | |         bound_condition = " on 64-bit targets",
    | |_____- in this macro invocation
    | |_____- in this macro invocation
    = note: candidate #2 is defined in an impl for the type `isize`
    = note: this error originates in the macro `wrap_bitshift` which comes from the expansion of the macro `int_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/../../portable-simd/crates/core_simd/src/ops.rs:57:49
    |
    |
57  |                   $rhs.bitand(Simd::splat(<$int>::BITS as $int - 1)),
    |                                                   ^^^^ multiple `BITS` found
168 | / for_base_ops! {
168 | / for_base_ops! {
169 | |     T = (i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);
170 | |     type Lhs = Simd<T, N>;
171 | |     type Rhs = Simd<T, N>;
224 | |     }
225 | | }
    | |_- in this macro invocation
    |
    |
note: candidate #1 is defined in an impl for the type `u8`
   --> ../../../library/core/src/num/uint_macros.rs:59:9
    |
59  |           pub const BITS: u32 = Self::MAX.count_ones();
    |
   ::: ../../../library/core/src/num/mod.rs:438:5
    |
438 | /     uint_impl! {
438 | /     uint_impl! {
439 | |         Self = u8,
440 | |         ActualT = u8,
441 | |         SignedT = i8,
...   |
455 | |         bound_condition = "",
456 | |     }
    | |_____- in this macro invocation
    = note: candidate #2 is defined in an impl for the type `u8`
    = note: this error originates in the macro `wrap_bitshift` which comes from the expansion of the macro `uint_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
    --> ../../../library/core/src/../../portable-simd/crates/core_simd/src/ops.rs:57:49
     |
     |
57   |                   $rhs.bitand(Simd::splat(<$int>::BITS as $int - 1)),
     |                                                   ^^^^ multiple `BITS` found
168  | / for_base_ops! {
168  | / for_base_ops! {
169  | |     T = (i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);
170  | |     type Lhs = Simd<T, N>;
171  | |     type Rhs = Simd<T, N>;
224  | |     }
225  | | }
     | |_- in this macro invocation
     |
     |
note: candidate #1 is defined in an impl for the type `u16`
    --> ../../../library/core/src/num/uint_macros.rs:59:9
     |
59   |           pub const BITS: u32 = Self::MAX.count_ones();
     |
    ::: ../../../library/core/src/num/mod.rs:1040:5
     |
1040 | /     uint_impl! {
1040 | /     uint_impl! {
1041 | |         Self = u16,
1042 | |         ActualT = u16,
1043 | |         SignedT = i16,
...    |
1057 | |         bound_condition = "",
1058 | |     }
     | |_____- in this macro invocation
     = note: candidate #2 is defined in an impl for the type `u16`
     = note: this error originates in the macro `wrap_bitshift` which comes from the expansion of the macro `uint_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
    --> ../../../library/core/src/../../portable-simd/crates/core_simd/src/ops.rs:57:49
     |
     |
57   |                   $rhs.bitand(Simd::splat(<$int>::BITS as $int - 1)),
     |                                                   ^^^^ multiple `BITS` found
168  | / for_base_ops! {
168  | / for_base_ops! {
169  | |     T = (i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);
170  | |     type Lhs = Simd<T, N>;
171  | |     type Rhs = Simd<T, N>;
224  | |     }
225  | | }
     | |_- in this macro invocation
     |
     |
note: candidate #1 is defined in an impl for the type `u32`
    --> ../../../library/core/src/num/uint_macros.rs:59:9
     |
59   |           pub const BITS: u32 = Self::MAX.count_ones();
     |
    ::: ../../../library/core/src/num/mod.rs:1088:5
     |
1088 | /     uint_impl! {
1088 | /     uint_impl! {
1089 | |         Self = u32,
1090 | |         ActualT = u32,
1091 | |         SignedT = i32,
...    |
1105 | |         bound_condition = "",
1106 | |     }
     | |_____- in this macro invocation
     = note: candidate #2 is defined in an impl for the type `u32`
     = note: this error originates in the macro `wrap_bitshift` which comes from the expansion of the macro `uint_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
    --> ../../../library/core/src/../../portable-simd/crates/core_simd/src/ops.rs:57:49
     |
     |
57   |                   $rhs.bitand(Simd::splat(<$int>::BITS as $int - 1)),
     |                                                   ^^^^ multiple `BITS` found
168  | / for_base_ops! {
168  | / for_base_ops! {
169  | |     T = (i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);
170  | |     type Lhs = Simd<T, N>;
171  | |     type Rhs = Simd<T, N>;
224  | |     }
225  | | }
     | |_- in this macro invocation
     |
     |
note: candidate #1 is defined in an impl for the type `u64`
    --> ../../../library/core/src/num/uint_macros.rs:59:9
     |
59   |           pub const BITS: u32 = Self::MAX.count_ones();
     |
    ::: ../../../library/core/src/num/mod.rs:1111:5
     |
1111 | /     uint_impl! {
1111 | /     uint_impl! {
1112 | |         Self = u64,
1113 | |         ActualT = u64,
1114 | |         SignedT = i64,
...    |
1128 | |         bound_condition = "",
1129 | |     }
     | |_____- in this macro invocation
     = note: candidate #2 is defined in an impl for the type `u64`
     = note: this error originates in the macro `wrap_bitshift` which comes from the expansion of the macro `uint_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
    --> ../../../library/core/src/../../portable-simd/crates/core_simd/src/ops.rs:57:49
     |
     |
57   |                   $rhs.bitand(Simd::splat(<$int>::BITS as $int - 1)),
     |                                                   ^^^^ multiple `BITS` found
168  | / for_base_ops! {
168  | / for_base_ops! {
169  | |     T = (i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);
170  | |     type Lhs = Simd<T, N>;
171  | |     type Rhs = Simd<T, N>;
224  | |     }
225  | | }
     | |_- in this macro invocation
     |
     |
note: candidate #1 is defined in an impl for the type `usize`
    --> ../../../library/core/src/num/uint_macros.rs:59:9
     |
59   |           pub const BITS: u32 = Self::MAX.count_ones();
     |
    ::: ../../../library/core/src/num/mod.rs:1207:5
     |
1207 | /     uint_impl! {
1207 | /     uint_impl! {
1208 | |         Self = usize,
1209 | |         ActualT = u64,
1210 | |         SignedT = isize,
...    |
1224 | |         bound_condition = " on 64-bit targets",
     | |_____- in this macro invocation
     | |_____- in this macro invocation
     = note: candidate #2 is defined in an impl for the type `usize`
     = note: this error originates in the macro `wrap_bitshift` which comes from the expansion of the macro `uint_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/../../portable-simd/crates/core_simd/src/ops.rs:57:49
    |
    |
57  |                   $rhs.bitand(Simd::splat(<$int>::BITS as $int - 1)),
    |                                                   ^^^^ multiple `BITS` found
168 | / for_base_ops! {
168 | / for_base_ops! {
169 | |     T = (i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);
170 | |     type Lhs = Simd<T, N>;
171 | |     type Rhs = Simd<T, N>;
224 | |     }
225 | | }
    | |_- in this macro invocation
    |
    |
note: candidate #1 is defined in an impl for the type `i8`
   --> ../../../library/core/src/num/int_macros.rs:61:9
    |
61  |           pub const BITS: u32 = <$UnsignedT>::BITS;
    |
   ::: ../../../library/core/src/num/mod.rs:246:5
    |
246 | /     int_impl! {
246 | /     int_impl! {
247 | |         Self = i8,
248 | |         ActualT = i8,
249 | |         UnsignedT = u8,
...   |
264 | |         bound_condition = "",
265 | |     }
    | |_____- in this macro invocation
    = note: candidate #2 is defined in an impl for the type `i8`
    = note: this error originates in the macro `wrap_bitshift` which comes from the expansion of the macro `int_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/../../portable-simd/crates/core_simd/src/ops.rs:57:49
    |
    |
57  |                   $rhs.bitand(Simd::splat(<$int>::BITS as $int - 1)),
    |                                                   ^^^^ multiple `BITS` found
168 | / for_base_ops! {
168 | / for_base_ops! {
169 | |     T = (i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);
170 | |     type Lhs = Simd<T, N>;
171 | |     type Rhs = Simd<T, N>;
224 | |     }
225 | | }
    | |_- in this macro invocation
    |
    |
note: candidate #1 is defined in an impl for the type `i16`
   --> ../../../library/core/src/num/int_macros.rs:61:9
    |
61  |           pub const BITS: u32 = <$UnsignedT>::BITS;
    |
   ::: ../../../library/core/src/num/mod.rs:269:5
    |
269 | /     int_impl! {
269 | /     int_impl! {
270 | |         Self = i16,
271 | |         ActualT = i16,
272 | |         UnsignedT = u16,
...   |
287 | |         bound_condition = "",
288 | |     }
    | |_____- in this macro invocation
    = note: candidate #2 is defined in an impl for the type `i16`
    = note: this error originates in the macro `wrap_bitshift` which comes from the expansion of the macro `int_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0034]: multiple applicable items in scope
   --> ../../../library/core/src/../../portable-simd/crates/core_simd/src/ops.rs:57:49
    |
    |
57  |                   $rhs.bitand(Simd::splat(<$int>::BITS as $int - 1)),
    |                                                   ^^^^ multiple `BITS` found
168 | / for_base_ops! {
168 | / for_base_ops! {
169 | |     T = (i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);
170 | |     type Lhs = Simd<T, N>;
171 | |     type Rhs = Simd<T, N>;
224 | |     }
225 | | }
    | |_- in this macro invocation
    |
    |
note: candidate #1 is defined in an impl for the type `i32`
   --> ../../../library/core/src/num/int_macros.rs:61:9
