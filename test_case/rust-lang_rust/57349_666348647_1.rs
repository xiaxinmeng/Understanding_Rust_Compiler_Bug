rust
const FOO: &i32 = {
    let x = 2;
    &mut (1 + x)
};
