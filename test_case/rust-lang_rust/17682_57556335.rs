 rust
extern crate rustc;
use rustc::driver;

fn main() {
    println!("{}", driver::release_str());
    println!("{}", driver::commit_hash_str());
    println!("{}", driver::commit_date_str());
    println!("{}", driver::driver::host_triple());
}
