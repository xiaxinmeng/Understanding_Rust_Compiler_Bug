
error[E0597]: `frame` does not live long enough
  --> src/main.rs:47:5
   |
35 | fn parse<'a, P>() -> Result<()>
   |          -- lifetime `'a` defined here
...
47 |     frame.write(&mut wtr)?;
   |     ^^^^^----------------
   |     |
   |     borrowed value does not live long enough
   |     argument requires that `frame` is borrowed for `'a`
48 |     Ok(())
49 | }
   | - `frame` dropped here while still borrowed
