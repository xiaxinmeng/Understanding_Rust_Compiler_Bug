
> use std;
> import std::arc;
> 
> fn main() {
>    let i = @5;
>    let arc_i = arc::arc(i);
> 
>    task::spawn { ||
>        let i = *arc::get(&arc_i);
>        #error("%?", i);
>    }
> }
> 