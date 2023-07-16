rust
let x;
let _v = do catch {
    x = 5;
    Ok(());
};
println!("{}", x); // OK
