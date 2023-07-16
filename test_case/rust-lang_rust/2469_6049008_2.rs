
  let x1 = 4u8;

    alt x1 {
      1i8 to 2i8 { }
      3i8 to 4i8 { log(error, "hello") }
      _ { }
    }
