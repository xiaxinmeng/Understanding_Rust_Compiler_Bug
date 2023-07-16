rs
    println!("before regex");
    println!("regex `{}`", &format!(r"(?i).+\s->\s{}$", gate));
    let regex = Regex::new(&format!(r"(?i).+\s->\s{}$", gate));
    println!("regex {:?}", regex);
    let output_to_gate = regex.unwrap();
    println!("after regex");
  