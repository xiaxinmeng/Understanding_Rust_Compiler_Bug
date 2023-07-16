rust
#[derive(Default)]
enum Padding {
    Space,
    Zero,
    #[default]
    None,
}

assert_eq!(Padding::default(), Padding::None);
