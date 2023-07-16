
struct A { pub i: int } // public field in private struct is useless
struct A { priv i: int } // priv should not be allowed on struct fields at all
pub enum A { pub Variant } // variant is already public
enum A { priv Variant } // variant is already private

pub trait A { pub fn foo(); } // function is already public
trait A { pub fn foo(); } // no effect
