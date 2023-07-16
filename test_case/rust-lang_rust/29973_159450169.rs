
pub struct Pub;
type Priv = Pub; // Simple type alias without type parameters
pub fn f(arg: Priv) {} // OK, no error
