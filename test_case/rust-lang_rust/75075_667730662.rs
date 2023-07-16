rust
#[cfg(not(windows))]
macro_rules! main_separator{
    ()=>{"/"}
}

#[cfg(windows)]
macro_rules! main_separator{
    ()=>{r#"\"#}
}

fn main(){
    println!("{}", include_str!(concat!(".", main_separator!(), "main.rs" )));
}
