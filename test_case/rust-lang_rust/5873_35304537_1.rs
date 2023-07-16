 rust
enum Category {
    A,
    B,
    C,
    NumCategories,
}
static NUM_CATEGORIES: uint = NumCategories as uint;

fn main() {
    let the_categories: [Category, ..NUM_CATEGORIES] = [A, B, C];
    println!("{}", the_categories);
}
