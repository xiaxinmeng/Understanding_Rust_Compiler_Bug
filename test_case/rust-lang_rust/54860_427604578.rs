
> let vec = vec![0; 5];
> assert_eq!(vec, [0, 0, 0, 0, 0]);
>
> // equivalent, but potentially slower:
> let mut vec1 = Vec::with_capacity(5);
> vec1.resize(5, 0);
> 