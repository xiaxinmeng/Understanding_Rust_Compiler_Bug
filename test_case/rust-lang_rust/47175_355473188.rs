rust
    let pb_ident = "p.q";
    let mut searcher = '.'.into_searcher(pb_ident);
    println!("{:?} {:?}", searcher.next_match_back(), searcher.next_match());
