rust
fn prompt_and_validate(msg: &str, options: &[&str]) {
  let reply = prompt(msg);
  let r: &str = &reply;
  if options.contains(&r) {
    
  }
}
