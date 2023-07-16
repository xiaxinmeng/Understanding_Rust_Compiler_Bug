
#[path = "mod_dir_simple"]
mod pancakes {
    #[path = "test.rs"]
    pub mod syrup;
}

#[path = "mod_dir_simple/test.rs"]
mod gravy;

pub fn main() {
    assert_eq!(pancakes::syrup::foo(), 10);
    assert_eq!(gravy::foo(), 10);
}
