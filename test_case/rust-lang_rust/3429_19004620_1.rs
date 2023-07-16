
con.rs:3:4: 3:15 error: multiple applicable methods in scope
con.rs:3     x.foobar();
             ^~~~~~~~~~~
con.rs:23:4: 23:23 note: candidate #1 is `__extensions__::foobar`
con.rs:23     fn foobar(&self) {}
              ^~~~~~~~~~~~~~~~~~~
con.rs:19:4: 19:23 note: candidate #2 is `__extensions__::foobar`
con.rs:19     fn foobar(&self) {}
              ^~~~~~~~~~~~~~~~~~~
con.rs:3:4: 3:15 error: failed to find an implementation of trait BarBase for X
con.rs:3     x.foobar();
             ^~~~~~~~~~~
