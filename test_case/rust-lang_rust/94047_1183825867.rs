rust
> fn main() {
>     let a = vec![Some(3), Some(4)];
>     let b = vec![Some(3), None];
>     let c = vec![Result::<_, ()>::Ok(3), Ok(4)];
>     let d = vec![Ok(3), Err(())];
> 
>     println!("{:?}", a.into_iter().collect::<Option<Vec<_>>>());
>     println!("{:?}", b.into_iter().collect::<Option<Vec<_>>>());
>     println!("{:?}", c.into_iter().collect::<Result<Vec<_>, _>>());
>     println!("{:?}", d.into_iter().collect::<Result<Vec<_>, _>>());
> }
> 