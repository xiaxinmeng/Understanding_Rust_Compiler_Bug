
let x;
#[cfg(windows)]
{ x = 5; }
#[cfg(unix)]
{ x = 3; }
