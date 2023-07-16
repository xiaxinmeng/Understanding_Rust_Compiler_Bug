
; not promoted
  %0 = bitcast i32** %_1 to {}**
  %1 = load {}*, {}** %0, align 8
; promoted
  %0 = load i32*, i32** %_1, align 8
  %1 = bitcast i32* %0 to {}*
