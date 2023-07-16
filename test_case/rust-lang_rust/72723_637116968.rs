
ubuntu@juju-b11c42-ubuntu-23:~$ rustup default stable
info: using existing install for 'stable-s390x-unknown-linux-gnu'
info: default toolchain set to 'stable-s390x-unknown-linux-gnu'

  stable-s390x-unknown-linux-gnu unchanged - rustc 1.43.1 (8d69840ab 2020-05-04)

ubuntu@juju-b11c42-ubuntu-23:~$ cat hello.rs
fn main() {
    println!("Hello World!");
}
ubuntu@juju-b11c42-ubuntu-23:~$ rustc -C lto hello.rs 
Segmentation fault (core dumped)
