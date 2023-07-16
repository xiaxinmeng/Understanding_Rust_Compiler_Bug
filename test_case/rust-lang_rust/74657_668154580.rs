
$ echo 'fn main() { let a = [0u64; 1024]; println!("{:?}", &a[..]); }' > foo.rs
$ rustc foo.rs -Clinker-plugin-lto=LLVMgold.so -C link-arg=-fuse-ld=bfd
