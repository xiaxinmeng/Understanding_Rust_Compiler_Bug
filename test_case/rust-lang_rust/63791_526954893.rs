
Replacing.2 t289: v32i8 = BUILD_VECTOR undef:i8, undef:i8, undef:i8, undef:i8, undef:i8, undef:i8, undef:i8, undef:i8, Constant:i8<2>, Constant:i8<3>, Constant:i8<6>, Constant:i8<7>, Constant:i8<10>, Constant:i8<11>, Constant:i8<14>, Constant:i8<15>, undef:i8, undef:i8, undef:i8, undef:i8, undef:i8, undef:i8, undef:i8, undef:i8, Constant:i8<18>, Constant:i8<19>, Constant:i8<22>, Constant:i8<23>, Constant:i8<26>, Constant:i8<27>, Constant:i8<30>, Constant:i8<31>

With: t292: v32i8 = BUILD_VECTOR undef:i8, undef:i8, undef:i8, undef:i8, undef:i8, undef:i8, undef:i8, undef:i8, undef:i8, undef:i8, undef:i8, undef:i8, undef:i8, undef:i8, undef:i8, undef:i8, undef:i8, undef:i8, undef:i8, undef:i8, undef:i8, undef:i8, undef:i8, undef:i8, undef:i8, undef:i8, undef:i8, undef:i8, Constant:i8<26>, Constant:i8<27>, undef:i8, undef:i8


Combining: t290: v32i8 = X86ISD::PSHUFB t288, t292
Creating new node: t293: v16i16 = bitcast t261
Creating constant: t294: i8 = Constant<-28>
Creating new node: t295: v16i16 = X86ISD::PSHUFLW t293, Constant:i8<-28>
Creating new node: t296: v32i8 = bitcast t295
 ... into: t296: v32i8 = bitcast t295
