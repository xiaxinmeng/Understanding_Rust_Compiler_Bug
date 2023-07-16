
$ echo 'extern crate proc_macro;
fn main()
{
 println!("hi");
}' > test.rs
$ strace rustc test.rs
