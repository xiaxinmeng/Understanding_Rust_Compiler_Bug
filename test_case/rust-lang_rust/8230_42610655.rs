 rust
enum E { L0 = -1, H0 = 1 }
enum F { L1 = 1, H1 = 0xFFFFFFFFFFFFFFFF }
static C0: f32 = L0 as f32;
static C1: f32 = H1 as f32;
pub fn main() {
        println!("{:?}", L0 as f32);
        println!("{:?}", C0);
        println!("{:?}", H1 as f32);
        println!("{:?}", C1);
}
