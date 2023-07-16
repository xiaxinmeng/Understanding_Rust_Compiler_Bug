
> if !x.drop_flag { T::drop(&mut x); x.drop_flag = true; }
> if !x.drop_flag { T::drop(&mut x); x.drop_flag = true; }
> mem::forget(x);
> 