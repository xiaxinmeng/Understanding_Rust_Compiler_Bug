rust
const fn foo(x: usize) -> usize { /* stuff */ }

fn bar() {
    let n = /* stuff */;
    let x: [u8; dyn foo(n)] = /* stuff */;
    let y: [u8; dyn foo(n)] = x; // OK.
}
