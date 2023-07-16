
error[E0597]: `line` does not live long enough
  --> <source>:42:5
   |
30 |         let v: Vec<&str> = line.split_whitespace().collect();
   |                            ---- borrow occurs here
...
42 |     }
   |     ^ `line` dropped here while still borrowed
43 | }
   | - borrowed value needs to live until here
error[E0597]: `line` does not live long enough
  --> <source>:62:5
   |
49 |         let v: Vec<&str> = line.split_whitespace().collect();
   |                            ---- borrow occurs here
...
62 |     }
   |     ^ `line` dropped here while still borrowed
63 | }
   | - borrowed value needs to live until here
error: aborting due to 2 previous errors
Compiler returned: 101
