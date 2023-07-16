rust
let mut string = arrayvec::ArrayString::<200>::new();
writeln!(&mut string, "{} {:x} {:o}", 123, 123, 123).unwrap();
