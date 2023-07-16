rs
// use krate::Mutex;   // causes ICE
use krate::MMutex as Mutex;  // okay, normal error
struct A(Mutex<A>);
