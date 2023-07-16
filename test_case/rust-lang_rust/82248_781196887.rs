
> let mut num_digits = 0;
> loop {
>     num_digits += 1;
>     n /= 10;
>     if n == 0 { break; }
> }
> num_digits
> 