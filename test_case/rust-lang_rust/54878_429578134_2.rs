
define void @test(i32* %addr1, i32* %addr2) {
start:
  br label %body

body:                                             ; preds = %start
  %x = load i32, i32* %addr1, !alias.scope !0
  store i32 %x, i32* %addr2, !noalias !0
  %addr1i.1 = getelementptr inbounds i32, i32* %addr1, i32 1
  %addr2i.1 = getelementptr inbounds i32, i32* %addr2, i32 1
  %x.1 = load i32, i32* %addr1i.1, !alias.scope !0
  store i32 %x.1, i32* %addr2i.1, !noalias !0
  %x.2 = load i32, i32* %addr1, !alias.scope !0
  store i32 %x.2, i32* %addr2, !noalias !0
  %addr1i.3 = getelementptr inbounds i32, i32* %addr1, i32 1
  %addr2i.3 = getelementptr inbounds i32, i32* %addr2, i32 1
  %x.3 = load i32, i32* %addr1i.3, !alias.scope !0
  store i32 %x.3, i32* %addr2i.3, !noalias !0
  ret void
}

!0 = !{!1}
!1 = distinct !{!1, !2}
!2 = distinct !{!2}
