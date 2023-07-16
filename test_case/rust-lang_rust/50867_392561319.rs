
$ cat issue50867/src/main.rs 
fn main() {
    println!("Hello, world!");
}

#[test]
fn test_transform_compress_none() {
}
$ rustc +nightly-2018-04-30 -Vv|grep hash
commit-hash: 79252ff4e25d82f9fe856cb66f127b79cdace163
$ rustc +nightly-2018-05-03 -Vv|grep hash         
commit-hash: 8a37c75a3a661385cc607d934c70e86a9eaf5fd7
