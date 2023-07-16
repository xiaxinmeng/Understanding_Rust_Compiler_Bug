
> [ a , b , c ]
>             ^--- 3..3 points here, right at the end of the slice, so it bound checks
> 
> [ a , b , c ]
>                 ^-- 4..4 points here, outside the slice, so it's an out of bounds
> 