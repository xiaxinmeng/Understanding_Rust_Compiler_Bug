
<anon>:7:9: 7:41 error: `book1` does not live long enough
<anon>:7   spawn(|| println!("{}", &book1.title));
                 ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
note: reference must be valid for the static lifetime...
<anon>:4:91: 8:2 note: ...but borrowed value is only valid for the block suffix following statement 0 at 4:90
<anon>:4   let mut book1 = Book { title: "Hello".to_string(), chapters: vec!["world".to_string()] };
<anon>:5   let mut book2 = Book { title: "Hello".to_string(), chapters: vec!["world".to_string()] };
<anon>:6 
<anon>:7   spawn(|| println!("{}", &book1.title));
<anon>:8 }
