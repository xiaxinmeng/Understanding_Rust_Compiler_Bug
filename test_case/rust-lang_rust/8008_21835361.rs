
trait SelectInner { ... runtime internals ... }
pub trait Select : SelectInner { }
trait SelecPortInner<T> { ... runtime internals ... }
pub trait SelectPort<T> : SelectPortInner<T> { }
