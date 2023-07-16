
    Checking freightlines v0.1.0 (file:///Users/xlange/code/viasat/freightlines)
error[E0597]: `s` does not live long enough
  --> src/main.rs:20:32
   |
20 |         stream: StreamObj::new(&mut s)
   |                                ^^^^^^ borrowed value does not live long enough
...
24 | }
   | - `s` dropped here while still borrowed
   |
   = note: borrowed value must be valid for the static lifetime...
