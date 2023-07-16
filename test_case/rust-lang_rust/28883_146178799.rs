
let secret_number = rand::thread_rng().gen_range(1,101);
println!("The sercet number {}",secret_number);

println!("Please enter number!");
let mut guess = String::new();

io::stdin().read_line(&mut guess)
    .ok()
    .expect("Faile to read line!");

let guess: i33 = guess.trim().parse()
    .ok()
    .expect("Please type a number!");

println!("You enter number {}",guess);

match guess.cmp(&secret_number)
{
     Ordering::Less =>("Too small!"),
     Ordering::Greater =>("Too big!"),
     Ordering::Equal =>("You win!"),
}
