rust
extern {
    type A;
    type B;
}

fn convert_ref(r: &A) -> &B { r }
