
LV: Checking a loop in "_ZN8collect219create_with_collect17h1d615bb066a9cb69E" from collect2.cgu-0.rs
LV: Loop hints: force=? width=0 unroll=0
LV: Found a loop: bb35.i.i.i.i
LV: Found an induction variable.
LV: Found an induction variable.
LAA: Found a loop in _ZN8collect219create_with_collect17h1d615bb066a9cb69E: bb35.i.i.i.i
LAA: Found a write-only loop!
LV: We can vectorize this loop!
LV: Analyzing interleaved accesses...
LV: Found uniform instruction:   %exitcond.i.i.i = icmp eq i32 %iter.sroa.0.1.i.i.i.i, 65535
LV: Found uniform instruction:   %ptr.0148.i.i.i.i = phi i32* [ %12, %bb35.i.i.i.i ], [ %10, %bb13.i.i.i.i.i ]
LV: Found uniform instruction:   %12 = getelementptr inbounds i32, i32* %ptr.0148.i.i.i.i, i64 1
LV: The Smallest and Widest types: 32 / 32 bits.
LV: The Widest register is: 128 bits.
LV: Found an estimated cost of 0 for VF 1 For instruction:   %ptr.0148.i.i.i.i = phi i32* [ %12, %bb35.i.i.i.i ], [ %10, %bb13.i.i.i.i.i ]
LV: Found an estimated cost of 0 for VF 1 For instruction:   %iter.sroa.0.0147.i.i.i.i = phi i32 [ %iter.sroa.0.1.i.i.i.i, %bb35.i.i.i.i ], [ 0, %bb13.i.i.i.i.i ]
LV: Found an estimated cost of 1 for VF 1 For instruction:   %iter.sroa.0.1.i.i.i.i = add nuw nsw i32 %iter.sroa.0.0147.i.i.i.i, 1
LV: Found an estimated cost of 1 for VF 1 For instruction:   store i32 %iter.sroa.0.0147.i.i.i.i, i32* %ptr.0148.i.i.i.i, align 4, !noalias !0
LV: Found an estimated cost of 0 for VF 1 For instruction:   %12 = getelementptr inbounds i32, i32* %ptr.0148.i.i.i.i, i64 1
LV: Found an estimated cost of 1 for VF 1 For instruction:   %exitcond.i.i.i = icmp eq i32 %iter.sroa.0.1.i.i.i.i, 65535
LV: Found an estimated cost of 0 for VF 1 For instruction:   br i1 %exitcond.i.i.i, label %_ZN4core4iter8iterator8Iterator7collect17hc35826f1180bb746E.exit, label %bb35.i.i.i.i
LV: Scalar loop costs: 3.
LV: Found an estimated cost of 0 for VF 2 For instruction:   %ptr.0148.i.i.i.i = phi i32* [ %12, %bb35.i.i.i.i ], [ %10, %bb13.i.i.i.i.i ]
LV: Found an estimated cost of 0 for VF 2 For instruction:   %iter.sroa.0.0147.i.i.i.i = phi i32 [ %iter.sroa.0.1.i.i.i.i, %bb35.i.i.i.i ], [ 0, %bb13.i.i.i.i.i ]
LV: Found an estimated cost of 1 for VF 2 For instruction:   %iter.sroa.0.1.i.i.i.i = add nuw nsw i32 %iter.sroa.0.0147.i.i.i.i, 1
LV: Found an estimated cost of 1 for VF 2 For instruction:   store i32 %iter.sroa.0.0147.i.i.i.i, i32* %ptr.0148.i.i.i.i, align 4, !noalias !0
LV: Found an estimated cost of 0 for VF 2 For instruction:   %12 = getelementptr inbounds i32, i32* %ptr.0148.i.i.i.i, i64 1
LV: Found an estimated cost of 1 for VF 2 For instruction:   %exitcond.i.i.i = icmp eq i32 %iter.sroa.0.1.i.i.i.i, 65535
LV: Found an estimated cost of 0 for VF 2 For instruction:   br i1 %exitcond.i.i.i, label %_ZN4core4iter8iterator8Iterator7collect17hc35826f1180bb746E.exit, label %bb35.i.i.i.i
LV: Vector loop of width 2 costs: 1.
LV: Found an estimated cost of 0 for VF 4 For instruction:   %ptr.0148.i.i.i.i = phi i32* [ %12, %bb35.i.i.i.i ], [ %10, %bb13.i.i.i.i.i ]
LV: Found an estimated cost of 0 for VF 4 For instruction:   %iter.sroa.0.0147.i.i.i.i = phi i32 [ %iter.sroa.0.1.i.i.i.i, %bb35.i.i.i.i ], [ 0, %bb13.i.i.i.i.i ]
LV: Found an estimated cost of 1 for VF 4 For instruction:   %iter.sroa.0.1.i.i.i.i = add nuw nsw i32 %iter.sroa.0.0147.i.i.i.i, 1
LV: Found an estimated cost of 1 for VF 4 For instruction:   store i32 %iter.sroa.0.0147.i.i.i.i, i32* %ptr.0148.i.i.i.i, align 4, !noalias !0
LV: Found an estimated cost of 0 for VF 4 For instruction:   %12 = getelementptr inbounds i32, i32* %ptr.0148.i.i.i.i, i64 1
LV: Found an estimated cost of 1 for VF 4 For instruction:   %exitcond.i.i.i = icmp eq i32 %iter.sroa.0.1.i.i.i.i, 65535
LV: Found an estimated cost of 0 for VF 4 For instruction:   br i1 %exitcond.i.i.i, label %_ZN4core4iter8iterator8Iterator7collect17hc35826f1180bb746E.exit, label %bb35.i.i.i.i
LV: Vector loop of width 4 costs: 0.
LV: Selecting VF: 4.
LV: The target has 16 registers
