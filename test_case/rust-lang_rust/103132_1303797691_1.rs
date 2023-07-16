
LVI Getting block end value   %arg.min = tail call i32 @llvm.umin.i32(i32 %arg, i32 16777216) at 'loop.latch'
PUSH:   %arg.min = tail call i32 @llvm.umin.i32(i32 %arg, i32 16777216) in loop.latch
PUSH:   %arg.min = tail call i32 @llvm.umin.i32(i32 %arg, i32 16777216) in loop
 compute BB 'loop' - overdefined because of pred (non local).
POP   %arg.min = tail call i32 @llvm.umin.i32(i32 %arg, i32 16777216) in loop = overdefined
 compute BB 'loop.latch' - overdefined because of pred (non local).
POP   %arg.min = tail call i32 @llvm.umin.i32(i32 %arg, i32 16777216) in loop.latch = overdefined
  Result = overdefined
