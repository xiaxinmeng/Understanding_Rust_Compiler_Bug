
root@csz25020:~/rust# rm -rf ~/.cargo/registry
root@csz25020:~/rust# ./x.py build --verbose
    Updating registry `https://github.com/rust-lang/crates.io-index`
 Downloading filetime v0.1.10
error: unable to get packages from source

Caused by:
  failed to unpack package `filetime v0.1.10`

Caused by:
  failed to iterate over archive

Caused by:
  corrupt deflate stream
Build completed unsuccessfully in 0:00:40

