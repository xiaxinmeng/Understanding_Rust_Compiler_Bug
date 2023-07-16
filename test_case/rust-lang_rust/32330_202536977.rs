
> for<'a> fn(&'a u32, &'a u32) <: for<'b, 'c> fn(&'b u32, &'c u32)
> fn(&'α u32, &'α u32) <: fn(&'S u32, &'T u32)
> 'S: 'α, 'T: 'α
> 