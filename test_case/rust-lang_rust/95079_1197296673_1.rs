
error: lifetime may not live long enough
 --> /home/ychen/hello_world/src/main_95079_2.rs:2:15
  |
2 |     move |()| s.chars().map(|c| format!("{}{}", c, s))
  |     --------- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ returning this value requires that `'1` must outlive `'2`
  |     |       |
  |     |       return type of closure `Map<Chars<'_>, [closure@/home/ychen/hello_world/src/main_95079_2.rs:2:29: 2:32]>` contains a lifetime `'2`
  |     lifetime `'1` represents this closure's body
  |
  = note: closure implements `Fn`, so references to captured variables can't escape the closure
help: consider adding 'move' keyword before the nested closure
  |
2 |     move |()| s.chars().map(move |c| format!("{}{}", c, s))
  |                             ++++

error: aborting due to previous error
