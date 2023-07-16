 llvm
.loop.preheader: ; preds = %.noexc15
  br label %.loop

for_loopback.i: ; preds = %for_body.i14
  %16 = icmp eq i8* %18, %14
  br i1 %16, label %.noexc6.loopexit, label %.exit

.loop: ; preds = %.loop.preheader, %for_loopback.i
  %i = phi i64 [ %21, %for_loopback.i ], [ 0, %.loop.preheader ]
  %ix = phi i8* [ %18, %for_loopback.i ], [ %.pre, %.loop.preheader ]
  %17 = icmp eq i8* %ix, null ; WTF? spurious null check
  br i1 %17, label %.noexc6.loopexit, label %for_body.i14

for_body.i14: ; preds = .loop
  %18 = getelementptr inbounds i8* %ix, i64 1
  %19 = load i8* %ix, align 1
  %20 = icmp eq i8 %19, 1
  %21 = add i64 %i, 1
  br i1 %20, label %.noexc6.loopexit, label %for_loopback.i
