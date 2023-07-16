
fn main(){
    let cmd = std::env::var_os("RUSTC");
    println!("{:?}", cmd);
}
