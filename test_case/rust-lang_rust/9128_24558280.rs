
<@brson> olsonjeffery: i would have like to make all the tests in sched.rs not rely on libuv
<@brson> olsonjeffery: for a BasicEventLoop i would expose the 'signal' part of the C++ 'lock_and_signal' type,
           which is just a mutex + condition variable, and use that for simulating the libuv async handles
