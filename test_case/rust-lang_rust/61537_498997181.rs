rust
struct Bool<const B: bool>;
impl Bool<{true }> { const B: bool = true; }
impl Bool<{false}> { const B: bool = false; }

fn should_work<const B: bool>() -> bool {
    Bool::<{B}>::B
}
