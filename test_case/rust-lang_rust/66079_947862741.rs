rust
extern crate spell_chk;

#[spell_chk::skip_function] // this is valid
fn func_noname () {         // spell checking skip this function
  // ....
}

#[spell_chk::acronym] // this is invalid and should error. The correct name is has_acronym
fn fyi_function() {
  // ...
}
