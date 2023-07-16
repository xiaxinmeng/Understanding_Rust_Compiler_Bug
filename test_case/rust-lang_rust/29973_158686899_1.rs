
struct Priv;
type Alias<T = Priv> = T;
pub fn f(arg: Alias) {}
