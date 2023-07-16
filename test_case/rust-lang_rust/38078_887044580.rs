rust
trait T
{
    type M : Sized;
}

struct S
{
    some_var: u32
}

impl T for S
{
    type M = u32;
}

fn main()
{
    let v: S::M = 0;
}
