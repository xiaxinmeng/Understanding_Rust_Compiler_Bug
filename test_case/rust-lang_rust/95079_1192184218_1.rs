
error: lifetime may not live long enough
 --> main_95079_2.rs:2:15
  |
2 |     move |()| s.chars().map(|c| format!("{}{}", c, s))
  |     --------- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ returning this value requires that `'1` must outlive `'2`
  |     |       |
  |     |       return type of closure `Map<Chars<'_>, [closure@main_95079_2.rs:2:29: 2:54]>` contains a lifetime `'2`
  |     lifetime `'1` represents this closure's body
  |
  = note: closure implements `Fn`, so references to captured variables can't escape the closure
  = help: consider adding 'move' keyword before the parameter of closure in `Map<Chars<'_>, [closure@main_95079_2.rs:2:29: 2:54]>`

error: aborting due to previous error
