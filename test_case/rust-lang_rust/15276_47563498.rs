 rust
let a = {println!("assigning"); 42u } // this is rarely useful

// but logically it's the same thing that happens here 
let a = if something {println!("assigning"); 42u } else {6u}

// and here
let a = match something {
   1..10 => {println!("assigning"); 42u }
   _ => 1
}

// and here
fn gimme_the_answer() -> uint {println!("assigning"); 42u }
