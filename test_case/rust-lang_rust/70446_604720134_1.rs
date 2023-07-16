rust
use pin_project_lite::pin_project;

pin_project! {
    pub struct Struct<T: 'static> {
        field: T
    }
}


fn main() {

}
