rust
struct A {
    data: &'static u32,
}

const aa_storage: [A; 32] = [A { data: &1 }; 32];
