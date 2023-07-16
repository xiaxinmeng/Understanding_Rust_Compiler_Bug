
>     vadd.f32        s5, s5, s12
>     vadd.f32        s1, s1, s1
>     vadd.f32        s3, s3, s3
>     vadd.f32        s5, s5, s5
>     vadd.f32        s1, s1, s12
>     vadd.f32        s3, s3, s12
>     vadd.f32        s5, s5, s12
>     vmul.f32        s7, s1, s1
>     vmul.f32        s9, s3, s3
>     vmul.f32        s11, s5, s5
>     vadd.f32        s7, s7, s9
>     vadd.f32        s7, s7, s11
> 