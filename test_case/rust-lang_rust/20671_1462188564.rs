
pub trait A {
/// lots of associated types with trait bounds
}

pub trait B : A 
where
/// constrains A's associated types further
{
}
