
DEBUG:rustc_trans::mir::rvalue: trans_rvalue(dest.llval=(%FooRef*:  %f = alloca %FooRef), rvalue=_1)
DEBUG:rustc_trans::mir::operand: trans_operand(operand=_1)
DEBUG:rustc_trans::mir::operand: trans_consume(lvalue=_1)
DEBUG:rustc_trans::mir::operand: trans_load: (%FooRef*:  %arg0 = alloca %FooRef) @ FooRef
DEBUG:rustc_trans::mir::operand: store_operand: operand=OperandRef(Immediate((%FooRef = type { { i8 }* }:  %4 = load %FooRef, %FooRef* %arg0, !dbg !41)) @ FooRef)
DEBUG:rustc_trans::mir::rvalue: trans_rvalue(dest.llval=(%str_slice*:  %s = alloca %str_slice), rvalue=&((*(_2.0: &(str,))).0: str))
DEBUG:rustc_trans::mir::operand: trans_consume(lvalue=(_2.0: &(str,)))
DEBUG:rustc_trans::mir::operand: trans_load: ({ i8 }**:  %5 = getelementptr inbounds %FooRef, %FooRef* %f, i32 0, i32 0, !dbg !42) @ &(str,)
DEBUG:rustc_trans::mir::operand: store_operand: operand=OperandRef(Pair((i8*:  %7 = getelementptr inbounds { i8 }, { i8 }* %6, i32 0, i32 0, !dbg !42),
rustc: /home/logic/build-tmp/rust-build/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/Casting.h:95: static bool llvm::isa_impl_cl<To, const From*>::doit(const From*) [with To = llvm::Value; From = llvm::Value]: Assertion `Val && "isa<> used on a null pointer"' failed.
Aborted
