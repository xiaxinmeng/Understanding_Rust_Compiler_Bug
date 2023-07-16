rust
let a = match foo() {
    Either::A(a) => a,
    Either::B(wrapper) => match wrapper.get_ref() {},
};
