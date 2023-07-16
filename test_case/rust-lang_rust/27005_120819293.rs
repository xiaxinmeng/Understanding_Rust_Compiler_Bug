 rust
match 'a' {
  'a' => { }
  'c' => { }
  'a'...'e' => { }
  _ => { }
}
// Specialize for the first arm
if 'a' == 'a' { } else {
  match 'a' {
    'c' => { }
    'b'...'e' => { } // Modified range to account for != 'a'
    _ => { }
  }
}
// specialize for the next arm
if 'a' == 'a' { } 
else if 'a' == 'c' { }
else {
    match 'a' {
    'b'...'b' | 'd'...'e' => { } // split range to account for !='c'
    _ => { }
  }
}
// And so on...
