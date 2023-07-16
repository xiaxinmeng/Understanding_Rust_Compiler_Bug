rust
fn main() {
    let vec = get_vec();
    if let Err(err) = Ok::<_, ()>(1) {
        println!("Got an error: {:?}", err);
    }
    else if vec.unwrap().len() == 0 {
        println!("Vec was 0 length.");
    }
    else {
        for i in vec.unwrap() {
            println!("{}", i);
        }
    }
}

fn get_vec() -> Result<Vec<i32>, i32> {
    Ok::<Vec<i32>, i32>(vec![1,2,3])
}
