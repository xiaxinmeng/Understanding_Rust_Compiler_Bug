rust
> let mut buf = vec![0; 10];
> buf.fill(1);  // compiler error!
> buf.fill(&1); // ok
> assert_eq!(buf, vec![1; 10]);
> 