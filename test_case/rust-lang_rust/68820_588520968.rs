
> #![feature(iter_map_while)]
> use std::convert::TryFrom;
>
> let a = [0, -1, 1, -2];
>
> let mut iter = a.iter().map_while(|x| u32::try_from(*x).ok());
>
> assert_eq!(iter.next(), Some(0u32));
>
> // We have more elements that are fit in u32, but since we already
> // got a None, map_while() isn't used any more
> assert_eq!(iter.next(), None);
> 