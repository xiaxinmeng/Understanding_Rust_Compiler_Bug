rust
    fn print_error(e: impl Error) { // or Box<Error> or whatever other actual type this is implemented for
      let mut chain = e.iter();
      eprintln!("error: {}", chain.next().unwrap());
      for source in chain {
        eprintln!("caused by {}", source);
      }
    