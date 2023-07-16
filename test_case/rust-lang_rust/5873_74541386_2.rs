 rust
#[derive(Debug)]
enum Category {
    A,
    B,
    C,
    NumCategories,
}
const NUM_CATEGORIES: usize = Category::NumCategories as usize;

use Category::*;

fn main() {
    let the_categories: [Category; NUM_CATEGORIES] = [A, B, C];
    println!("{:?}", the_categories);
}
