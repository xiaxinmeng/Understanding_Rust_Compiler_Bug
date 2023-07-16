
    let guess: Result<u32, ParseIntError> = guess.trim().parse();
    let guess: u32 = match guess {
        Ok(val) => val,
        Err(ParseIntError { kind: InvalidDigit}) => {
            println!("Por favor, escriba un número!");
            ::std::process::exit(1);
        }
        Err(err) => {
            println!("Se ha encontrado un error: {:?}", err);
            ::std::process::exit(1);
        }
    };
