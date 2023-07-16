
test.rs:5:23: 5:25 error: unexpected `::` after tuple index
test.rs:5     println!("{}", t.0::<Vec<uint>>)
                                ^~
test.rs:5:23: 5:25 note: tuple indices may not have type parameters
test.rs:5     println!("{}", t.0::<Vec<uint>>)
                                ^~
test.rs:5:34: 5:36 error: unexpected token: `<eof>`
test.rs:5     println!("{}", t.0::<Vec<uint>>)
                                           ^~
