
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Adivina el número!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("El número secreto es: {}", secret_number);

    // here's the problem start -- initialization thus first type set outside of the loop
    let mut guess = String::new();

    loop {

        println!("Por favor ingrese su conjetura.");

        io::stdin().read_line(&mut guess)
            .expect("Error al leer la línea...");

        println!("Usted lo adivinó: {}", guess);

//        let guess: u32 = guess.trim().parse()
//            .expect("Por favor, escriba un número!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("Número malo! {:?} : Inténtalo de nuevo...", err);
                continue
            },
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("¡Demasiado pequeña!"),
            Ordering::Greater => println!("¡Demasiado grande!"),
            Ordering::Equal => {
                println!("¡Tú ganas!");
                break;
            },
        }
    }
}
