
mod outer {
    mod inner {
        pub struct Private;
        pub fn interface() -> Private {
            Private
        }
    }
    // This line should cause an error/warning
    pub use self::inner::{interface};
}

use outer::{interface};

fn main() {
    // Does not compile because this type is not in scope here
    // But, remove the explicit type declaration and there is no error.
    // Type inference has access to types that are out of scope.
    let _ : outer::inner::Private = interface();
}
