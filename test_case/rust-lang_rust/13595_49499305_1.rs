 llvm
define internal void @_ZN3foo20h98be24cee3d646d9eaa4v0.0E() unnamed_addr #0 {
entry-block:
  ret void
}

define internal nonnull void ()** @_ZN3bar20hb2bd4683ab9952d6haa4v0.0E() unnamed_addr #0 {
entry-block:
  %addr_of = alloca void ()*
  store void ()* @_ZN3foo20h98be24cee3d646d9eaa4v0.0E, void ()** %addr_of
  ret void ()** %addr_of
}
