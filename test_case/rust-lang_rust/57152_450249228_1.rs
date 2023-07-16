
bb.0 (%ir-block.0):
  liveins: $arguments
  %0:i32 = ARGUMENT_i32 0, implicit $arguments
  %10:i32 = CONST_I32 2, implicit-def $arguments
  %5:i32 = COPY %0:i32
  %14:i32 = CONST_I32 2, implicit-def dead $arguments
  %7:i32 = AND_I32 %0:i32, killed %14:i32, implicit-def dead $arguments
  %8:i32 = CONST_I32 255, implicit-def $arguments
  %9:i32 = AND_I32 %7:i32, %8:i32, implicit-def $arguments
  %11:i32 = CONST_I32 255, implicit-def $arguments
  %12:i32 = AND_I32 %10:i32, %11:i32, implicit-def $arguments
  %13:i32 = EQ_I32 %9:i32, %12:i32, implicit-def $arguments
  %6:i32 = COPY %5:i32
  STORE8_I32 0, 0, %stack.0.t, %6:i32, implicit-def $arguments :: (store 1 into %ir.x8)
  %3:i32 = COPY %13:i32
  STORE8_I32 0, 1, %stack.0.t, %3:i32, implicit-def $arguments :: (store 1 into %ir.x10)
  RETURN_VOID implicit-def $arguments
