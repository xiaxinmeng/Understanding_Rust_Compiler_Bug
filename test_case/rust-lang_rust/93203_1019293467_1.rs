diff
fn main ()
{
    ::crossbeam::thread::scope(|scope| {
        scope.spawn(|_| {
            // …
        });
        scope.spawn(|_| {
+           scope.spawn(|_| { /* … */ })
            // …
        });
    })
    .expect("Some thread panicked");
}
