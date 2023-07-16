 rust
mod scope_for_i32 {
    pub type Num = i32;
}
mod scope_for_u32 {
    pub type Num = u32;
}

fn main() {
    let _ :scope_for_i32::Num = 42;
    let _ :scope_for_u32::Num = 42;
}
