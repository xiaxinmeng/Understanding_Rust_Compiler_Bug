
$ cat src/lib.rs
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let _v = vec![1; 1024 * 1024 * 1024 * 1024 * 1024];
    }
}
$ cargo test
   Compiling bar v0.1.0 (file:///Users/sfackler/bar)
    Finished dev [unoptimized + debuginfo] target(s) in 0.62 secs
     Running target/debug/deps/bar-4ac15025f612bc06

running 1 test
fatal runtime error: allocator memory exhausted
