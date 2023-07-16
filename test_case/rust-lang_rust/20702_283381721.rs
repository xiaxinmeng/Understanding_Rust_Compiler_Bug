rust
let a = 10;
{
    let a = 11;
    println!("{}", a); // WHY DOES IT PRINT 11, I SAID `a = 10` PREVIOUSLY?
}
