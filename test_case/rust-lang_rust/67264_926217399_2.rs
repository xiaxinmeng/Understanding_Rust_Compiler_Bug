rust
match power_level {
    ..=0 | ..=100 => println!("an incredibly low power level"),
    ..=1000 => println!("power someone might actually notice!"),
    ..=2000 => println!("millennial power"),
    ..=4000 => println!("badass"),
    ..=9000=> println!("super badass"),
    9001.. => panic!("it's over nine thousand?!")
}
