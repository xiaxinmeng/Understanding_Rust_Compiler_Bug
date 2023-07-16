rust
mod users {
    pub struct table;
    pub mod dsl {
        pub use super::table as users;
    }
}

fn main() {
    use users::dsl::*;
}
