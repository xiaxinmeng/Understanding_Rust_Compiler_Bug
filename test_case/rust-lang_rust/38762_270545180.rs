llvm
; C
define void @foo(%struct.Foo* noalias nocapture sret, %struct.Foo* byval align 16) local_unnamed_addr #0;

; rust
declare void @foo(%Foo* noalias nocapture sret dereferenceable(24), %Foo* byval noalias nocapture dereferenceable(24)) unnamed_addr #2;
