rust
struct NotDrop(String) // has drop glue, but no Drop impl

trait MyTrait {}
impl<T: Drop> MyTrait for T {}
impl MyTrait for NotDrop {}
