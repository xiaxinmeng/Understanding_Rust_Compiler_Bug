rust
let x;
let _v = do catch {
    x = Ok(5)?;
    Ok(());
};
println!("{}", x); // ERROR
