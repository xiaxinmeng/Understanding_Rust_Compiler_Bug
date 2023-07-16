 rust
match Ok::<_, ()>(0) {
    Ok(0) => { 
        #![most_likely] 
        ::std::process::exit(0) 
    },
    Ok(_) => { 
        #![not_that_likely] 
        ::std::process::exit(1) 
    },
    Err(_) => { #![unlikely] ::std::process::exit(42) }
}
