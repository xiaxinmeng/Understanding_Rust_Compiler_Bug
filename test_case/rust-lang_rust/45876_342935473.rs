rust
mod item {
    mod visibility {
        pub struct Visibility;
    }
    pub use self::visibility::*;

    use Visibility;

    struct Item(Visibility);

    impl Item {
        fn get_nodes(&self) {
            self.0;
        }
    }
}
pub use self::item::*;

fn main() {}
