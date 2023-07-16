shell
> cat main.rs
fn main() {
    println!("{:?}", std::env::current_exe());
}
> ./main
Ok("/Users/ryan/Code/./main")
> ln -s main not-main
> ./not-main
Ok("/Users/ryan/Code/./not-main")
