plain
   Compiling rustc_smir v0.0.0 (/checkout/compiler/rustc_smir)
   Compiling rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
   Compiling rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
   Compiling rustc_transmute v0.1.0 (/checkout/compiler/rustc_transmute)
error: using `.borrow()` on a double reference, which copies `&Mmap` instead of borrowing the inner type
   --> compiler/rustc_codegen_ssa/src/back/link.rs:576:43
    |
576 |             (*self.arena_mmap.alloc(data)).borrow()
    |
    |
    = note: `-D clone-double-ref` implied by `-D warnings`

error: using `.borrow()` on a double reference, which copies `&Relocations` instead of borrowing the inner type
   --> compiler/rustc_codegen_ssa/src/back/link.rs:586:50
    |
586 |             (*self.arena_relocations.alloc(data)).borrow()

error: could not compile `rustc_codegen_ssa` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:07:03
