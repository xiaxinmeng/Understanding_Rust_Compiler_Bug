rust
pub mod definer {
    pub struct PubStruct;
}

mod user {
    use crate::definer::PubStruct;
}

mod fail {
    use crate::user::PubStruct;
    // error[E0603]: struct `PubStruct` is private
}
