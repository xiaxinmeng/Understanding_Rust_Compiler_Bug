 rust
#[derive(Debug)]
enum Category {
    A,
    B,
    C,
    NumCategories = 2, // this is too low!
}
const NUM_CATEGORIES: usize = Category::NumCategories as usize;

use Category::*;

fn main() {
    let the_categories: [Category; NUM_CATEGORIES] = [A, B];
    println!("{:?}", the_categories);
}
