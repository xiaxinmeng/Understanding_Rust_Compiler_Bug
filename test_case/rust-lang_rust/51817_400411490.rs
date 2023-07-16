rust
> let s = [0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];
> // ...
> let r = s.binary_search(&1);
> assert!(match r { Ok(1...4) => true, _ => false, });
> 