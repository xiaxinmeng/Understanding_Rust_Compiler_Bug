
error: lifetime may not live long enough
  --> src/main.rs:13:7
   |
9  |   fn run<'run, 'event>(
   |          ----  ------ lifetime `'event` defined here
   |          |
   |          lifetime `'run` defined here
...
13 |       self.run1(args)
   |       ^^^^^^^^^^^^^^^ returning this value requires that `'run` must outlive `'event`
   |
   = help: consider adding the following bound: `'run: 'event`
