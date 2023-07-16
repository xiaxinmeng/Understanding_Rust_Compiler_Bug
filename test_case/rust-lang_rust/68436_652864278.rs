rust
const fn bar<const Z: usize>() where is_prime(Z) {}
const fn foo<const X: usize, const Y: usize>() {
    bar::<{X + Y}>();
}
