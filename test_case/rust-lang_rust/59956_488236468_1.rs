rust
#[allow(dead_code)]
struct Bug<B = Self>(Option<B>);

impl Bug<u32> {}
