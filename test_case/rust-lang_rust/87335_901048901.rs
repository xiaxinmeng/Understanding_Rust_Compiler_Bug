rust
> let parents: HashMap<u32, u32> = dfs();
> let mut curr: u32 = 0;
> let mut path: Vec<u32> = Vec::new();
> loop {
>     path.push(curr);
>     let Some(&curr) = parents.get(curr) else { break }
> }
> 