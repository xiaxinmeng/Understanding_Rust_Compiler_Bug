rs
let mut guess = String::new();
loop {
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    // ^ `guess` as a variable of `u32` is not visible to this line...
    let guess:u32 = guess.trim().parse().expect("Please type a number!");
    // because it is only defined here in the loop...
}
