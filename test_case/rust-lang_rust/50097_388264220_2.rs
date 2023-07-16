llvm-ir
cleanup.body.i.i.i.i.i:                           ; preds = %bb2.i.i.i.i
  %23 = landingpad { i8*, i32 }
          cleanup
  %24 = load {}*, {}** %8, align 8, !nonnull !6
  %25 = load {}*, {}** %10, align 8, !nonnull !6
; call alloc::alloc::box_free
  tail call fastcc void @_ZN5alloc5alloc8box_free17h95ac39da6ae7db02E({}* nonnull %24, {}* noalias nonnull readonly %25) #11
  %26 = load %"std::io::error::Custom"*, %"std::io::error::Custom"** %6, align 8, !nonnull !6
; call alloc::alloc::box_free
  tail call fastcc void @_ZN5alloc5alloc8box_free17h74d9b2058bbfc89bE(%"std::io::error::Custom"* nonnull %26) #11
  resume { i8*, i32 } %23
