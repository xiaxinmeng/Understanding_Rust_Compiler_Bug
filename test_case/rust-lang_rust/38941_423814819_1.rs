llvm
> ; playground::f1
> ; Function Attrs: nounwind uwtable
> define void @_ZN10playground2f117h829c91c07d14df5dE(i32* %a, i32* %b, i32* readonly %x) unnamed_addr #1 {
> start:
>   %0 = icmp ne i32* %a, null
>   tail call void @llvm.assume(i1 %0)
>   %1 = icmp ne i32* %b, null
>   tail call void @llvm.assume(i1 %1)
>   %2 = icmp ne i32* %x, null
>   tail call void @llvm.assume(i1 %2)
>   %x.val = load i32, i32* %x, align 4
>   store i32 %x.val, i32* %a, align 4, !alias.scope !0, !noalias !3
>   store i32 %x.val, i32* %b, align 4, !alias.scope !3, !noalias !0
>   ret void
> }
> 