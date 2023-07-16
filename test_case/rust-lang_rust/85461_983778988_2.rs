rust
// src/lib.rs
mod cow {
  #[derive(Debug)]
  struct CowBytes<'a>(&'a ());

  #[derive(Debug)]
  enum Imp<'a> {
      Owned(&'a [u8]),
  }
}

pub fn memchr() {
  static FN: std::sync::Once = std::sync::Once::new();

  let _f = &FN.is_completed();
}

mod memmem;
