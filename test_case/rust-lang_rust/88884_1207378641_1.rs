rust
> fn evil_iter<'a>(arg: &'a Vec<&'a str>) -> impl Iterator<Item=&'a &'a str> {
>     arg.iter()
> }
> 
> fn first_str_len<'z: 'z>(arg: Vec<&'z str>) -> usize {
>     let first = evil_iter(&arg).next().unwrap();
>     let return_arg = |str: &'z str| str;
>     return_arg(first).len()
> }
> 