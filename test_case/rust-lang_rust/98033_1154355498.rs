rust
let is_terminal = false;
#[cfg(windows)]
let is_terminal = stdin.as_view::<File>().is_terminal()?;
#[cfg(unix)]
let is_terminal = stdin.as_view::<File>().is_terminal()?;
