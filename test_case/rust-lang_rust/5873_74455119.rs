 rust
enum Category {
    A,
    B,
    C,
    NumCategories,
}
static NUM_CATEGORIES: uint = Category::NumCategories as uint;

fn main() {
    let the_categories: [Category; NUM_CATEGORIES] = [Category::A, Category::B, Category::C];
    println!("{}", the_categories);
}
