rs
// first example
loop {
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    let guess:u32 = guess.trim().parse().expect("Please type a number!");
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {println!("You win!"); break; }
    }
}
