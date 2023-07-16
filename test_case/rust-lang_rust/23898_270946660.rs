rust
use self::Category::*;

#[derive(Debug)]
enum Category {
    A = 0,
    B = 1,
    C = 2,
    NumCategories = 3,
}
const NUM_CATEGORIES: usize = NumCategories as usize;

fn main() {
    let the_categories: [Category; NUM_CATEGORIES] = [A, B, C];
    println!("{:?}", the_categories);
}
