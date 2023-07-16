rust
pub trait N {}

pub trait M {}

struct A;

impl M for String where A: N {} // Cannot resolve String: M 
impl M for String where A: N {}

fn main() {}
