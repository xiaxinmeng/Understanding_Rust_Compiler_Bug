rust
#[derive(Default)]
struct Struct {
    #[default = "MaybeUninit::uninit"]
    field: MaybeUninit<T>,
}
