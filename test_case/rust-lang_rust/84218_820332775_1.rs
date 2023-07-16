rust
use std::ops::Shr;

fn works() { 1_u64 >> 2_u32; }

fn breaks<T>()          where u64: Shr<T>              { 1_u64 >> 2_u32; }
fn breaks_too<T>()      where u64: Shr<u32> + Shr<T>   { 1_u64 >> 2_u32; }
fn breaks_as_well<T>()  where u64: Shr<u32> + Shr<T>   { Shr::<_>::shr(1_u64, 2_u32); } // Should be infered to `Shr::<u32>` from my understanding, but doesn't seem to be the case
fn works_though<T>()    where u64: Shr<T>              { Shr::<u32>::shr(1_u64, 2_u32); }
fn works_too<T>()       where u64: Shr<u32> + Shr<T>   { Shr::<u32>::shr(1_u64, 2_u32); }

fn works_more_surprisingly<T, U>()
where 
    u64: Shr<T> + Shr<U> // An additional bound on another generic parameter seems to fix the resolution ??
{ 
    1_u64 >> 2_u32; 
}

fn works_more_surprisingly_in_full_form_too<T, U>()
where 
    u64: Shr<T> + Shr<U>
{ 
    Shr::<_>::shr(1_u64, 2_u32);
}
