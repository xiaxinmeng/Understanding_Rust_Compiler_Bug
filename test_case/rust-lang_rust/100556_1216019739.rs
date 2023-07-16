assembly
>         minss   xmm2, xmm0
>         cmpnless        xmm0, xmm1
>         andps   xmm2, xmm0
>         andnps  xmm0, xmm1
>         orps    xmm0, xmm2
>         ret
> 