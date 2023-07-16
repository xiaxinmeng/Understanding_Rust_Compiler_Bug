rust
> let mut buf = vec![0; 10];
> buf.fill(1);  // compiler error!
> buf.fill(&1); // ok
> 