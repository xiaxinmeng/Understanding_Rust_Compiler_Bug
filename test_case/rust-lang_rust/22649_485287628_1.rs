rust
fn prompt_and_validate(msg: &str, options: &[&str]) {
  let reply = prompt(msg);
  let r = &reply;
  if options.contains(&r) {
    
  }
}
