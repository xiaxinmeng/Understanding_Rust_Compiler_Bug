rust
fn foo2(x: Result<u32,!>) -> u32 { 
    match x {
        Ok(a) => a,
    }
}

fn foo3(x: Result<u32,!>) -> u32 { 
    match x {
        Ok(a) => a,
        Err(_) => panic!(),
    }
}
