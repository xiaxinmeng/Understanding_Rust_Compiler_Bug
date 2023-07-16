rust
// In my actual code, this is a more complicated proc macro.
macro_rules! special_number {
  ($value: expr) => {
    {
      // In my actual code, these static variables also have
      // the special `export_name` of
      // "\x01L_special_number_<unique_id>", where
      // `<unique_id>` is a unique identifier to avoid
      // symbol conflicts.
      #[link_section = ".data,__custom_special_section"]
      static SPECIAL_NUMBER: usize = $value;
      &SPECIAL_NUMBER
    }
  };
}

extern {
  fn consume_special_number(value: &usize);
}

pub fn main() {
  unsafe {
    consume_special_number(special_number!(42));
    consume_special_number(special_number!(42));
    consume_special_number(special_number!(42));
  };
}
