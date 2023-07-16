
zmd@ReflectiveCoherence:~/Code/Misc$ cat 51902.rs 
#![allow(unused)]
#![deny(elided_lifetimes_in_paths)]

struct Bar<'a> {
    a: &'a i32,
}

fn main() {
    let bar = Bar::<'_> {a: &42};
}
zmd@ReflectiveCoherence:~/Code/Misc$ rustc +nightly 51902.rs 
zmd@ReflectiveCoherence:~/Code/Misc$ echo $?
0
