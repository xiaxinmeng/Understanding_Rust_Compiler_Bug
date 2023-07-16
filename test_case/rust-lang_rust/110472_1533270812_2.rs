
BISECT: running pass (1) Annotation2MetadataPass on [module]
BISECT: running pass (2) ForceFunctionAttrsPass on [module]
BISECT: running pass (3) InferFunctionAttrsPass on [module]
BISECT: running pass (4) LowerExpectIntrinsicPass on _ZN36_$LT$u8$u20$as$u20$simple..Clone$GT$5clone17h3411880f7ff41262E
BISECT: running pass (5) SimplifyCFGPass on _ZN36_$LT$u8$u20$as$u20$simple..Clone$GT$5clone17h3411880f7ff41262E
BISECT: running pass (6) SROAPass on _ZN36_$LT$u8$u20$as$u20$simple..Clone$GT$5clone17h3411880f7ff41262E
BISECT: running pass (7) EarlyCSEPass on _ZN36_$LT$u8$u20$as$u20$simple..Clone$GT$5clone17h3411880f7ff41262E
BISECT: running pass (8) LowerExpectIntrinsicPass on _ZN57_$LT$$u5b$u8$u3b$$u20$1$u5d$$u20$as$u20$simple..Clone$GT$5clone17hfbbe4e8d0c1471c4E
BISECT: running pass (9) SimplifyCFGPass on _ZN57_$LT$$u5b$u8$u3b$$u20$1$u5d$$u20$as$u20$simple..Clone$GT$5clone17hfbbe4e8d0c1471c4E
BISECT: running pass (10) SROAPass on _ZN57_$LT$$u5b$u8$u3b$$u20$1$u5d$$u20$as$u20$simple..Clone$GT$5clone17hfbbe4e8d0c1471c4E
BISECT: NOT running pass (11) EarlyCSEPass on _ZN57_$LT$$u5b$u8$u3b$$u20$1$u5d$$u20$as$u20$simple..Clone$GT$5clone17hfbbe4e8d0c1471c4E
[...]
BISECT: NOT running pass (233) X86 LEA Fixup on function (_ZN36_$LT$u8$u20$as$u20$simple..Clone$GT$5clone17h3411880f7ff41262E)
huge alignment values are unsupported
  %2 = load i8, ptr %0, align 4611686018427387904
in function _ZN57_$LT$$u5b$u8$u3b$$u20$1$u5d$$u20$as$u20$simple..Clone$GT$5clone17hfbbe4e8d0c1471c4E
LLVM ERROR: Broken function found, compilation aborted!
