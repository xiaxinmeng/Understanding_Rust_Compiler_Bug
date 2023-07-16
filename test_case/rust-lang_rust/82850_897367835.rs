rust
struct PanicOnDrop {
}

impl Drop for PanicOnDrop {
    fn drop(&mut self) {
        panic!("dropped");
    }
}

fn main() {
    let _panic_on_drop = PanicOnDrop { };
    panic!("first panic!");
}

#[cfg(test)]
mod tests {
    use main;
    #[test]
    fn foo() {
        main();
    }
}
