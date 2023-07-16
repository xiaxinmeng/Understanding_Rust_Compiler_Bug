rs
#[test]
fn test1() -> Result<(), &'static str> {
    println!("test1 stdout");
    eprintln!("test1 stderr");
    Err("test1 return")
}

#[test]
fn test2() -> Result<(), &'static str> {
    println!("test2 stdout");
    eprintln!("test2 stderr");
    Err("test2 return")
}

#[test]
fn test3() -> Result<(), &'static str> {
    println!("test3 stdout");
    eprintln!("test3 stderr");
    Err("test3 return")
}
