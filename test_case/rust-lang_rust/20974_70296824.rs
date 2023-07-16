 rust
// crate 1
pub enum Status { .. };
// crate 2
trait Modifier<M>;
impl<T> Modifier<T> for Status { .. } // ***
// crate 3
struct Response { };
impl Modifier<Response> for Status { .. }
