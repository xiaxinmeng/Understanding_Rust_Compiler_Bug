
$ cat main.rs
fn main() {
    println!("Hello, world!");
}
$ strace -o old-ce45663e14dac3f0f58be698cc530bc2e6e21682.full.strace -f -T ~/.rustup/toolchains/ce45663e14dac3f0f58be698cc530bc2e6e21682/bin/rustc main.rs
$ strace -o new-4b920a40932f74a7159435b06d96cb50212514ff.full.strace -f -T ~/.rustup/toolchains/4b920a40932f74a7159435b06d96cb50212514ff/bin/rustc main.rs
