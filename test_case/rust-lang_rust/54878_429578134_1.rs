
define void @test(i32* %addr1, i32* %addr2) {
start:
    br label %body

body:
    %i = phi i32 [ 0, %start ], [ %i2, %body ]
    %j = and i32 %i, 1
    %addr1i = getelementptr inbounds i32, i32* %addr1, i32 %j
    %addr2i = getelementptr inbounds i32, i32* %addr2, i32 %j

    %x = load i32, i32* %addr1i, !alias.scope !2
    store i32 %x, i32* %addr2i, !noalias !2

    %i2 = add i32 %i, 1
    %cmp = icmp slt i32 %i2, 4
    br i1 %cmp, label %body, label %end

end:
    ret void
}

!0 = !{!0}
!1 = !{!1, !0}
!2 = !{!1}
