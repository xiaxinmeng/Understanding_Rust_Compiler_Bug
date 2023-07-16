rust
    let (dedup, _dup) = numbers.partition_dedup();
    println!("dedup {:?} dup {:?}",dedup, _dup );
    match _dup.len() {
         0 => println!("no pair"),
         1 => println!("one pair"),
         2 => println!("three of a kind or two pairs, do an other match"),
         3 => println!("four of a kind"),
         _ => println!("not handled"),
    }
    