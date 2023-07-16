
#[derive(Debug)]
enum MyEnum {
    AB,
}

fn main() {
    let value = MyEnum::AB;
    println!("[{: <3?}]", value); // The result is `[AB]`
                                  // while I expected that the result is `[AB ]`.
}
