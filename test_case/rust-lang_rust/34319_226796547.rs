
zmd@ExpectedReturn:~/Code/Misc$ rustc --version
rustc 1.11.0-nightly (bb4a79b08 2016-06-15)
zmd@ExpectedReturn:~/Code/Misc$ cat e0527.rs 
#![feature(slice_patterns)]

fn main() {
    let r = &[1, 2, 3, 4];
    match r {
        &[a, b] => { // error: pattern requires 2 elements but array
                     //        has 4
            println!("a={}, b={}", a, b);
        }
    }
}
zmd@ExpectedReturn:~/Code/Misc$ rustc e0527.rs 
e0527.rs:6:10: 6:16 error: pattern requires 2 elements but array has 4 [E0527]
e0527.rs:6         &[a, b] => { // error: pattern requires 2 elements but array
                    ^~~~~~
error: aborting due to previous error
zmd@ExpectedReturn:~/Code/Misc$ cat e0528.rs 
#![feature(slice_patterns)]

fn main() {
    let r = &[1, 2];
    match r {
        &[a, b, c, rest..] => { // error: pattern requires at least 3
                                //        elements but array has 2
            println!("a={}, b={}, c={} rest={:?}", a, b, c, rest);
        }
    }
}
zmd@ExpectedReturn:~/Code/Misc$ rustc e0528.rs 
e0528.rs:6:10: 6:27 error: pattern requires at least 3 elements but array has 2 [E0528]
e0528.rs:6         &[a, b, c, rest..] => { // error: pattern requires at least 3
                    ^~~~~~~~~~~~~~~~~
error: aborting due to previous error
zmd@ExpectedReturn:~/Code/Misc$ cat e0529.rs 
#![feature(slice_patterns)]

fn main() {
    let r: f32 = 1.0;
    match r {
        [a, b] => { // error: expected an array or slice, found `f32`
            println!("a={}, b={}", a, b);
        }
    }
}
zmd@ExpectedReturn:~/Code/Misc$ rustc e0529.rs 
e0529.rs:6:9: 6:15 error: expected an array or slice, found `f32` [E0529]
e0529.rs:6         [a, b] => { // error: expected an array or slice, found `f32`
                   ^~~~~~
error: aborting due to previous error
