
let data = with_file(|f| { 
    let mut buffer = String::new();
    f.read_to_string(&mut buffer);
    buffer
}
