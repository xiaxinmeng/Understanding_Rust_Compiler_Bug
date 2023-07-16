
; playground::use_it
; Function Attrs: noinline noreturn nonlazybind uwtable
define internal fastcc void @_ZN10playground6use_it17h9abb7f0a088ea9f2E() unnamed_addr #4 {
start:
; call std::process::abort
  tail call void @_ZN3std7process5abort17h887d510ab1f66ceaE() #10
  unreachable
}
