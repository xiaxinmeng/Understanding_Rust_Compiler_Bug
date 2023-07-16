rust
fn main() {
    let x = || 0;
    println!("{:?}", x());
    {
        println!("{:?}", x());
        fn x() -> i32 { 1 }
        println!("{:?}", x());
        let x = || 2;
        println!("{:?}", x());
    }
    println!("{:?}", x());
}
---
0
1
1
2
0
