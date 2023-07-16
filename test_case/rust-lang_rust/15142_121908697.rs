 rust
<anon>:13:13: 13:15 error: lifetime name `'a` shadows a lifetime name that is already in scope
<anon>:13     fn next<'a>(&'a mut self) -> Option<&'a isize> {
                      ^~
<anon>:12:6: 12:8 note: shadowed lifetime `'a` declared here
<anon>:12 impl<'a, T> IntIterator for T where T: Iterator<&'a isize> {
               ^~
error: aborting due to previous error
playpen: application terminated with error code 101
