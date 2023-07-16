rust
const fn bar() {}

fn main() {
    #[allow(dead_code)]
    const _: () = bar();
}
