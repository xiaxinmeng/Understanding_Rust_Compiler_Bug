rust
> pub struct Stats { x: u32, y: u32, z: u32, }
> 
> pub extern "C" fn sum_c(a: &Stats, b: &Stats) -> Stats {
>     return Stats {x: a.x + b.x, y: a.y + b.y, z: a.z + b.z };
> }
> 