
  let x1 = 4u8;

    alt x1 {
      1 to 2 { }
      3 to 4 { log(error, "hello") }
      _ { }
    }
