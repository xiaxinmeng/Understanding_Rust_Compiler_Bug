rust
const CONST: [(); 1] = [()];
match &[()] {
    &[()] => {}
    &CONST => {}
}
