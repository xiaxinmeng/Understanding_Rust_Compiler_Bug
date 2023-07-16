rust
> fn bench_read_exact_std_ok(b: &mut Bencher) {
>     b.iter(|| {
>         let mut buf = [0u8; READ_SIZE];
>         let mut r: &[u8] = &[0u8; READ_SIZE]; //always would be ok
> 
>         for _ in 0..LOOPS {
>             black_box(Read::read_exact(&mut r, &mut buf).is_ok());
>         }
>     });
> }
> 