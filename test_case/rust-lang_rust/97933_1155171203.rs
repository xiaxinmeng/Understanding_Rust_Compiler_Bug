rust
pub struct Container<const B: bool>;
const _: Container<{constants::Y}> = Container::<{!constants::X}>;
