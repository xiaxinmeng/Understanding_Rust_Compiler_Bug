llvm
; issue59793::empty
; Function Attrs: noreturn nonlazybind uwtable
define void @_ZN10issue597935empty17h4ff3ba3a79e34872E(%EmptyEnum* noalias nonnull readonly align 1) unnamed_addr #1 !dbg !13 {
start:
  %x = alloca %EmptyEnum*, align 8
  store %EmptyEnum* %0, %EmptyEnum** %x, align 8
  call void @llvm.dbg.declare(metadata %EmptyEnum** %x, metadata !22, metadata !DIExpression()), !dbg !23
  call void @llvm.trap(), !dbg !24
  unreachable, !dbg !24
}

; issue59793::unreach
; Function Attrs: noreturn nonlazybind uwtable
define void @_ZN10issue597937unreach17h2cd2e01f0a9a6888E(%EmptyEnum* noalias nonnull readonly align 1) unnamed_addr #1 !dbg !25 {
start:
  %x = alloca %EmptyEnum*, align 8
  store %EmptyEnum* %0, %EmptyEnum** %x, align 8
  call void @llvm.dbg.declare(metadata %EmptyEnum** %x, metadata !26, metadata !DIExpression()), !dbg !27
; call core::hint::unreachable_unchecked
  call void @_ZN4core4hint21unreachable_unchecked17h592d1c4b48415e36E(), !dbg !28
  unreachable, !dbg !28
}
