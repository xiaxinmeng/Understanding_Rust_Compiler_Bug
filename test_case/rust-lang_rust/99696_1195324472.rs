plain
   Compiling rand v0.7.3
   Compiling alloc v0.0.0 (/checkout/library/alloc)
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling core v0.0.0 (/checkout/library/core)
error: for loop over an `Option`. This is more readably written as an `if let` statement
   |
   |
64 |     for _x in x {
   |
   |
   = note: `-D for-loop-over-fallibles` implied by `-D warnings`
help: to check pattern in a loop use `while let`
   |
64 |     while let Some(_x) = x {
   |     ~~~~~~~~~~~~~~~  ~~~
help: consider using `if let` to clear intent
   |
64 |     if let Some(_x) = x {
   |     ~~~~~~~~~~~~  ~~~
error: could not compile `core` due to previous error
warning: build failed, waiting for other jobs to finish...
 finished in 140.960 seconds
Build completed unsuccessfully in 0:18:22
