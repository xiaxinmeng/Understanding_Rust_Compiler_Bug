 llvm
; Function Attrs: uwtable
define void @"_ZN155_$LT$lib..Callback$u20$as$u20$core..ops..FnOnce$LT$$LP$$RF$$u27$a$u20$lib..Button$C$$u20$$RF$$u27$b$u20$mut$u20$lib..Trait$u20$$u2b$$u20$$u27$b$RP$$GT$$GT$9call_once17h8c75272c3c1fa0f7E"(%Button* noalias readonly, i8* nonnull, void (i8*)** nonnull) unnamed_addr #1 !dbg !24 {
entry-block:
  %arg1 = alloca { %Button*, { i8*, void (i8*)** } }
  %args = alloca { %Button*, { i8*, void (i8*)** } }
  call void @llvm.dbg.declare(metadata { %Button*, { i8*, void (i8*)** } }* %arg1, metadata !31, metadata !32), !dbg !30
  ; ...

  call void @llvm.dbg.declare(metadata { %Button*, { i8*, void (i8*)** } }* %arg1, metadata !33, metadata !34), !dbg !30
  call void @llvm.dbg.declare(metadata { %Button*, { i8*, void (i8*)** } }* %args, metadata !38, metadata !18), !dbg !37
  ; ...
}

!10 = !DICompositeType(tag: DW_TAG_structure_type, name: "(&lib::Button, &mut Trait)", file: !7, size: 192, align: 64, elements: !11, identifier: "{tuple {&{struct lib/45447b7afbd5e544f7d0f1df0fccd26014d9850130abd3f020b89ff96b82079f/5}}{&mut{trait lib/45447b7afbd5e544f7d0f1df0fccd26014d9850130abd3f020b89ff96b82079f/4}}}")
!13 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&lib::Button", baseType: !14, size: 64, align: 64)
!16 = !DICompositeType(tag: DW_TAG_structure_type, name: "&mut Trait", scope: !6, file: !7, size: 128, align: 64, elements: !2, identifier: "{&mut{trait lib/45447b7afbd5e544f7d0f1df0fccd26014d9850130abd3f020b89ff96b82079f/4}}")
!31 = !DILocalVariable(arg: 2, scope: !24, file: !5, line: 1, type: !13)
!32 = !DIExpression(DW_OP_deref, DW_OP_plus, 0)
!33 = !DILocalVariable(arg: 3, scope: !24, file: !5, line: 1, type: !16)
!34 = !DIExpression(DW_OP_deref, DW_OP_plus, 8)
!38 = !DILocalVariable(name: "args", scope: !36, file: !5, line: 19, type: !10)
