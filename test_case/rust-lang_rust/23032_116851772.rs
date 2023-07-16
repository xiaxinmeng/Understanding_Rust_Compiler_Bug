 rust
static ALNUM: &'static u32 = &1;
static C:u32 = *(&/*implicit deref*/ALNUM as &'static u32);
fn main() { println!("{}", C); }
