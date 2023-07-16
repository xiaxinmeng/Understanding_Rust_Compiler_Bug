 rust
let x = 5i;

    match x {
        1 => { println!("one");
               println!("{:d}", 1i); }
        2 => { println!("two");
               println!("{:d}", 2i); }
        3 => { println!("three");
               println!("{:d}", 3i); }
        4 => { println!("four");
               println!("{:d}", 4i); }
        5 => { println!("five");
               println!("{:d}", 5i); }
        _ => { println!("something else");
               println!("{:d}", 42i); }
    }
