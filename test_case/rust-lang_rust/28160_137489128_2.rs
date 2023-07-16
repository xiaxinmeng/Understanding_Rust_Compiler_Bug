
    %temp : &mut [u8] = call calculate, onerr exit
    x = %temp
    %boundck_1 : bool = i >= x.len
    bt %boundck_1, panic_boundck_1
    %lhs : &mut u8 = &mut x[i]
    %boundck_2: bool = j >= x.len
    bt %boundck_2, panic_boundck_2
    %rhs : u8 = x[j]
    *%lhs = %rhs
    ret
exit:
    unwind
